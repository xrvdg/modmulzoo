use std::{
    ops::{BitAnd, Mul},
    simd::{self, num::SimdFloat, Simd, StdFloat, ToBytes},
};

/// Make sure to call set_round_to_zero before using any of the functions in this module
pub mod paper;
mod uint52;

pub const MASK52: u64 = 2_u64.pow(52) - 1;
pub const MASK48: u64 = 2_u64.pow(48) - 1;

pub fn subtraction_step_u52<const N: usize>(a: [u64; N], b: [u64; N]) -> [u64; N] {
    let mut borrow: i64 = 0;
    let mut c = [0; N];
    for i in 0..N {
        let tmp = a[i] as i128 - b[i] as i128 + borrow as i128;
        c[i] = (tmp as u64) & MASK52;
        borrow = (tmp >> 52) as i64
    }

    if borrow != 0 {
        a
    } else {
        c
    }
}

/// Modulus operation by repeatedly performing a:=(a-b) until a < b
pub fn modulus_u52<const N: usize>(a: [u64; N], b: [u64; N]) -> [u64; N] {
    let mut d = a;
    let mut prev = d;
    loop {
        d = subtraction_step_u52(d, b);
        if d == prev {
            break;
        }
        prev = d;
    }
    d
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

/// Resolve into non-redundant form meaning that there are no carries in the
/// high [52..] part
#[inline(always)]
fn resolve(mut t: [u64; 6]) -> [u64; 6] {
    let mut carry = 0;
    for i in 0..t.len() {
        let tmp = t[i] + carry;
        t[i] = tmp & MASK52;
        carry = tmp >> 52;
    }
    t
}

#[inline(always)]
fn resolve_simd(t: [Simd<u64, 2>; 6]) -> [[u64; 6]; 2] {
    let mut out = [[0; 6]; 2];
    let mut carry = Simd::splat(0);
    for i in 0..t.len() {
        let tmp = t[i] + carry;
        [out[0][i], out[1][i]] = tmp.bitand(Simd::splat(MASK52)).to_array();
        carry = tmp >> 52;
    }
    out
}

#[inline(always)]
fn resolve_simd_sat(t: [Simd<u64, 4>; 6]) -> [[u64; 6]; 4] {
    let mut out = [[0; 6]; 4];
    let mut carry = Simd::splat(0);
    for i in 0..t.len() {
        let tmp = t[i] + carry;
        [out[0][i], out[1][i], out[2][i], out[3][i]] = tmp.bitand(Simd::splat(MASK52)).to_array();
        carry = tmp >> 52;
    }
    out
}

pub fn stub_resolve_2(tx: [[u64; 6]; 2]) -> [[u64; 6]; 2] {
    resolve_2(tx)
}

#[inline(always)]
fn resolve_2(mut tx: [[u64; 6]; 2]) -> [[u64; 6]; 2] {
    let mut carry = 0;
    for t in &mut tx {
        for i in 0..t.len() {
            let tmp = t[i] + carry;
            t[i] = tmp & MASK52;
            carry = tmp >> 52;
        }
    }
    tx
}
/// Based on CIOS_OPT from ACAR but with floating point multiplication
pub fn cios_opt(a: [u64; 5], b: [u64; 5], n: [u64; 5], np0: u64) -> [u64; 6] {
    let mut t = [0_u64; 6];
    for i in 0..a.len() {
        // a_i * B
        for j in 0..b.len() {
            let p_hi = (a[i] as f64).mul_add(b[j] as f64, C1);
            let p_lo = (a[i] as f64).mul_add(b[j] as f64, C2 - p_hi);
            t[j + 1] += p_hi.to_bits() - C1.to_bits();
            t[j] += p_lo.to_bits() - C3.to_bits();
        }

        let m = (t[0].wrapping_mul(np0) & MASK52) as f64;
        // Outside of the loop because the loop does shifting
        let p_hi = m.mul_add(n[0] as f64, C1);
        let p_lo = m.mul_add(n[0] as f64, C2 - p_hi);
        t[0] += p_lo.to_bits() - C3.to_bits();
        t[1] += (p_hi.to_bits() - C1.to_bits()) + (t[0] >> 52);

        for j in 1..n.len() {
            let p_hi = m.mul_add(n[j] as f64, C1);
            let p_lo = m.mul_add(n[j] as f64, C2 - p_hi);
            t[j + 1] += p_hi.to_bits() - C1.to_bits();
            t[j - 1] = t[j] + (p_lo.to_bits() - C3.to_bits());
        }
        t[n.len() - 1] = t[n.len()];
        t[n.len()] = 0;
    }

    resolve(t)
}

/// Like cios_opt above but with the subtraction optimisation
/// `t` is initialised with the - sum of exponents
pub fn cios_opt_sub(a: [u64; 5], b: [u64; 5], n: [u64; 5], np0: u64) -> [u64; 6] {
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
            t[j + 1] = t[j + 1].wrapping_add(p_hi.to_bits());
            t[j] = t[j].wrapping_add(p_lo.to_bits());
        }

        let m = (t[0].wrapping_mul(np0) & MASK52) as f64;
        // Outside of the loop because the loop does shifting
        let p_hi = m.mul_add(n[0] as f64, C1);
        let p_lo = m.mul_add(n[0] as f64, C2 - p_hi);
        t[0] = t[0].wrapping_add(p_lo.to_bits());
        t[1] = t[1].wrapping_add((p_hi.to_bits()) + (t[0] >> 52));

        for j in 1..n.len() {
            let p_hi = m.mul_add(n[j] as f64, C1);
            let p_lo = m.mul_add(n[j] as f64, C2 - p_hi);
            t[j + 1] = t[j + 1].wrapping_add(p_hi.to_bits());
            t[j - 1] = t[j].wrapping_add(p_lo.to_bits());
        }
        t[n.len() - 1] = t[n.len()];
    }

    resolve(t)
}

