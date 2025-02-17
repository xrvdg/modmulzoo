#![feature(bigint_helper_methods)]

use num_traits::MulAdd;
use quickcheck::Arbitrary;

use crate::{convert_limb_sizes, MASK48, MASK52};

#[inline(always)]
pub fn adds_u52(t: &mut [u64], mut carry: u64) {
    for i in 0..t.len() {
        // Performance drops heavily when introducing this check
        // if carry == 0 {
        //     break;
        // }
        let tmp = t[i] + carry;
        (t[i], carry) = (tmp & MASK52, tmp >> 52);
    }
}

pub fn sos(a: U256b52, b: U256b52, n: U256b52, n0: u64) -> [u64; 10] {
    let a = a.0;
    let b = b.0;
    let n = n.0;
    let mut t = [0_u64; 10];

    // TODO: try to write an iterator version of this. I don't think you can't as you refer back to an element you just created

    // multiplication a * b
    for i in 0..a.len() {
        let mut carry = 0;
        for j in 0..b.len() {
            (t[i + j], carry) = carrying_mul_add_u104(a[i], b[j], t[i + j], carry)
        }
        t[i + b.len()] = carry;
    }

    for i in 0..(n.len()) {
        let mut carry = 0;
        let m = t[i].wrapping_mul(n0) & MASK52;
        for j in 0..n.len() {
            (t[i + j], carry) = carrying_mul_add_u104(m, n[j], t[i + j], carry)
        }
        adds_u52(&mut t[(i + n.len())..], carry)
    }

    t
}

// TODO: how to deal with all the converions
// Int to float is an expensive operation, or not if you do a casting?
// TODO how to ensure that the right multiplication algorithm is used.
// - First compare what the difference is
fn full_product(a: f64, b: f64) -> (f64, f64) {
    let p_hi = a.mul_add(b, 0.);
    let p_lo: f64 = a.mul_add(b, -p_hi);
    (p_lo, p_hi)
}

const fn pow_2(n: u32) -> f64 {
    // Unfortunately we can't use f64::powi in const fn yet
    // This is a workaround that creates the bit pattern directly
    let exp = ((n as u64 + 1023) & 0x7FF) << 52;
    f64::from_bits(exp)
}

// Define your constants using the const fn
const C1: f64 = pow_2(104); // 2.0^104
const C2: f64 = pow_2(104) + pow_2(52); // 2.0^104 + 2.0^52
const C3: f64 = pow_2(52); // 2.0^52

// As in the paper, but does not work as is
// The subtractions causes the value to be normalized again meaning
fn dpf_full_product(a: f64, b: f64) -> (f64, f64) {
    let p_hi = a.mul_add(b, C1);
    let p_lo = a.mul_add(b, C2 - p_hi);

    // It will normalize the values again, which is what we don't want at this step
    (p_lo - C3, p_hi - C1)
}

// Working variant
fn dpf_full_product_u64(a: f64, b: f64) -> (u64, u64) {
    let p_hi = a.mul_add(b, C1);
    let p_lo = a.mul_add(b, C2 - p_hi);

    // This part is omitted in the paper, but essential
    // If you do subtraction in floating point domain the mantissa will move to the exponent
    (
        (p_lo.to_bits() - C3.to_bits()),
        (p_hi.to_bits() - C1.to_bits()),
    )
    // Might be beneficial to convert them to floats again as they are still in the register
    // might also not be that useful as it can create a bottleneck on v0
}

// Check which full product variant gives better assembly
// This should also stay in vector register and can
fn int_full_product(a: f64, b: f64) -> (u64, u64) {
    let p_hi = a.mul_add(b, C1);
    let p_lo = a.mul_add(b, C2 - p_hi);

    // vectorizable
    (p_lo.to_bits() & MASK52, p_hi.to_bits() & MASK52)
}

// These probably have to stay within the 11 bits
// Not totally convinced about the correctness of this one
fn make_initial(low_count: usize, high_count: usize) -> u64 {
    let val = high_count * 0x467 + low_count * 0x433;

    // Double if this is correct
    -((val as i64 & 0xFFF) << 52) as u64
}

