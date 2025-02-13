use num_modular::{Montgomery, Reducer};
use num_traits::Unsigned;

fn main() {
    println!("Hello, world!");

    // These values are going to be outside of the allowed range
    // 255ˆ2 = 65025
    // Just see what it does and understand what the problem is
    let a = 25;
    let b = 25;

    let p: u32 = a * b;

    println!("p: {}", p);
    println!("p mod 97: {}", p % 97);
    println!("p mod 100: {}", p % 100);

    let red = montgomery_reduction(p, 97, 100);

    println!("montgomery reduction: {}", red);
    // How to undo the reduction properly?
    println!("montgomery reduction reversed: {}", red * 100 % 97);
    println!("montgomery reduction reversed: {}", red * (100 % 97));
    println!(
        "montgomery reduction reversed: {}",
        montgomery_reduction(red, 97, 100)
    );

    println!("");

    let red = montgomery_reduction_power_of_two(p, 97, 256);

    println!("montgomery reduction: {}", red);
    // How to undo the reduction properly?
    println!("montgomery reduction reversed: {}", red * 256 % 97);
    println!("montgomery reduction reversed: {}", red * (256 % 97));
    println!(
        "montgomery reduction reversed: {}",
        montgomery_reduction_power_of_two(red, 97, 256)
    );

    println!("");
    let a = 4;
    let b = 5;
    let p = a * b;

    let n = u8::MAX as u16 * u8::MAX as u16;
    println!("");
    let a = n - 1;
    let b = n - 1;

    let r = u16::MAX as u32 + 1;
    println!("n: {} \t r: {}", n, r);
    let bytes_a: [u8; 4] = (a as u32).to_le_bytes();
    let values_a = [u16::from_le_bytes([bytes_a[0], bytes_a[1]])];
    let bytes_b: [u8; 4] = (b as u32).to_le_bytes();
    let values_b = [u16::from_le_bytes([bytes_b[0], bytes_b[1]])];
    let red = montgomery_reduction_overflow((a as u32 * b as u32) as u32, n as u32, r as u32);
    println!("montgomery reduction overflow: {}", red);
    let red = montgomery_reduction((a as u32 * b as u32) as u32, n as u32, r as u32);
    println!("montgomery reduction: {}", red);
    let red = montgomery_reduction_base16(&values_a, &values_b, n.try_into().unwrap(), r);
    println!("montgomery reduction base16: {}", red);
    let red = Montgomery::<u16>::new(n as u16).mul(&b, &a);
    println!("montgomery num_modular: {}", red);

    let red = montgomery_reduction_base8(
        &bytes_a[0..2],
        &bytes_b[0..2],
        n.to_le_bytes(),
        r.to_le_bytes(),
    );
    println!("montgomery reduction base8: {}", red);
}

// This version works for any r
// p < nˆ2
// Don't understand the constraint on p
// looks like it for the bounding step, but for my numbers I haven't found a case where it's violated
// or is it for some integer bound
pub fn montgomery_reduction(p: u32, n: u32, r: u32) -> u32 {
    let u = montgomery_constant(n, r);
    // While it is unnecessary mathematically, it reduces the size of p

    // Would like to have widening and narrowing operations, but if you have the register size and it fills it already.
    // Things is that it was recommended on NEON to not do the widening yourself
    let q = u * (p % r) % r;
    // let q = u * p % r;
    let c = (p + n * q) / r;
    if c >= n {
        c - n
    } else {
        c
    }
}

pub fn montgomery_reduction_base16_single(a: &[u16], b: u32, n: u16, r: u32) -> u16 {
    let u = montgomery_constant(n as u32, r);
    // While it is unnecessary mathematically, it reduces the size of p

    // Would like to have widening and narrowing operations, but if you have the register size and it fills it already.
    // Things is that it was recommended on NEON to not do the widening yourself

    let mut c: u32 = 0;
    for i in 0..a.len() - 1 {
        c = c + a[i] as u32 * b;
        let q = (u % r) * (c % r) % r;
        // let q = u * p % r;
        c = (c + n as u32 * q) / r;
    }

    if c >= n as u32 {
        (c - (n as u32)) as u16
    } else {
        c as u16
    }
}