pub fn fios_opt_sub_sat(
    a: [u64; 5],
    b: [u64; 5],
    c: [u64; 5],
    d: [u64; 5],
    n: [u64; 5],
    np0: u64,
) -> [[u64; 6]; 2] {
    let mut out = [[0_u64; 6]; 2];
    for i in 0..out[0].len() - 1 {
        out[0][i] = make_initial(2 + 2 * i, 2 * i);
        out[1][i] = make_initial(2 + 2 * i, 2 * i);
    }

    for i in 0..a.len() {
        // a_i * B
        out[0][n.len()] = make_initial(2 * (n.len() - 1 - i), 2 * (n.len() - i));
        let p_hi = (a[i] as f64).mul_add(b[0] as f64, C1);
        let p_lo = (a[i] as f64).mul_add(b[0] as f64, C2 - p_hi);

        out[0][0] = out[0][0].wrapping_add(p_lo.to_bits());
        out[0][1] = out[0][1].wrapping_add(p_hi.to_bits());
        let m = (out[0][0].wrapping_mul(np0) & MASK52) as f64;
        // Outside of the loop because the loop does division by shifting
        let p_hi = m.mul_add(n[0] as f64, C1);
        let p_lo = m.mul_add(n[0] as f64, C2 - p_hi);
        // Only interested in the carry bits of t[0], that's why we are not writing it back to
        // t[0]
        let carry_t0 = (out[0][0].wrapping_add(p_lo.to_bits())) >> 52;
        out[0][1] += p_hi.to_bits() + carry_t0;

        out[1][n.len()] = make_initial(2 * (n.len() - 1 - i), 2 * (n.len() - i));
        let cdp_hi = (c[i] as f64).mul_add(d[0] as f64, C1);
        let cdp_lo = (c[i] as f64).mul_add(d[0] as f64, C2 - cdp_hi);

        out[1][0] = out[1][0].wrapping_add(cdp_lo.to_bits());
        out[1][1] = out[1][1].wrapping_add(cdp_hi.to_bits());
        let mcd = (out[1][0].wrapping_mul(np0) & MASK52) as f64;
        // Outside of the loop because the loop does division by shifting
        let cdp_hi = mcd.mul_add(n[0] as f64, C1);
        let cdp_lo = mcd.mul_add(n[0] as f64, C2 - cdp_hi);
        // Only interested in the carry bits of out[1][0], that's why we are not writing it back to
        // out[1][0]
        let carry_t0 = (out[1][0].wrapping_add(cdp_lo.to_bits())) >> 52;
        out[1][1] += cdp_hi.to_bits() + carry_t0;

        for j in 1..b.len() {
            let ab_hi = (a[i] as f64).mul_add(b[j] as f64, C1);
            let ab_lo = (a[i] as f64).mul_add(b[j] as f64, C2 - ab_hi);
            let mn_hi = m.mul_add(n[j] as f64, C1);
            let mn_lo = m.mul_add(n[j] as f64, C2 - mn_hi);
            out[0][j + 1] = out[0][j + 1].wrapping_add(ab_hi.to_bits() + mn_hi.to_bits());
            out[0][j - 1] = out[0][j].wrapping_add(ab_lo.to_bits() + mn_lo.to_bits());

            let abcd_hi = (c[i] as f64).mul_add(d[j] as f64, C1);
            let abcd_lo = (c[i] as f64).mul_add(d[j] as f64, C2 - abcd_hi);
            let mncd_hi = mcd.mul_add(n[j] as f64, C1);
            let mncd_lo = mcd.mul_add(n[j] as f64, C2 - mncd_hi);
            out[1][j + 1] = out[1][j + 1].wrapping_add(abcd_hi.to_bits() + mncd_hi.to_bits());
            out[1][j - 1] = out[1][j].wrapping_add(abcd_lo.to_bits() + mncd_lo.to_bits());
        }
        out[0][n.len() - 1] = out[0][n.len()];

        out[1][n.len() - 1] = out[1][n.len()];
    }

    resolve_2(out)
}