const N: usize = 5;
// Runs faster than the masked variant
pub fn sampled_product(a: [f64; N], b: [f64; N]) -> [u64; 2 * N] {
    // TODO make these const across the code base
    // Does require doing a compile time computation

    let mut col_sums: [u64; 10] = [0; 2 * N];

    // Since our N is fixed this can be made a compile time constant
    // For a known size this is probably mixed in
    for i in 0..N {
        col_sums[i] = make_initial(i + 1, i);
        col_sums[2 * N - 1 - i] = make_initial(i, i + 1);
    }

    for i in 0..a.len() {
        for j in 0..b.len() {
            // These two multiplications can be shared between implementations
            // Shows what is common and what is different
            // This cannot be a vector operation
            let p_hi = a[i].mul_add(b[j], C1);
            let p_lo = a[i].mul_add(b[j], C2 - p_hi);
            // Looks like this could be vectorized
            col_sums[i + j + 1] = col_sums[i + j + 1].wrapping_add(p_hi.to_bits());
            col_sums[i + j] = col_sums[i + j].wrapping_add(p_lo.to_bits());
        }
    }
    let mut carry = 0;
    for i in 0..col_sums.len() {
        let tmp = col_sums[i] + carry;
        col_sums[i] = tmp & MASK52;
        carry = tmp >> 52;
    }
    // Kind of need to return a pair of high and lo
    // or shift the whole thing
    // Also didn't typecheck due to not using 5 which is needed for the U256 method
    col_sums
}

// Masking works but likely make use of the M-pipeline which we want to
// keep free for another implementation
pub fn sampled_product_masked(a: [f64; N], b: [f64; N]) -> [u64; 2 * N] {
    let mut col_sums: [u64; 10] = [0; 2 * N];

    for i in 0..a.len() {
        for j in 0..b.len() {
            let p_hi = a[i].mul_add(b[j], C1);
            let p_lo = a[i].mul_add(b[j], C2 - p_hi);
            // Looks like this could be vectorized
            col_sums[i + j + 1] += p_hi.to_bits() & MASK52;
            col_sums[i + j] += p_lo.to_bits() & MASK52;
        }
    }

    // This make non-redundant b52. Is that best keep it here or outside?
    let mut carry = 0;

    // This loop is relatively cheap. Does it get fused into the upper one?
    // Should we write it as such?
    for i in 0..col_sums.len() {
        let tmp = col_sums[i] + carry;
        col_sums[i] = tmp & MASK52;
        carry = tmp >> 52;
    }
    col_sums
}

#[inline(always)]
pub fn carrying_mul_add_u104(a: u64, b: u64, add: u64, carry: u64) -> (u64, u64) {
    let c: u128 = a as u128 * b as u128 + carry as u128 + add as u128;
    (c as u64 & MASK52, (c >> 52) as u64)
}

#[inline(always)]
pub fn carrying_mul_add(a: u64, b: u64, add: u64, carry: u64) -> (u64, u64) {
    // TODO intrinsic
    // Check assembly output for this kind of widening
    // unchecked version might be better, shouldn't be possible to overflow due to widening beforehand.
    // Is there a difference between unchecked
    // using widening_mul might be friendlier to use
    let c: u128 = a as u128 * b as u128 + carry as u128 + add as u128;
    (c as u64, (c >> 64) as u64)
}

pub fn school_method(a: U256b64, b: U256b64) -> [u64; 8] {
    let mut ab = [0_u64; 8];
    let U256b64(a) = a;
    let U256b64(b) = b;
    for i in 0..a.len() {
        let mut carry = 0;
        for j in 0..b.len() {
            (ab[i + j], carry) = carrying_mul_add(a[i], b[j], ab[i + j], carry)
        }
        ab[i + b.len()] = carry;
    }
    ab
}

// Cost of getting it into the right from
// Seeing that there is exponentiation could it be profitable to keep in a certain range?
// Proper types
// TODO write a proper test suite
// Does rust have something like quickcheck?

