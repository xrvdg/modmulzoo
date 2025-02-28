use std::{
    arch::aarch64::vcvtq_f64_u64,
    ops::BitAnd,
    simd::{num::SimdFloat, Simd, StdFloat},
};

use crate::{
    emmart::{self, make_initial, MASK52},
    U52_NP0, U52_P,
};

const RHO_1: [u64; 5] = [
    0x82e644ee4c3d2,
    0xf93893c98b1de,
    0xd46fe04d0a4c7,
    0x8f0aad55e2a1f,
    0x005ed0447de83,
];

const RHO_2: [u64; 5] = [
    0x74eccce9a797a,
    0x16ddcc30bd8a4,
    0x49ecd3539499e,
    0xb23a6fcc592b8,
    0x00e3bd49f6ee5,
];

const RHO_3: [u64; 5] = [
    0x0E8C656567D77,
    0x430D05713AE61,
    0xEA3BA6B167128,
    0xA7DAE55C5A296,
    0x01B4AFD513572,
];

const RHO_4: [u64; 5] = [
    0x22E2400E2F27D,
    0x323B46EA19686,
    0xE6C43F0DF672D,
    0x7824014C39E8B,
    0x00C6B48AFE1B8,
];

#[inline(always)]
fn mult(a: u64, b: u64) -> (u64, u64) {
    let p_hi = (a as f64).mul_add(b as f64, emmart::C1);
    let p_lo = (a as f64).mul_add(b as f64, emmart::C2 - p_hi);
    // Maybe we can keep it in float?
    (p_lo.to_bits(), p_hi.to_bits())
}

#[inline(always)]
pub fn vmult(a: [u64; 5], b: [u64; 5]) -> [u64; 10] {
    let mut t = [0; 10];

    for i in 0..5 {
        t[i] = make_initial(i + 1, i);
        t[10 - 1 - i] = make_initial(i, i + 1);
    }

    for i in 0..a.len() {
        for j in 0..b.len() {
            let p_hi = (a[i] as f64).mul_add(b[j] as f64, emmart::C1);
            let p_lo = (a[i] as f64).mul_add(b[j] as f64, emmart::C2 - p_hi);
            // OPTIMIZATION: can be vectorized
            t[i + j + 1] = t[i + j + 1].wrapping_add(p_hi.to_bits());
            t[i + j] = t[i + j].wrapping_add(p_lo.to_bits());
        }
    }

    t
}

#[inline(always)]
pub fn vmultadd_noinit(a: [u64; 5], b: [u64; 5], mut t: [u64; 10]) -> [u64; 10] {
    for i in 0..a.len() {
        for j in 0..b.len() {
            let p_hi = (a[i] as f64).mul_add(b[j] as f64, emmart::C1);
            let p_lo = (a[i] as f64).mul_add(b[j] as f64, emmart::C2 - p_hi);
            // OPTIMIZATION: can be vectorized
            t[i + j + 1] = t[i + j + 1].wrapping_add(p_hi.to_bits());
            t[i + j] = t[i + j].wrapping_add(p_lo.to_bits());
        }
    }

    t
}

#[inline(always)]
pub fn vmultadd_noinit_simd(
    a: [[u64; 5]; 2],
    b: [[u64; 5]; 2],
    mut t: [Simd<u64, 2>; 10],
) -> [Simd<u64, 2>; 10] {
    for i in 0..5 {
        let avi = Simd::from_array([a[0][i] as f64, a[1][i] as f64]);
        for j in 0..5 {
            // Maybe use the vector convert
            let bvj = Simd::from_array([b[0][j] as f64, b[1][j] as f64]);
            let p_hi = (avi).mul_add(bvj, Simd::splat(emmart::C1));
            let p_lo = (avi).mul_add(bvj, Simd::splat(emmart::C2) - p_hi);
            // OPTIMIZATION: can be vectorized
            t[i + j + 1] = t[i + j + 1] + p_hi.to_bits();
            t[i + j] = t[i + j] + p_lo.to_bits();
        }
    }

    t
}

const fn heaviside(x: isize) -> usize {
    (x >= 0) as usize
}

#[inline(always)]
fn smult(s: u64, v: [u64; 5]) -> [u64; 6] {
    let mut t: [u64; 6] = [0; 6];

    // This should be combined with the vmult in the algorithm
    t[0] = emmart::make_initial(1, 0);
    for i in 1..t.len() - 1 {
        t[i] = emmart::make_initial(1, 1)
    }
    t[5] = emmart::make_initial(0, 1);

    for i in 0..v.len() {
        let (sum, carry) = mult(s, v[i]);
        t[i] = t[i].wrapping_add(sum);
        t[i + 1] = t[i + 1].wrapping_add(carry);
    }
    t
}

#[inline(always)]
fn smult_noinit(s: u64, v: [u64; 5]) -> [u64; 6] {
    let mut t: [u64; 6] = [0; 6];

    for i in 0..v.len() {
        let (sum, carry) = mult(s, v[i]);
        t[i] = t[i].wrapping_add(sum);
        t[i + 1] = t[i + 1].wrapping_add(carry);
    }
    t
}