// FIOS variant of the above cios_opt_sub
// Batch all the subtractions on t[i] together
// Best performing f64 version on the RPi
#[inline(always)]
pub fn fios_opt_sub(a: [u64; 5], b: [u64; 5], n: [u64; 5], np0: u64) -> [u64; 6] {
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
        t[1] += p_hi.to_bits() + carry_t0;

        for j in 1..b.len() {
            let ab_hi = (a[i] as f64).mul_add(b[j] as f64, C1);
            let ab_lo = (a[i] as f64).mul_add(b[j] as f64, C2 - ab_hi);
            let mn_hi = m.mul_add(n[j] as f64, C1);
            let mn_lo = m.mul_add(n[j] as f64, C2 - mn_hi);
            t[j + 1] = t[j + 1].wrapping_add(ab_hi.to_bits() + mn_hi.to_bits());
            t[j - 1] = t[j].wrapping_add(ab_lo.to_bits() + mn_lo.to_bits());
        }
        t[n.len() - 1] = t[n.len()];
    }

    resolve(t)
}

pub fn fios_opt_sub_simd(
    a: [u64; 5],
    b: [u64; 5],
    c: [u64; 5],
    d: [u64; 5],
    n: [u64; 5],
    np0: u64,
) -> [[u64; 6]; 2] {
    let mut t: [Simd<u64, 2>; 6] = [Simd::splat(0); 6];
    for i in 0..t.len() - 1 {
        t[i] = Simd::splat(make_initial(2 + 2 * i, 2 * i))
    }
    let sb0 = Simd::from_array([b[0] as f64, d[0] as f64]);

    for i in 0..a.len() {
        // Does this give optimal assembly?
        // Probably a transpose would improve
        let sai = Simd::from_array([a[i] as f64, c[i] as f64]);
        // a_i * B
        t[n.len()] = Simd::splat(make_initial(2 * (n.len() - 1 - i), 2 * (n.len() - i)));
        // This seems to be a better fit for a scalar add
        let p_hi = sai.mul_add(sb0, Simd::splat(C1));
        let p_lo = sai.mul_add(sb0, Simd::splat(C2) - p_hi);

        t[0] = t[0] + p_lo.to_bits();
        t[1] = t[1] + p_hi.to_bits();
        // This should have been scalar
        let m = (t[0] * Simd::splat(np0)).bitand(Simd::splat(MASK52));
        let m = Simd::from_array([m[0] as f64, m[1] as f64]);
        // Outside of the loop because the loop does division by shifting
        let p_hi = m.mul_add(Simd::splat(n[0] as f64), Simd::splat(C1));
        let p_lo = m.mul_add(Simd::splat(n[0] as f64), Simd::splat(C2) - p_hi);
        // Only interested in the carry bits of t[0], that's why we are not writing it back to
        // t[0]
        let carry_t0 = (t[0] + p_lo.to_bits()) >> 52;
        t[1] += p_hi.to_bits() + carry_t0;

        for j in 1..b.len() {
            let sbj = Simd::from_array([b[j] as f64, d[j] as f64]);
            // Multiplication with scalar
            let ab_hi = sai.mul_add(sbj, Simd::splat(C1));
            let ab_lo = sai.mul_add(sbj, Simd::splat(C2) - ab_hi);
            // Multiplication with scalar
            let mn_hi = m.mul_add(Simd::splat(n[j] as f64), Simd::splat(C1));
            let mn_lo = m.mul_add(Simd::splat(n[j] as f64), Simd::splat(C2) - mn_hi);
            t[j + 1] = t[j + 1] + ab_hi.to_bits() + mn_hi.to_bits();
            t[j - 1] = t[j] + ab_lo.to_bits() + mn_lo.to_bits();
        }
        t[n.len() - 1] = t[n.len()];
    }

    resolve_simd(t)
}