// Do multiplication on the float representation and to convert it to 52-bit integer
// you add 2^52 and mask it.

// Looks like the value first needs to be converted to float

fn convert_limb_debug() {
    let s = vec![0];
    let c = convert_limb_sizes(&s, 256, 64, 52);
    println!("s: {s:?}");
    println!("c: {c:?}");
    let d = convert_limb_sizes(&c, 256, 52, 64);
    println!("c: {d:?}");
    let b = s == d;
    print!("{b}")
}
fn sampled_product_masked_debug() {
    set_round_to_zero();

    let (a, b) = (U256b64([0, 0, 0, 1]), U256b64([0, 0, 1, 0]));
    print!("a:\t");
    print_bits(&a.0);
    print!("b:\t");
    print_bits(&b.0);
    let s = school_method(a, b);
    print!("s:\t");
    print_bits(&s);
    let (a52, b52): (U256b52, U256b52) = (a.into(), b.into());
    print!("a52:\t");
    print_bits(&a52.0);
    print!("b52:\t");
    print_bits(&b52.0);
    let spm = sampled_product_masked(a52.0.map(|ai| ai as f64), b52.0.map(|bi| bi as f64));
    print_bits(&spm);
}

fn dpf_debug() {
    set_round_to_zero();
    let (a, b): (u64, u64) = (1, 1);
    println!("a: {:064b}, b: {:064b}", a, b);
    let (ilo, ihi) = dpf_full_product(a as f64, b as f64);
    println!("ia: {:064b}, ib: {:064b}", ilo as u64, ihi as u64);
    let (wlo, whi) = a.widening_mul(b);
    println!("wa: {:064b}, wb: {:064b}", wlo, whi);

    println!("");
    let (a, b): (u64, u64) = (2, 2251799813685248);
    println!("a: {:064b}, b: {:064b}", a, b);
    let (dlo, dhi) = dpf_full_product(a as f64, b as f64);
    println!(
        "dlo: {:064b}/{}/{}, dhi: {:064b}/{}/{}",
        dlo as u64, dlo as u64, dlo, dhi as u64, dhi as u64, dhi
    );
    let (ulo, uhi) = dpf_full_product_u64(a as f64, b as f64);
    println!("ulo: {:064b}, uhi: {:064b}", ulo as u64, uhi as u64);
    let (wlo, whi) = a.widening_mul(b);
    println!("wa: {:064b}, wb: {:064b}", wlo, whi);
}

fn int_mul_debug() {
    set_round_to_zero();
    println!("{:064b}", 2_u64.pow(52));
    println!("{:064b}", 2251799813685249_u64);
    println!("{}", 2_u64.pow(52) - 2251799813685249_u64);

    let (a, b): (u64, u64) = (1, 2251799813685249_u64);
    println!("a: {:064b}, b: {:064b}", a, b);
    let (ilo, ihi) = int_full_product(a as f64, b as f64);
    println!("ia: {:064b}, ib: {:064b}", ilo, ihi);
    let (wlo, whi) = a.widening_mul(b);
    println!("wa: {:064b}, wb: {:064b}", wlo, whi);
    print!("\n");
    let (a, b): (u64, u64) = (2, 2251799813685248);
    println!("a: {:064b}, b: {:064b}", a, b);
    let (ia, ib) = int_full_product(a as f64, b as f64);
    println!("ia: {:064b}, ib: {:064b}", ia, ib);
    let (wa, wb) = a.widening_mul(b);
    println!("wa: {:064b}, wb: {:064b}", wa, wb);
}