#[inline(always)]
fn smult_noinit_simd(s: Simd<u64, 2>, v: [u64; 5]) -> [Simd<u64, 2>; 6] {
    let mut t = [Simd::splat(0); 6];
    let s: Simd<f64, 2> = unsafe { vcvtq_f64_u64(s.into()).into() };

    for i in 0..v.len() {
        // Maybe use the vector convert
        let p_hi = s.mul_add(Simd::splat(v[i] as f64), Simd::splat(emmart::C1));
        let p_lo = s.mul_add(Simd::splat(v[i] as f64), Simd::splat(emmart::C2) - p_hi);
        // OPTIMIZATION: can be vectorized
        t[i + 1] = t[i + 1] + p_hi.to_bits();
        t[i] = t[i] + p_lo.to_bits();
    }

    t
}

#[inline(always)]
fn addv<const N: usize>(mut va: [u64; N], vb: [u64; N]) -> [u64; N] {
    for i in 0..va.len() {
        va[i] = va[i].wrapping_add(vb[i]);
    }
    va
}

#[inline(always)]
fn addv_simd<const N: usize>(
    mut va: [Simd<u64, 2>; N],
    vb: [Simd<u64, 2>; N],
) -> [Simd<u64, 2>; N] {
    for i in 0..va.len() {
        va[i] = va[i] + vb[i];
    }
    va
}

pub fn parallel_ref(a: [u64; 5], b: [u64; 5]) -> [u64; 5] {
    // Continuation can happen after the first three rounds
    // That could be a way of describing it, but it will likely create anonymous functions what we don't want
    // However that could be a way to write the algorithm and then let the code be generated from there.
    let mut t = vmult(a, b);
    // println!("t: {t:?}");

    // combining the initials might not even have a benefit in it's current form. The first add would otherwise
    // Still might save another combination but it requires stringing the accumulator value. Is that something
    // desirable or undesirable? If we don't the multiplications can start earlier

    // Can be a loop, or use seq to unroll it
    t[1] += t[0] >> 52;
    t[2] += t[1] >> 52;
    t[3] += t[2] >> 52;
    t[4] += t[3] >> 52;
    // These multiplications can be interleaved, each step is independ
    let r0 = smult(t[0] & MASK52, RHO_4);
    let r1 = smult(t[1] & MASK52, RHO_3);
    let r2 = smult(t[2] & MASK52, RHO_2);
    let r3 = smult(t[3] & MASK52, RHO_1);

    let s: [u64; 6] = t[4..].try_into().unwrap();
    // These additions can pause after the first one has given the result to start multiplying.
    // but for the floating point it doesn't matter that much as the addition is done on the same pipe
    let s = addv(r3, addv(addv(s, r0), addv(r1, r2)));

    let m = s[0].wrapping_mul(U52_NP0) & MASK52;
    emmart::resolve(addv(s, smult(m, U52_P)))[1..]
        .try_into()
        .unwrap()
    // Could resolve it here, but can also delay the resolving if it stays then instead of max number of addition in algo it will be max number + first step
    // or for certainity 2x number of additions in algo
}

// Performs a lot better on MacOS (22ns vs 28 ns) but loses 2-3 ns on the Raspberry Pi compared to parallel_ref
pub fn parallel_sub(a: [u64; 5], b: [u64; 5]) -> [u64; 5] {
    // Continuation can happen after the first three rounds
    // That could be a way of describing it, but it will likely create anonymous functions what we don't want
    let mut t: [u64; 10] = [0; 10];
    // vmult
    for i in 0..5 {
        t[i] = make_initial(i + 1 + 5 * heaviside(i as isize - 4), i);
        let j = 10 - 1 - i;
        t[j] = make_initial(i + 5 * (1 - heaviside(j as isize - 9)), i + 1 + 5 * 1);
    }

    let mut t = vmultadd_noinit(a, b, t);

    // println!("t: {t:?}");

    // combining the initials might not even have a benefit in it's current form. The first add would otherwise
    // Still might save another combination but it requires stringing the accumulator value. Is that something
    // desirable or undesirable? If we don't the multiplications can start earlier

    // Can be a loop, or use seq to unroll it
    t[1] += t[0] >> 52;
    t[2] += t[1] >> 52;
    t[3] += t[2] >> 52;
    t[4] += t[3] >> 52;
    // These multiplications can be interleaved, each step is independ
    let r0 = smult_noinit(t[0] & MASK52, RHO_4);
    let r1 = smult_noinit(t[1] & MASK52, RHO_3);
    let r2 = smult_noinit(t[2] & MASK52, RHO_2);
    let r3 = smult_noinit(t[3] & MASK52, RHO_1);

    let s: [u64; 6] = t[4..].try_into().unwrap();
    // These additions can pause after the first one has given the result to start multiplying.
    // but for the floating point it doesn't matter that much as the addition is done on the same pipe
    let s = addv(r3, addv(addv(s, r0), addv(r1, r2)));

    let m = s[0].wrapping_mul(U52_NP0) & MASK52;
    emmart::resolve(addv(s, smult_noinit(m, U52_P)))[1..]
        .try_into()
        .unwrap()
    // Could resolve it here, but can also delay the resolving if it stays then instead of max number of addition in algo it will be max number + first step
    // or for certainity 2x number of additions in algo
}