pub fn fios_opt_sub_simd_sat(
    a: [u64; 5],
    b: [u64; 5],
    c: [u64; 5],
    d: [u64; 5],
    e: [u64; 5],
    f: [u64; 5],
    g: [u64; 5],
    h: [u64; 5],
    n: [u64; 5],
    np0: u64,
) -> [[u64; 6]; 4] {
    let mut t: [Simd<u64, 4>; 6] = [Simd::splat(0); 6];
    for i in 0..t.len() - 1 {
        t[i] = Simd::splat(make_initial(2 + 2 * i, 2 * i))
    }
    let sb0 = Simd::from_array([b[0] as f64, d[0] as f64, f[0] as f64, h[0] as f64]);

    for i in 0..a.len() {
        // Does this give optimal assembly?
        // Probably a transpose would improve
        let sai = Simd::from_array([a[i] as f64, c[i] as f64, e[i] as f64, g[i] as f64]);
        // a_i * B
        t[n.len()] = Simd::splat(make_initial(2 * (n.len() - 1 - i), 2 * (n.len() - i)));
        // This seems to be a better fit for a scalar add
        let p_hi = sai.mul_add(sb0, Simd::splat(C1));
        let p_lo = sai.mul_add(sb0, Simd::splat(C2) - p_hi);

        t[0] = t[0] + p_lo.to_bits();
        t[1] = t[1] + p_hi.to_bits();
        // This should have been scalar
        let m = (t[0] * Simd::splat(np0)).bitand(Simd::splat(MASK52));
        let m = Simd::from_array([m[0] as f64, m[1] as f64, m[2] as f64, m[3] as f64]);
        // Outside of the loop because the loop does division by shifting
        let p_hi = m.mul_add(Simd::splat(n[0] as f64), Simd::splat(C1));
        let p_lo = m.mul_add(Simd::splat(n[0] as f64), Simd::splat(C2) - p_hi);
        // Only interested in the carry bits of t[0], that's why we are not writing it back to
        // t[0]
        let carry_t0 = (t[0] + p_lo.to_bits()) >> 52;
        t[1] += p_hi.to_bits() + carry_t0;

        for j in 1..b.len() {
            let sbj = Simd::from_array([b[j] as f64, d[j] as f64, f[j] as f64, h[j] as f64]);
            // Multiplication with scalar
            let ab_hi = sai.mul_add(sbj, Simd::splat(C1));
            let ab_lo = sai.mul_add(sbj, Simd::splat(C2) - ab_hi);
            // Multiplication with scalar
            let mn_hi = m.mul_add(Simd::splat(n[j] as f64), Simd::splat(C1));
            let mn_lo = m.mul_add(Simd::splat(n[j] as f64), Simd::splat(C2) - mn_hi);
            t[j + 1] = t[j + 1] + ab_hi.to_bits() + mn_hi.to_bits();
            t[j - 1] = t[j] + ab_lo.to_bits() + mn_lo.to_bits();
        }
        t[n.len() - 1] = t[n.len()];
    }

    resolve_simd_sat(t)
}