// Probably allows for too big inputs? a * b < nˆ2
pub fn montgomery_reduction_base16(a: &[u16], b: &[u16], n: u16, r: u32) -> u16 {
    let u = montgomery_constant(n as u32, r);
    // While it is unnecessary mathematically, it reduces the size of p

    // Would like to have widening and narrowing operations, but if you have the register size and it fills it already.
    // Things is that it was recommended on NEON to not do the widening yourself

    let mut c: u32 = 0;
    for i in 0..a.len() {
        for j in 0..b.len() {
            c = c + a[i] as u32 * b[j] as u32;
            let q = (u % r) * (c % r) % r;
            // let q = u * p % r;
            c = (c + n as u32 * q) / r;
        }
    }

    if c >= n as u32 {
        (c - (n as u32)) as u16
    } else {
        c as u16
    }
}

pub fn montgomery_reduction_base8(a: &[u8], b: &[u8], n: [u8; 2], r_bytes: [u8; 4]) -> u16 {
    println!("a: {:?}", a);
    println!("b: {:?}", b);
    println!("n: {:?}", n);
    println!("r: {:?}", r_bytes);

    let n_u32 = u32::from_le_bytes([n[0], n[1], 0, 0]);
    println!("n_u32: {}", n_u32);

    let r_u32 = u32::from_le_bytes(r_bytes);
    println!("r_u32: {}", r_u32);

    if (u8::MAX as u32) >= n_u32 {
        panic!("n_u32 must be greater than u8::MAX");
    }
    if n_u32 >= r_u32 {
        panic!("n_u32 must be less than r_u32");
    }

    let base = 1 << 8;
    println!("base: {}", base);

    // Calculate u using the full modulus
    let u = montgomery_constant(n_u32, r_u32);
    println!("u: {}", u);

    let mut c: u32 = 0;
    for i in 0..b.len() {
        for j in 0..a.len() {
            c += (a[j] as u32) * (b[i] as u32);
            println!("c after multiplication: {}", c);

            // Do one reduction per multiplication
            let q = ((u % base) * (c % base)) % base;
            println!("q: {}", q);
            c = (c + (n_u32 * q)) / base;
            println!("c after reduction: {}", c);
        }
    }

    while c >= n_u32 {
        c -= n_u32;
    }

    c as u16
}

pub fn montgomery_reduction_overflow(p: u32, n: u32, r: u32) -> u32 {
    let u = montgomery_constant(n, r);
    // While it is unnecessary mathematically, it reduces the size of p

    // Would like to have widening and narrowing operations, but if you have the register size and it fills it already.
    // Things is that it was recommended on NEON to not do the widening yourself
    let q = (u.overflowing_mul(p).0) % r;
    // let q = u * p % r;
    let c = (p + n * q) / r;
    if c >= n {
        c - n
    } else {
        c
    }
}

// Do I ever need to reduce things in parallel?
// Do I ever need to explicitly reduce or can it be done in the multiplication step?

//blackbox compare with a power of two version?
// interesting that this is slower
pub fn montgomery_reduction_power_of_two(p: u32, n: u32, r: u32) -> u32 {
    // wouldn't lcnt be faster?
    if r.count_ones() != 1 {
        panic!("r must be a power of two");
    };
    let u = montgomery_constant(n, r);
    let bit_position = r.trailing_zeros();
    let mask = r - 1;
    // While it is unnecessary mathematically, it reduces the size of p
    let q = (u * (p & mask)) & mask;
    // let q = u * p % r;
    let c = (p + n * q) >> bit_position;
    if c >= n {
        c - n
    } else {
        c
    }
}

// how to unreduce?

// Better way to compute the inverse?
// r > n
// output and inputs are maxed out by r
fn montgomery_constant<T: Unsigned + Copy>(n: T, r: T) -> T {
    let mut u = T::one();
    // find inverse of n
    while ((u) * (n)) % r != T::one() {
        u = u + T::one();
    }
    // negate
    r - u
}

