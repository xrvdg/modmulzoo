use quickcheck::Arbitrary;

use crate::{MASK48, MASK52};

mod uint52 {
    use crate::MASK52;

    use super::U256b52;

    #[inline(always)]
    pub fn adds_u52(t: &mut [u64], mut carry: u64) {
        for ti in t {
            // Performance heavily affected by this carry check
            // if carry == 0 {
            //     break;
            // }
            let tmp = *ti + carry;
            (*ti, carry) = (tmp & MASK52, tmp >> 52);
        }
    }

    pub fn sos_u52(a: U256b52, b: U256b52, n: U256b52, n0: u64) -> [u64; 10] {
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

    fn carry_add_u52(lhs: u64, carry: u64) -> (u64, u64) {
        let tmp = lhs + carry;
        (tmp & MASK52, tmp >> 52)
    }

    // Has excessive shifting and masking
    pub fn cios_opt(a: U256b52, b: U256b52, n: U256b52, np0: u64) -> [u64; 7] {
        let a = a.0;
        let b = b.0;
        let n = n.0;

        let mut t = [0_u64; 7];
        for i in 0..a.len() {
            let mut carry = 0;
            for j in 0..b.len() {
                (t[j], carry) = carrying_mul_add_u104(a[i], b[j], t[j], carry);
            }
            (t[b.len()], t[b.len() + 1]) = carry_add_u52(t[b.len()], carry);

            // Last entry can probably be skipped brings it closer to Yuval's. It's mostly the last
            // carry add that makes the difference
            // (t[b.len()], _) = carry_add(t[b.len()], carry);

            let mut carry = 0;
            let m = t[0].wrapping_mul(np0) & MASK52;
            // Outside of the loop because the loop does shifting
            (_, carry) = carrying_mul_add_u104(m, n[0], t[0], carry);

            for j in 1..n.len() {
                (t[j - 1], carry) = carrying_mul_add_u104(m, n[j], t[j], carry);
            }
            (t[n.len() - 1], carry) = carry_add_u52(t[n.len()], carry);
            // Last shift can probably be skipped. This brings it very close to Yuval's numbers
            (t[n.len()], _) = carry_add_u52(t[n.len() + 1], carry);
        }
        t
    }

    #[inline(always)]
    // Useful as a function to convert an existing algorithm to u104
    pub fn carrying_mul_add_u104(a: u64, b: u64, add: u64, carry: u64) -> (u64, u64) {
        let c: u128 = a as u128 * b as u128 + carry as u128 + add as u128;
        (c as u64 & MASK52, (c >> 52) as u64)
    }
    #[cfg(test)]
    mod tests {
        use quickcheck_macros::quickcheck;

        use super::U256b52;
        use crate::{subtraction_step_u52, U52_NP0, U52_P, U52_R2};
        #[quickcheck]
        fn sos_round(a: U256b52) -> bool {
            let a_tilde = super::sos_u52(a, U256b52(U52_R2), U256b52(U52_P), U52_NP0);
            let a_round = super::sos_u52(
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

        #[quickcheck]
        fn cios_round(a: U256b52) -> bool {
            let a_tilde = super::cios_opt(a, U256b52(U52_R2), U256b52(U52_P), U52_NP0);
            let a_round = super::cios_opt(
                U256b52(a_tilde[..5].try_into().unwrap()),
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

            d == subtraction_step_u52(a_round[..5].try_into().unwrap(), U52_P)
        }
    }
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

pub fn cios_opt_f64(a: U256b52, b: U256b52, n: U256b52, np0: u64) -> [u64; 6] {
    let a = a.0;
    let b = b.0;
    let n = n.0;

    let mut t = [0_u64; 6];
    for i in 0..a.len() {
        // a_i * B
        for j in 0..b.len() {
            let p_hi = (a[i] as f64).mul_add(b[j] as f64, C1);
            let p_lo = (a[i] as f64).mul_add(b[j] as f64, C2 - p_hi);
            // TODO(xrvdg) optimize subtractions
            t[j + 1] += p_hi.to_bits() - C1.to_bits();
            t[j] += p_lo.to_bits() - C3.to_bits();
        }

        let m = (t[0].wrapping_mul(np0) & MASK52) as f64;
        // Outside of the loop because the loop does shifting
        let p_hi = m.mul_add(n[0] as f64, C1);
        let p_lo = m.mul_add(n[0] as f64, C2 - p_hi);
        // TODO(xrvdg) optmize subtractions
        // TODO(xrvdg) Don't write to a memory address, it's thrown away
        t[0] += p_lo.to_bits() - C3.to_bits();
        // Doesn't this shift already do most of the heavy work
        t[1] += (p_hi.to_bits() - C1.to_bits()) + (t[0] >> 52);

        for j in 1..n.len() {
            let p_hi = m.mul_add(n[j] as f64, C1);
            let p_lo = m.mul_add(n[j] as f64, C2 - p_hi);
            // TODO(xrvdg) optmize subtractions
            // Worried about read after write. Is the carry formulation maybe better?
            t[j + 1] += p_hi.to_bits() - C1.to_bits();
            t[j - 1] = t[j] + (p_lo.to_bits() - C3.to_bits());
        }
        t[n.len() - 1] = t[n.len()];
        t[n.len()] = 0;
        // t[n.len()] = t[n.len() + 1];
        // t[n.len()] = (t[n.len()] >> 52) + t[n.len() + 1];
    }

    // This takes a 5ns
    let mut carry = 0;
    // When we return we only look at the first 5
    // Could reduce the round with one
    // Could we deal with the carries later? As in the next round of the multiplication

    for i in 0..t.len() {
        let tmp = t[i] + carry;
        t[i] = tmp & MASK52;
        carry = tmp >> 52;
    }
    t
}

pub fn cios_opt_sub_f64(a: U256b52, b: U256b52, n: U256b52, np0: u64) -> [u64; 6] {
    let a = a.0;
    let b = b.0;
    let n = n.0;

    let mut t = [0_u64; 6];
    for i in 0..t.len() - 1 {
        t[i] = make_initial(2 + 2 * i, 2 * i);
    }

    for i in 0..a.len() {
        t[n.len()] = make_initial(10 - 2 - 2 * i, 10 - 2 * i);
        // a_i * B
        for j in 0..b.len() {
            let p_hi = (a[i] as f64).mul_add(b[j] as f64, C1);
            let p_lo = (a[i] as f64).mul_add(b[j] as f64, C2 - p_hi);
            // TODO(xrvdg) optimize subtractions
            t[j + 1] = t[j + 1].wrapping_add(p_hi.to_bits());
            t[j] = t[j].wrapping_add(p_lo.to_bits());
        }

        let m = (t[0].wrapping_mul(np0) & MASK52) as f64;
        // Outside of the loop because the loop does shifting
        let p_hi = m.mul_add(n[0] as f64, C1);
        let p_lo = m.mul_add(n[0] as f64, C2 - p_hi);
        // TODO(xrvdg) optmize subtractions
        // TODO(xrvdg) Don't write to a memory address, it's thrown away
        t[0] = t[0].wrapping_add(p_lo.to_bits());
        t[1] = t[1].wrapping_add((p_hi.to_bits()) + (t[0] >> 52));

        for j in 1..n.len() {
            let p_hi = m.mul_add(n[j] as f64, C1);
            let p_lo = m.mul_add(n[j] as f64, C2 - p_hi);
            // TODO(xrvdg) optmize subtractions
            // Worried about read after write. Is the carry formulation maybe better?
            t[j + 1] = t[j + 1].wrapping_add(p_hi.to_bits());
            t[j - 1] = t[j].wrapping_add(p_lo.to_bits());
        }
        t[n.len() - 1] = t[n.len()];
        // t[n.len()] = t[n.len() + 1];
        // t[n.len()] = (t[n.len()] >> 52) + t[n.len() + 1];
    }

    // This takes a 5ns
    let mut carry = 0;
    // When we return we only look at the first 5
    // Could reduce the round with one
    // Could we deal with the carries later? As in the next round of the multiplication

    for i in 0..t.len() {
        let tmp = t[i] + carry;
        t[i] = tmp & MASK52;
        carry = tmp >> 52;
    }
    t
}

// Batch all the subtractions on t[i] together
// #[inline(never)]
// Bester performing version so far
pub fn fios_opt_sub_f64(a: U256b52, b: U256b52, n: U256b52, np0: u64) -> [u64; 6] {
    let a = a.0;
    let b = b.0;
    let n = n.0;

    let mut t = [0_u64; 6];
    for i in 0..t.len() - 1 {
        t[i] = make_initial(2 + 2 * i, 2 * i);
    }

    for i in 0..a.len() {
        // a_i * B
        t[n.len()] = make_initial(2 * (n.len() - 1 - i), 2 * (n.len() - i));
        let p_hi = (a[i] as f64).mul_add(b[0] as f64, C1);
        let p_lo = (a[i] as f64).mul_add(b[0] as f64, C2 - p_hi);

        t[0] = t[0].wrapping_add(p_lo.to_bits());
        t[1] = t[1].wrapping_add(p_hi.to_bits());
        let m = (t[0].wrapping_mul(np0) & MASK52) as f64;
        // Outside of the loop because the loop does division by shifting
        let p_hi = m.mul_add(n[0] as f64, C1);
        let p_lo = m.mul_add(n[0] as f64, C2 - p_hi);
        // Only interested in the carry bits of t[0], that's why we are not writing it back to
        // t[0]
        let carry_t0 = (t[0].wrapping_add(p_lo.to_bits())) >> 52;
        // Doesn't this shift already do most of the heavy work
        t[1] += p_hi.to_bits() + carry_t0;

        for j in 1..b.len() {
            let ab_hi = (a[i] as f64).mul_add(b[j] as f64, C1);
            let ab_lo = (a[i] as f64).mul_add(b[j] as f64, C2 - ab_hi);
            let mn_hi = m.mul_add(n[j] as f64, C1);
            let mn_lo = m.mul_add(n[j] as f64, C2 - mn_hi);
            // TODO(xrvdg) optmize subtractions
            // Worried about read after write. Is the carry formulation maybe better?
            // TODO(xrvdg) optimize subtractions
            // How does the assembly of this piece look like. Is the to_bits shared?
            // The t[j+1] can be kept in memory. Debating  whether it makes a different to make  this a carry
            // Acar counts write as just writing to any variable

            // Another option is to first subtract the hi from each other and then convert to bits, but that
            // appears to be slower.
            t[j + 1] = t[j + 1].wrapping_add(ab_hi.to_bits() + mn_hi.to_bits());
            t[j - 1] = t[j].wrapping_add(ab_lo.to_bits() + mn_lo.to_bits());
        }
        t[n.len() - 1] = t[n.len()];
    }

    // println!("c: {c:?}");
    // println!("count: {count:?}");
    // This takes a 5ns
    let mut carry = 0;
    // When we return we only look at the first 5
    // Could reduce the round with one
    // Could we deal with the carries later? As in the next round of the multiplication

    for ti in &mut t {
        let tmp = *ti + carry;
        *ti = tmp & MASK52;
        carry = tmp >> 52;
    }
    t
}

pub fn fios_opt_f64(a: U256b52, b: U256b52, n: U256b52, np0: u64) -> [u64; 6] {
    let a = a.0;
    let b = b.0;
    let n = n.0;

    let mut t = [0_u64; 6];

    for i in 0..a.len() {
        // a_i * B
        let p_hi = (a[i] as f64).mul_add(b[0] as f64, C1);
        let p_lo = (a[i] as f64).mul_add(b[0] as f64, C2 - p_hi);

        t[0] += p_lo.to_bits() - C3.to_bits();
        t[1] += p_hi.to_bits() - C1.to_bits();
        let m = (t[0].wrapping_mul(np0) & MASK52) as f64;
        // Outside of the loop because the loop does shifting
        let p_hi = m.mul_add(n[0] as f64, C1);
        let p_lo = m.mul_add(n[0] as f64, C2 - p_hi);
        // TODO(xrvdg) optmize subtractions
        // TODO(xrvdg) Don't write to a memory address, it's thrown away

        // Only interested in the carry bits of t[0], that's why we are not writing it back to
        // t[0].
        let carry_t0 = (t[0] + p_lo.to_bits() - C3.to_bits()) >> 52;
        // Doesn't this shift already do most of the heavy work
        t[1] += (p_hi.to_bits() - C1.to_bits()) + carry_t0;

        for j in 1..b.len() {
            let ab_hi = (a[i] as f64).mul_add(b[j] as f64, C1);
            let ab_lo = (a[i] as f64).mul_add(b[j] as f64, C2 - ab_hi);
            let mn_hi = m.mul_add(n[j] as f64, C1);
            let mn_lo = m.mul_add(n[j] as f64, C2 - mn_hi);
            // TODO(xrvdg) optmize subtractions
            // Worried about read after write. Is the carry formulation maybe better?
            // TODO(xrvdg) optimize subtractions
            // How does the assembly of this piece look like. Is the to_bits shared?
            // The t[j+1] can be kept in memory. Debating  whether it makes a different to make  this a carry
            // Acar counts write as just writing to any variable
            t[j + 1] += ab_hi.to_bits() + mn_hi.to_bits() - 2 * C1.to_bits();
            t[j - 1] = t[j] + ab_lo.to_bits() + mn_lo.to_bits() - 2 * C3.to_bits();
        }
        t[n.len() - 1] = t[n.len()];
        t[n.len()] = 0;
    }

    // This takes a 5ns
    let mut carry = 0;
    for ti in &mut t {
        let tmp = *ti + carry;
        *ti = tmp & MASK52;
        carry = tmp >> 52;
    }
    t
}

// Intermediate steps in the paper to explain the algorithm, but not actually used in the implementation
mod paper {
    use crate::MASK52;

    use super::{make_initial, C1, C2, C3};
    /// Technique for extracting the truncated part of a f64 multiplication
    fn full_product(a: f64, b: f64) -> (f64, f64) {
        let p_hi = a.mul_add(b, 0.);
        let p_lo: f64 = a.mul_add(b, -p_hi);
        (p_lo, p_hi)
    }

    /// As in the paper, but does not work as is
    /// The final subtractions causes the value to be normalized floating point again meaning
    /// that we can't use the bit representation.
    /// See [dpf_full_product_u64]
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
        // IMPROVE: The to_bits might become a noop if it's in a SIMD register
    }

    // Masking is likely to be more expensive and can not be combined with the
    // subtraction optimization.
    fn int_full_product(a: f64, b: f64) -> (u64, u64) {
        let p_hi = a.mul_add(b, C1);
        let p_lo = a.mul_add(b, C2 - p_hi);

        (p_lo.to_bits() & MASK52, p_hi.to_bits() & MASK52)
    }

    #[inline(always)]
    pub fn carrying_mul_add_fu104(a: u64, b: u64, add: u64, carry: u64) -> (u64, u64) {
        let (mut lo, mut hi) = dpf_full_product_u64(a as f64, b as f64);
        lo += add + carry;
        hi += lo >> 52;
        (lo & MASK52, hi)
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

        for col_sum in &mut col_sums {
            let tmp = *col_sum + carry;
            *col_sum = tmp & MASK52;
            carry = tmp >> 52;
        }
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

        let mut carry = 0;

        for col_sum in &mut col_sums {
            let tmp = *col_sum + carry;
            *col_sum = tmp & MASK52;
            carry = tmp >> 52;
        }
        col_sums
    }
    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::emmart::*;
        use crate::{convert_limb_sizes, MASK52};
        use quickcheck_macros::quickcheck;

        #[quickcheck]
        /// Compare widening mul to int_full_product
        fn multiplication_mask(a: u64, b: u64) -> bool {
            set_round_to_zero();
            let a = a & MASK52;
            let b = b & MASK52;
            let (lo, hi) = int_full_product(a as f64, b as f64);
            (lo | hi << 52, hi >> 12) == a.widening_mul(b)
        }

        #[quickcheck]
        /// Compare widening mul to dpf_full_product_u64
        fn multiplication_dpf(a: u64, b: u64) -> bool {
            set_round_to_zero();
            let a = a & MASK52;
            let b = b & MASK52;
            let (lo, hi) = dpf_full_product_u64(a as f64, b as f64);
            let (lo, hi) = (lo as u64, hi as u64);
            (lo | hi << 52, hi >> 12) == a.widening_mul(b)
        }

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
    }
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

// These probably have to stay within the 11 bits
// Not totally convinced about the correctness of this one
fn make_initial(low_count: usize, high_count: usize) -> u64 {
    let val = high_count * 0x467 + low_count * 0x433;
    -((val as i64 & 0xFFF) << 52) as u64
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

// Cost of getting it into the right from
// Seeing that there is exponentiation could it be profitable to keep in a certain range?
// Proper types
// TODO write a proper test suite
// Does rust have something like quickcheck?

// Do multiplication on the float representation and to convert it to 52-bit integer
// you add 2^52 and mask it.

// Looks like the value first needs to be converted to float

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

    use crate::subtraction_step_u52;
    use crate::U52_NP0;
    use crate::U52_P;
    use crate::U52_R2;

    use super::set_round_to_zero;
    use super::{U256b52, U256b64};
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

    // This comparison doesn't make sense

    #[quickcheck]
    fn cios_f64_sub_round(a: U256b52) -> bool {
        set_round_to_zero();
        let a_tilde = super::cios_opt_sub_f64(a, U256b52(U52_R2), U256b52(U52_P), U52_NP0);
        let a_round = super::cios_opt_sub_f64(
            U256b52(a_tilde[..5].try_into().unwrap()),
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

        d == subtraction_step_u52(a_round[..5].try_into().unwrap(), U52_P)
    }

    #[quickcheck]
    fn fios_f64_sub_round(a: U256b52) -> bool {
        set_round_to_zero();
        let a_tilde = super::fios_opt_sub_f64(a, U256b52(U52_R2), U256b52(U52_P), U52_NP0);
        let a_round = super::fios_opt_sub_f64(
            U256b52(a_tilde[..5].try_into().unwrap()),
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

        d == subtraction_step_u52(a_round[..5].try_into().unwrap(), U52_P)
    }

    #[quickcheck]
    fn fios_f64_round(a: U256b52) -> bool {
        set_round_to_zero();
        let a_tilde = super::fios_opt_f64(a, U256b52(U52_R2), U256b52(U52_P), U52_NP0);
        let a_round = super::fios_opt_f64(
            U256b52(a_tilde[..5].try_into().unwrap()),
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

        d == subtraction_step_u52(a_round[..5].try_into().unwrap(), U52_P)
    }

    #[quickcheck]
    fn cios_f64_round(a: U256b52) -> bool {
        set_round_to_zero();
        let a_tilde = super::cios_opt_f64(a, U256b52(U52_R2), U256b52(U52_P), U52_NP0);
        let a_round = super::cios_opt_f64(
            U256b52(a_tilde[..5].try_into().unwrap()),
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

        d == subtraction_step_u52(a_round[..5].try_into().unwrap(), U52_P)
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