fn earlier_experiment() {
    let a = 2.0_f64.powi(52) + 4.;
    print_float(a);
    let b = 2.0_f64.powi(52) + 5.;
    print_float(b);
    print_float(a * b);

    print_float(4.0.mul_add(5.0, 2.0_f64.powi(52)));

    println!("int product: {:?}", full_product(4., 5.));

    let a = f64::from_bits(5 + 2_u64.pow(62));
    let b = f64::from_bits(4 + 2_u64.pow(62));
    print_float(a);
    print_float(b);

    let a = 2.0_f64.powi(50) + 1.0;
    let c = dpf_full_product(a, a);
    println!("{:?}", c);
    let d = int_full_product(a, a);
    println!("{:?}", d);

    let a = U256b52([
        522065082635604,
        3957429228622370,
        3604049937975926,
        1024102382665162,
        1561280683024766,
    ]);
    let value = U256b64::from(a);
    let end = U256b52::from(value);
    println!("value: {value:?}");
    println!("a:\t {a:?}");
    println!("end:\t {end:?}");

    let (a, b) = (U256b64([0, 0, 1, 0]), U256b64([0, 65535, 0, 0]));
    println!("{a:?}\t{b:?}");
    print_bits(&[0, 0, 1, 0]);
    println!("");
    print_bits(&[0, 65535, 0, 0]);
    let c = school_method(a, b);
    println!("{c:?}");
    println!("b52");
    let U256b52(a52) = a.into();
    let U256b52(b52) = b.into();
    print_bits(&a52);
    println!("");
    print_bits(&b52);
    // float conversion works
    let a52 = a52.map(|ai| ai as f64);
    let b52 = b52.map(|bi| bi as f64);
    println!("float {a52:?}\t{b52:?}");
    let fres = sampled_product(a52, b52);
    println!("{fres:?}");
    let r: U256b64 = U256b52(fres[..5].try_into().unwrap()).into();
    println!("{r:?}");
}

fn print_float(num: f64) {
    let bits = num.to_bits();
    let sign = (bits >> 63) & 1;
    let exponent = (bits >> 52) & 0x7FF;
    let mantissa = bits & 0xFFFFFFFFFFFFF;
    println!(
        // "{num}: \t sign: {:b}, exponent: {:011b}/{exponent}\t mantissa: {:052b}/{mantissa}",
        "sign: {:b}, exponent: {:011b}/{exponent}\t mantissa: {:052b}/{mantissa}",
        sign, exponent, mantissa
    );
}

fn print_bits(nums: &[u64]) {
    for (i, &num) in nums.iter().enumerate() {
        print!("{num:b}\t");
    }
    println!("");
}

// Mention endianness
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct U256b64(pub [u64; 4]);
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct U256b52(pub [u64; 5]);

// Even u8 seems to big, max size is 64 so that's
// 2^6
// Can I use generic here to make it perform better?
// Should this become a macro?

// Helper function specifically for 64-to-52 bit conversion

impl From<U256b64> for U256b52 {
    fn from(u: U256b64) -> Self {
        let U256b64(limbs) = u;
        let [l0, l1, l2, l3] = limbs;
        U256b52([
            l0 & MASK52, // Lower 52 bits
            ((l0 >> 52) | (l1 << 12)) & MASK52,
            ((l1 >> 40) | (l2 << 24)) & MASK52,
            ((l2 >> 28) | (l3 << 36)) & MASK52,
            l3 >> 16,
        ])
    }
}

// Generalized shifting algorithm

impl From<U256b52> for U256b64 {
    fn from(u: U256b52) -> Self {
        let U256b52(limbs) = u;
        let [l0, l1, l2, l3, l4] = limbs;
        U256b64([
            l0 | (l1 << 52),
            ((l1 >> 12) | (l2 << 40)),
            ((l2 >> 24) | (l3 << 28)),
            ((l3 >> 36) | (l4 << 16)),
        ])
    }
}

impl Arbitrary for U256b52 {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        U256b52([
            u64::arbitrary(g) & MASK52,
            u64::arbitrary(g) & MASK52,
            u64::arbitrary(g) & MASK52,
            u64::arbitrary(g) & MASK52,
            u64::arbitrary(g) & MASK48,
        ])
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let U256b52([l0, l1, l2, l3, l4]) = *self;

        // If all limbs are 0, nothing to shrink
        if l0 == 0 && l1 == 0 && l2 == 0 && l3 == 0 && l4 == 0 {
            return Box::new(std::iter::empty());
        }

