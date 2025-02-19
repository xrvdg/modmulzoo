use super::{make_initial, C1, C2, C3};
use crate::MASK52;

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

    let mut col_sums: [u64; 2 * N] = [0; 2 * N];

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
    use crate::arith::school_method;
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
        let res = school_method(a.0, b.0);
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