/// CIOS (Coarsely Integrated Operand Scanning) Montgomery multiplication.
/// This implementation processes operands word by word, combining multiplication and reduction steps.
///
/// # Arguments
/// * `x` - First operand as array of words (least significant word first)
/// * `y` - Second operand as array of words (least significant word first)
/// * `m` - Modulus as array of words (least significant word first)
/// * `mp` - Montgomery constant (-m^(-1) mod 2^w) for word size w
/// * `n` - Number of words
/// * `word_size` - Size of each word in bits (typically 8, 16, 32, or 64)
pub fn montgomery_multiply_cios(
    x: &[u32],
    y: &[u32],
    m: &[u32],
    mp: u32,
    n: usize,
    word_size: u32,
) -> Vec<u32> {
    assert!(x.len() >= n && y.len() >= n && m.len() >= n);
    assert!(word_size <= 32);

    let word_mask = (1u64 << word_size) - 1;
    let mut t = vec![0u32; n + 1];

    // Main CIOS loop
    for i in 0..n {
        let mut c: u64 = 0; // Carry for multiplication

        // Step 1: Multiply i-th word of x with y and add to t
        for j in 0..n {
            let product = (t[j] as u64) + (x[i] as u64 * y[j] as u64) + c;
            t[j] = (product & word_mask) as u32;
            c = product >> word_size;
        }
        t[n] = c as u32;

        // Step 2: Compute quotient for reduction
        let u: u64 = ((t[0] as u64 * mp as u64) & word_mask) as u64;

        // Step 3: Multiply quotient with modulus and add
        c = 0;
        for j in 0..n {
            let product = t[j] as u64 + (u * m[j] as u64) + c;
            t[j] = (product & word_mask) as u32;
            c = product >> word_size;
        }

        // Handle final carry
        let mut carry = (t[n] as u64 + c) & word_mask;
        t[n] = carry as u32;

        // Step 4: Shift right (divide by word base)
        for j in 0..n {
            t[j] = t[j + 1];
        }
        t[n] = 0;
    }

    // Final reduction if needed
    if compare_arrays(&t[0..n], m) >= 0 {
        let mut borrow = 0i64;
        for j in 0..n {
            let diff = t[j] as i64 - m[j] as i64 - borrow;
            t[j] = (diff & word_mask as i64) as u32;
            borrow = if diff < 0 { 1 } else { 0 };
        }
    }

    t[0..n].to_vec()
}

/// Helper function to compare two arrays of equal length
fn compare_arrays(a: &[u32], b: &[u32]) -> i32 {
    for i in (0..a.len()).rev() {
        if a[i] < b[i] {
            return -1;
        }
        if a[i] > b[i] {
            return 1;
        }
    }
    0
}

fn carrying_mul(a: u8, b: u8, add: u8, carry: u8) -> (u8, u8) {
    let c: u16 = a as u16 * b as u16 + carry as u16 + add as u16;
    (c as u8, (c >> 8) as u8)
}

fn multiply_precision(a: &[u8], b: &[u8]) -> Vec<u8> {
    let n = a.len() + b.len();
    let mut out = vec![0; n];

    let mut carry = 0;

    for i in 0..a.len() {
        carry = 0;
        for j in 0..b.len() {
            (out[i + j], carry) = carrying_mul(a[i], b[j], out[i + j], carry);
        }
        out[i + b.len()] = carry;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        // Add your test cases here
    }

    #[test]
    fn test_montgomery_multiply_cios() {
        // Test case with small numbers
        // Using 8-bit words for simplicity
        let x = vec![0x12, 0x34]; // 0x3412 in little-endian
        let y = vec![0x56, 0x78]; // 0x7856 in little-endian
        let m = vec![0x89, 0xAB]; // 0xAB89 in little-endian
        let mp = 0x89; // Example Montgomery constant

        let result = montgomery_multiply_cios(&x, &y, &m, mp, 2, 8);

        // Verify result is in valid range
        assert!(compare_arrays(&result, &m) < 0);
    }
}