/// FIOS variant with the subtraction optimization
pub fn fios_opt(a: [u64; 5], b: [u64; 5], n: [u64; 5], np0: u64) -> [u64; 6] {
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

        // Only interested in the carry bits of t[0], that's why we are not writing it back to
        // t[0].
        let carry_t0 = (t[0] + p_lo.to_bits() - C3.to_bits()) >> 52;
        t[1] += (p_hi.to_bits() - C1.to_bits()) + carry_t0;

        for j in 1..b.len() {
            let ab_hi = (a[i] as f64).mul_add(b[j] as f64, C1);
            let ab_lo = (a[i] as f64).mul_add(b[j] as f64, C2 - ab_hi);
            let mn_hi = m.mul_add(n[j] as f64, C1);
            let mn_lo = m.mul_add(n[j] as f64, C2 - mn_hi);

            t[j + 1] += ab_hi.to_bits() + mn_hi.to_bits() - 2 * C1.to_bits();
            t[j - 1] = t[j] + ab_lo.to_bits() + mn_lo.to_bits() - 2 * C3.to_bits();
        }
        t[n.len() - 1] = t[n.len()];
        t[n.len()] = 0;
    }

    resolve(t)
}

const fn pow_2(n: u32) -> f64 {
    // Unfortunately we can't use f64::powi in const fn yet
    // This is a workaround that creates the bit pattern directly
    let exp = ((n as u64 + 1023) & 0x7FF) << 52;
    f64::from_bits(exp)
}

const C1: f64 = pow_2(104); // 2.0^104
const C2: f64 = pow_2(104) + pow_2(52); // 2.0^104 + 2.0^52
const C3: f64 = pow_2(52); // 2.0^52

#[inline]
fn make_initial(low_count: usize, high_count: usize) -> u64 {
    let val = high_count * 0x467 + low_count * 0x433;
    -((val as i64 & 0xFFF) << 52) as u64
}

#[cfg(test)]
mod tests {
    use crate::emmart::modulus_u52;
    use crate::emmart::subtraction_step_u52;
    use crate::gen::U256b52;
    use crate::U52_NP0;
    use crate::U52_P;
    use crate::U52_R2;

    use super::set_round_to_zero;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn cios_f64_sub_round(a: U256b52) -> bool {
        set_round_to_zero();
        let a_tilde = super::cios_opt_sub(a.0, U52_R2, U52_P, U52_NP0);
        let a_round = super::cios_opt_sub(
            a_tilde[..5].try_into().unwrap(),
            [1, 0, 0, 0, 0],
            U52_P,
            U52_NP0,
        );

        modulus_u52(a.0, U52_P) == subtraction_step_u52(a_round[..5].try_into().unwrap(), U52_P)
    }

    #[quickcheck]
    fn fios_f64_sub_round(a: U256b52) -> bool {
        set_round_to_zero();
        let a_tilde = super::fios_opt_sub(a.0, U52_R2, U52_P, U52_NP0);
        let a_round = super::fios_opt_sub(
            a_tilde[..5].try_into().unwrap(),
            [1, 0, 0, 0, 0],
            U52_P,
            U52_NP0,
        );

        modulus_u52(a.0, U52_P) == subtraction_step_u52(a_round[..5].try_into().unwrap(), U52_P)
    }