// Performs a lot better on MacOS (22ns vs 28 ns) but loses 2-3 ns on the Raspberry Pi compared to parallel_ref
pub fn parallel_simd_sub(a: [[u64; 5]; 2], b: [[u64; 5]; 2]) -> [[u64; 5]; 2] {
    // Continuation can happen after the first three rounds
    // That could be a way of describing it, but it will likely create anonymous functions what we don't want
    let mut t: [Simd<u64, 2>; 10] = [Simd::splat(0); 10];
    // vmult
    for i in 0..5 {
        t[i] = Simd::splat(make_initial(i + 1 + 5 * heaviside(i as isize - 4), i));
        let j = 10 - 1 - i;
        t[j] = Simd::splat(make_initial(
            i + 5 * (1 - heaviside(j as isize - 9)),
            i + 1 + 5 * 1,
        ));
    }

    let mut t = vmultadd_noinit_simd(a, b, t);

    // println!("t: {t:?}");

    // combining the initials might not even have a benefit in it's current form. The first add would otherwise
    // Still might save another combination but it requires stringing the accumulator value. Is that something
    // desirable or undesirable? If we don't the multiplications can start earlier

    // Can be a loop, or use seq to unroll it
    t[1] += t[0] >> 52;
    t[2] += t[1] >> 52;
    t[3] += t[2] >> 52;
    t[4] += t[3] >> 52;
    // These multiplications can be interleaved, each step is independ
    let r0 = smult_noinit_simd(t[0].bitand(Simd::splat(MASK52)), RHO_4);
    let r1 = smult_noinit_simd(t[1].bitand(Simd::splat(MASK52)), RHO_3);
    let r2 = smult_noinit_simd(t[2].bitand(Simd::splat(MASK52)), RHO_2);
    let r3 = smult_noinit_simd(t[3].bitand(Simd::splat(MASK52)), RHO_1);

    let s: [Simd<u64, 2>; 6] = t[4..].try_into().unwrap();
    // These additions can pause after the first one has given the result to start multiplying.
    // but for the floating point it doesn't matter that much as the addition is done on the same pipe
    let s = addv_simd(r3, addv_simd(addv_simd(s, r0), addv_simd(r1, r2)));

    let m = (s[0] * Simd::splat(U52_NP0)).bitand(Simd::splat(MASK52));
    // This needs to be optimized in two ways, The final resolve_simd should be combined with addition
    let mp = smult_noinit_simd(m, U52_P);
    resolve_simd_add_truncate(s, mp)
    // Could resolve it here, but can also delay the resolving if it stays then instead of max number of addition in algo it will be max number + first step
    // or for certainity 2x number of additions in algo
}

#[inline(always)]
pub fn resolve_simd_add_truncate(s: [Simd<u64, 2>; 6], mp: [Simd<u64, 2>; 6]) -> [[u64; 5]; 2] {
    {
        let t = addv_simd(s, mp);
        let mut out = [[0; 5]; 2];
        let mut carry = t[0] >> 52;
        for i in 0..5 {
            let tmp = t[i + 1] + carry;
            [out[0][i], out[1][i]] = tmp.bitand(Simd::splat(MASK52)).to_array();
            carry = tmp >> 52;
        }
        out
    }
}
#[cfg(test)]
mod tests {
    use std::{hint::black_box, simd::Simd};

    use crate::{
        emmart::{modulus_u52, set_round_to_zero, subtraction_step_u52},
        gen::U256b52,
        U52_P, U52_R2,
    };
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn parallel_round(a: U256b52) {
        set_round_to_zero();
        let a_tilde = super::parallel_ref(a.0, U52_R2);
        let a_round = super::parallel_ref(a_tilde, [1, 0, 0, 0, 0]);

        assert_eq!(modulus_u52(a.0, U52_P), modulus_u52(a_round, U52_P))
    }

    #[quickcheck]
    fn parallel_sub_round(a: U256b52) {
        set_round_to_zero();
        let a_tilde = super::parallel_sub(a.0, U52_R2);
        let a_round = super::parallel_sub(a_tilde, [1, 0, 0, 0, 0]);

        assert_eq!(modulus_u52(a.0, U52_P), modulus_u52(a_round, U52_P))
    }

    #[quickcheck]
    fn parallel_sub_simd_round(a: U256b52, b: U256b52) {
        set_round_to_zero();
        let a_arrays = [a.0, b.0];
        let r2_arrays = [U52_R2, U52_R2];
        let a_tilde = super::parallel_simd_sub(a_arrays, r2_arrays);

        let ones_arrays = [[1, 0, 0, 0, 0], [1, 0, 0, 0, 0]];
        let a_round = super::parallel_simd_sub(a_tilde, ones_arrays);

        assert_eq!(modulus_u52(a.0, U52_P), modulus_u52(a_round[0], U52_P));
        assert_eq!(modulus_u52(b.0, U52_P), modulus_u52(a_round[1], U52_P));
    }
}