        // Create vector of shrunk values
        let mut shrunk = Vec::new();

        // Add zero as the smallest possible value
        shrunk.push(U256b52([0, 0, 0, 0, 0]));

        // Try shrinking each non-zero limb to 0 while keeping others
        if l0 != 0 {
            shrunk.push(U256b52([0, l1, l2, l3, l4]));
        }
        if l1 != 0 {
            shrunk.push(U256b52([l0, 0, l2, l3, l4]));
        }
        if l2 != 0 {
            shrunk.push(U256b52([l0, l1, 0, l3, l4]));
        }
        if l3 != 0 {
            shrunk.push(U256b52([l0, l1, l2, 0, l4]));
        }
        if l4 != 0 {
            shrunk.push(U256b52([l0, l1, l2, l3, 0]));
        }

        // Try halving each non-zero limb
        if l0 != 0 {
            shrunk.push(U256b52([l0 >> 1, l1, l2, l3, l4]));
        }
        if l1 != 0 {
            shrunk.push(U256b52([l0, l1 >> 1, l2, l3, l4]));
        }
        if l2 != 0 {
            shrunk.push(U256b52([l0, l1, l2 >> 1, l3, l4]));
        }
        if l3 != 0 {
            shrunk.push(U256b52([l0, l1, l2, l3 >> 1, l4]));
        }
        if l4 != 0 {
            shrunk.push(U256b52([l0, l1, l2, l3, l4 >> 1]));
        }

        Box::new(shrunk.into_iter())
    }
}

impl Arbitrary for U256b64 {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        U256b64([
            u64::arbitrary(g),
            u64::arbitrary(g),
            u64::arbitrary(g),
            u64::arbitrary(g),
        ])
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let U256b64([l0, l1, l2, l3]) = *self;

        // If all limbs are 0, nothing to shrink
        if l0 == 0 && l1 == 0 && l2 == 0 && l3 == 0 {
            return Box::new(std::iter::empty());
        }

        // Create vector of shrunk values
        let mut shrunk = Vec::new();

        // Add zero as the smallest possible value
        shrunk.push(U256b64([0, 0, 0, 0]));

        // Try shrinking each non-zero limb to 0 while keeping others
        if l0 != 0 {
            shrunk.push(U256b64([0, l1, l2, l3]));
        }
        if l1 != 0 {
            shrunk.push(U256b64([l0, 0, l2, l3]));
        }
        if l2 != 0 {
            shrunk.push(U256b64([l0, l1, 0, l3]));
        }
        if l3 != 0 {
            shrunk.push(U256b64([l0, l1, l2, 0]));
        }

        // Try halving each non-zero limb
        if l0 != 0 {
            shrunk.push(U256b64([l0 >> 1, l1, l2, l3]));
        }
        if l1 != 0 {
            shrunk.push(U256b64([l0, l1 >> 1, l2, l3]));
        }
        if l2 != 0 {
            shrunk.push(U256b64([l0, l1, l2 >> 1, l3]));
        }
        if l3 != 0 {
            shrunk.push(U256b64([l0, l1, l2, l3 >> 1]));
        }

        Box::new(shrunk.into_iter())
    }
}

#[cfg(test)]
mod tests {
    use crate::subtraction_step;
    use crate::subtraction_step_u52;
    use crate::U52_NP0;
    use crate::U52_P;
    use crate::U52_R2;

    use super::int_full_product;
    use super::sampled_product_masked;
    use super::set_round_to_zero;
    use super::MASK52;
    use super::*; // Make sure we're importing everything from parent
    use super::{school_method, U256b52, U256b64};
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn conv64_52(a: U256b64) -> bool {
        let converted = U256b52::from(a);
        let reconverted = U256b64::from(converted);
        if a != reconverted {
            println!("conv64_52 failed:");
            println!("Original U256b64: {:?}", a);
            println!("After conversion to U256b52: {:?}", converted);
            println!("After reconversion to U256b64: {:?}", reconverted);
        }
        a == reconverted
    }