    #[quickcheck]
    fn fios_f64_sub_sat_round(a: U256b52, b: U256b52) -> bool {
        set_round_to_zero();
        let a_tilde = super::fios_opt_sub_sat(a.0, U52_R2, b.0, U52_R2, U52_P, U52_NP0);
        let a_round = super::fios_opt_sub_sat(
            a_tilde[0][..5].try_into().unwrap(),
            [1, 0, 0, 0, 0],
            a_tilde[1][..5].try_into().unwrap(),
            [1, 0, 0, 0, 0],
            U52_P,
            U52_NP0,
        );

        modulus_u52(a.0, U52_P) == subtraction_step_u52(a_round[0][..5].try_into().unwrap(), U52_P)
            && modulus_u52(b.0, U52_P)
                == subtraction_step_u52(a_round[1][..5].try_into().unwrap(), U52_P)
    }

    #[quickcheck]
    fn fios_f64_sub_simd_round(a: U256b52, b: U256b52) -> bool {
        set_round_to_zero();
        let a_tilde = super::fios_opt_sub_simd(a.0, U52_R2, b.0, U52_R2, U52_P, U52_NP0);
        let a_round = super::fios_opt_sub_simd(
            a_tilde[0][..5].try_into().unwrap(),
            [1, 0, 0, 0, 0],
            a_tilde[1][..5].try_into().unwrap(),
            [1, 0, 0, 0, 0],
            U52_P,
            U52_NP0,
        );

        modulus_u52(a.0, U52_P) == subtraction_step_u52(a_round[0][..5].try_into().unwrap(), U52_P)
            && modulus_u52(b.0, U52_P)
                == subtraction_step_u52(a_round[1][..5].try_into().unwrap(), U52_P)
    }

    #[quickcheck]
    fn fios_f64_sub_simd_sat_round(a: U256b52, b: U256b52, c: U256b52, d: U256b52) -> bool {
        set_round_to_zero();
        let a_tilde = super::fios_opt_sub_simd_sat(
            a.0, U52_R2, b.0, U52_R2, c.0, U52_R2, d.0, U52_R2, U52_P, U52_NP0,
        );
        let a_round = super::fios_opt_sub_simd_sat(
            a_tilde[0][..5].try_into().unwrap(),
            [1, 0, 0, 0, 0],
            a_tilde[1][..5].try_into().unwrap(),
            [1, 0, 0, 0, 0],
            a_tilde[2][..5].try_into().unwrap(),
            [1, 0, 0, 0, 0],
            a_tilde[3][..5].try_into().unwrap(),
            [1, 0, 0, 0, 0],
            U52_P,
            U52_NP0,
        );

        modulus_u52(a.0, U52_P) == subtraction_step_u52(a_round[0][..5].try_into().unwrap(), U52_P)
            && modulus_u52(b.0, U52_P)
                == subtraction_step_u52(a_round[1][..5].try_into().unwrap(), U52_P)
            && modulus_u52(c.0, U52_P)
                == subtraction_step_u52(a_round[2][..5].try_into().unwrap(), U52_P)
            && modulus_u52(d.0, U52_P)
                == subtraction_step_u52(a_round[3][..5].try_into().unwrap(), U52_P)
    }

    #[quickcheck]
    fn fios_f64_round(a: U256b52) -> bool {
        set_round_to_zero();
        let a_tilde = super::fios_opt(a.0, U52_R2, U52_P, U52_NP0);
        let a_round = super::fios_opt(
            a_tilde[..5].try_into().unwrap(),
            [1, 0, 0, 0, 0],
            U52_P,
            U52_NP0,
        );

        modulus_u52(a.0, U52_P) == subtraction_step_u52(a_round[..5].try_into().unwrap(), U52_P)
    }

    #[quickcheck]
    fn cios_f64_round(a: U256b52) -> bool {
        set_round_to_zero();
        let a_tilde = super::cios_opt(a.0, U52_R2, U52_P, U52_NP0);
        let a_round = super::cios_opt(
            a_tilde[..5].try_into().unwrap(),
            [1, 0, 0, 0, 0],
            U52_P,
            U52_NP0,
        );

        modulus_u52(a.0, U52_P) == subtraction_step_u52(a_round[..5].try_into().unwrap(), U52_P)
    }
}