    #[quickcheck]
    fn conv52_64(a: U256b52) -> bool {
        let converted = U256b64::from(a);
        let reconverted = U256b52::from(converted);
        if a != reconverted {
            println!("conv52_64 failed:");
            println!("Original U256b52: {:?}", a);
            println!("After conversion to U256b64: {:?}", converted);
            println!("After reconversion to U256b52: {:?}", reconverted);
        }
        a == reconverted
    }
    #[quickcheck]
    fn multiplication_mask(a: u64, b: u64) -> bool {
        set_round_to_zero();
        let a = a & MASK52;
        let b = b & MASK52;
        let (lo, hi) = int_full_product(a as f64, b as f64);
        (lo | hi << 52, hi >> 12) == a.widening_mul(b)
    }

    #[quickcheck]
    fn multiplication_dpf(a: u64, b: u64) -> bool {
        set_round_to_zero();
        let a = a & MASK52;
        let b = b & MASK52;
        // Write generic shifting operations
        let (lo, hi) = dpf_full_product_u64(a as f64, b as f64);
        let (lo, hi) = (lo as u64, hi as u64);
        (lo | hi << 52, hi >> 12) == a.widening_mul(b)
    }

    // Bidirectional doesn't work as u64 -> 2 x u52 -> 104 -> 2x u64
    // Should be fixable but do save it for later
    // #[quickcheck]
    // fn test_bidirectional_conversion(s: Vec<u64>) -> bool {
    //     let c = convert_limb_sizes(&s, 256, 64, 52);
    //     s == convert_limb_sizes(&c, 256, 52, 64)
    // }

    // Test if school multiplication is correct
    // School multiplication can be tested with u8 and u32 and that should point out if it works.
    // Maybe generalize sampled_product, later we can make it such that it is optimized for a certain size without any overhead
    #[quickcheck]
    fn long_multiplication(a: U256b64, b: U256b64) -> bool {
        set_round_to_zero();
        let res = school_method(a, b);
        let U256b52(a52) = a.into();
        let U256b52(b52) = b.into();
        let fres = sampled_product_masked(a52.map(|ai| ai as f64), b52.map(|bi| bi as f64));

        let cres = convert_limb_sizes(&res, 256, 64, 52);
        cres == fres
    }

    #[quickcheck]
    fn long_multiplication_sampled(a: U256b52, b: U256b52) -> bool {
        set_round_to_zero();
        sampled_product(a.0.map(|x| x as f64), b.0.map(|x| x as f64))
            == sampled_product_masked(a.0.map(|x| x as f64), b.0.map(|x| x as f64))
    }

    // This comparison doesn't make sense
    #[quickcheck]
    fn sos_round(a: U256b52) -> bool {
        let a_tilde = super::sos(a, U256b52(U52_R2), U256b52(U52_P), U52_NP0);
        let a_round = super::sos(
            U256b52(a_tilde[5..].try_into().unwrap()),
            U256b52([1, 0, 0, 0, 0]),
            U256b52(U52_P),
            U52_NP0,
        );

        let mut d = a.0;
        let mut prev = d;
        loop {
            d = subtraction_step_u52(d, U52_P);
            if d == prev {
                break;
            }
            prev = d;
        }

        d == subtraction_step_u52(a_round[5..].try_into().unwrap(), U52_P)
    }
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn set_round_to_zero() {
    unsafe {
        // Set RMode (bits 22-23) to 0b11 for round toward zero
        core::arch::asm!(
            "mrs {tmp}, fpcr",             // Read current FPCR
            "orr {tmp}, {tmp}, #0b11<<22", // Set RMode bits to 11 using bit shift notation
            "msr fpcr, {tmp}",             // Write back to FPCR
            tmp = out(reg) _
        );
    }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline]
pub fn set_round_to_zero() {
    // No-op or panic depending on your needs for non-ARM platforms
    unimplemented!("Round to zero is only implemented for ARM64");
}
