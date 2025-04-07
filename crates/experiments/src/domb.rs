use std::{
    arch::aarch64::vcvtq_f64_u64,
    array,
    ops::BitAnd,
    simd::{
        cmp::SimdPartialEq,
        num::{SimdFloat, SimdInt, SimdUint},
        Simd, StdFloat,
    },
};

use seq_macro::seq;

use crate::emmart::{self, make_initial};
use block_multiplier::{
    constants::{MASK52, U52_2P, U52_NP0, U52_P},
    rtz::RTZ,
};

use block_multiplier::subarray;

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
            t[i + j + 1] = t[i + j + 1].wrapping_add(p_hi.to_bits());
            t[i + j] = t[i + j].wrapping_add(p_lo.to_bits());
        }
    }

    t
}

#[inline(always)]
pub fn vmultadd_noinit(a: [u64; 5], b: [u64; 5], mut t: [u64; 10]) -> [u64; 10] {
    seq!(i in 0..5 {
        seq!(j in 0..5 {
            let p_hi = (a[i] as f64).mul_add(b[j] as f64, emmart::C1);
            let p_lo = (a[i] as f64).mul_add(b[j] as f64, emmart::C2 - p_hi);
            t[i + j + 1] = t[i + j + 1].wrapping_add(p_hi.to_bits());
            t[i + j] = t[i + j].wrapping_add(p_lo.to_bits());
        });
    });

    t
}

#[inline(always)]
pub fn trans_vmultadd_noinit_simd(
    a: [[u64; 5]; 2],
    b: [[u64; 5]; 2],
    mut t: [Simd<u64, 2>; 10],
) -> [Simd<u64, 2>; 10] {
    // Manually unrolling these loop does not result in any performance increase
    seq!( i in 0..5 {
        let avi = Simd::from_array([a[0][i] as f64, a[1][i] as f64]);
        seq!(j in 0..5 {
            let bvj = Simd::from_array([b[0][j] as f64, b[1][j] as f64]);
            let p_hi = (avi).mul_add(bvj, Simd::splat(emmart::C1));
            let p_lo = (avi).mul_add(bvj, Simd::splat(emmart::C2) - p_hi);
            t[i + j + 1] += p_hi.to_bits();
            t[i + j] += p_lo.to_bits();
        });
    });

    t
}

#[inline(always)]
pub fn vmultadd_noinit_simd(
    a: [Simd<u64, 2>; 5],
    b: [Simd<u64, 2>; 5],
    mut t: [Simd<u64, 2>; 10],
) -> [Simd<u64, 2>; 10] {
    // Manually unrolling these loop does not result in any performance increase
    seq!( i in 0..5 {
        let avi: Simd<f64, 2> = unsafe { vcvtq_f64_u64(a[i].into()).into() };
        seq!(j in 0..5 {
            let bvj: Simd<f64, 2> = unsafe { vcvtq_f64_u64(b[j].into()).into() };
            let p_hi = (avi).mul_add(bvj, Simd::splat(emmart::C1));
            let p_lo = (avi).mul_add(bvj, Simd::splat(emmart::C2) - p_hi);
            t[i + j + 1] += p_hi.to_bits();
            t[i + j] += p_lo.to_bits();
        });
    });

    t
}

pub const fn heaviside(x: isize) -> usize {
    (x >= 0) as usize
}

#[inline(always)]
fn smult(s: u64, v: [u64; 5]) -> [u64; 6] {
    let mut t: [u64; 6] = [0; 6];

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

#[inline(never)]
pub fn smult_noinit_simd_stub(s: Simd<u64, 2>, v: [u64; 5]) -> [Simd<u64, 2>; 6] {
    smult_noinit_simd(s, v)
}

#[inline(always)]
fn smult_noinit_simd(s: Simd<u64, 2>, v: [u64; 5]) -> [Simd<u64, 2>; 6] {
    let mut t = [Simd::splat(0); 6];
    let s: Simd<f64, 2> = unsafe { vcvtq_f64_u64(s.into()).into() };

    for i in 0..v.len() {
        let p_hi = s.mul_add(Simd::splat(v[i] as f64), Simd::splat(emmart::C1));
        let p_lo = s.mul_add(Simd::splat(v[i] as f64), Simd::splat(emmart::C2) - p_hi);
        t[i + 1] += p_hi.to_bits();
        t[i] += p_lo.to_bits();
    }

    t
}

#[inline(always)]
/// Wrapping addition
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
        va[i] += vb[i];
    }
    va
}

pub fn parallel_ref(_rtz: &RTZ, a: [u64; 5], b: [u64; 5]) -> [u64; 5] {
    // The rest of the algorithm can start afeter the first 4 rounds
    let mut t = vmult(a, b);

    t[1] += t[0] >> 52;
    t[2] += t[1] >> 52;
    t[3] += t[2] >> 52;
    t[4] += t[3] >> 52;
    let r0 = smult(t[0] & MASK52, RHO_4);
    let r1 = smult(t[1] & MASK52, RHO_3);
    let r2 = smult(t[2] & MASK52, RHO_2);
    let r3 = smult(t[3] & MASK52, RHO_1);

    let s = subarray!(t, 4, 6);
    let s = addv(r3, addv(addv(s, r0), addv(r1, r2)));

    let m = s[0].wrapping_mul(U52_NP0) & MASK52;
    let resolved = emmart::resolve(addv(s, smult(m, U52_P)));
    subarray!(resolved, 1, 5)
}

pub fn parallel_sub_stub(rtz: &RTZ, a: [u64; 5], b: [u64; 5]) -> [u64; 5] {
    parallel_sub(rtz, a, b)
}

#[inline(always)]
pub fn u256_to_u260_shl2(limbs: [u64; 4]) -> [u64; 5] {
    let [l0, l1, l2, l3] = limbs;

    [
        (l0 << 2) & MASK52,
        ((l0 >> 50) | (l1 << 14)) & MASK52,
        ((l1 >> 38) | (l2 << 26)) & MASK52,
        ((l2 >> 26) | (l3 << 38)) & MASK52,
        l3 >> 14,
    ]
}

pub fn u256_to_u260_shl2_simd_stub(limbs: [Simd<u64, 2>; 4]) -> [Simd<u64, 2>; 5] {
    u256_to_u260_shl2_simd(limbs)
}

#[inline(always)]
pub fn u256_to_u260_shl2_simd(limbs: [Simd<u64, 2>; 4]) -> [Simd<u64, 2>; 5] {
    let [l0, l1, l2, l3] = limbs;
    [
        (l0 << 2) & Simd::splat(MASK52),
        ((l0 >> 50) | (l1 << 14)) & Simd::splat(MASK52),
        ((l1 >> 38) | (l2 << 26)) & Simd::splat(MASK52),
        ((l2 >> 26) | (l3 << 38)) & Simd::splat(MASK52),
        l3 >> 14,
    ]
}

#[inline(always)]
fn u260_to_u256(limbs: [u64; 5]) -> [u64; 4] {
    let [l0, l1, l2, l3, l4] = limbs;
    [
        l0 | (l1 << 52),
        ((l1 >> 12) | (l2 << 40)),
        ((l2 >> 24) | (l3 << 28)),
        ((l3 >> 36) | (l4 << 16)),
    ]
}

// THis can probably be combined and monomorphised, but that would require an into most likely
#[inline(always)]
pub fn u260_to_u256_simd(limbs: [Simd<u64, 2>; 5]) -> [Simd<u64, 2>; 4] {
    let [l0, l1, l2, l3, l4] = limbs;
    [
        l0 | (l1 << 52),
        ((l1 >> 12) | (l2 << 40)),
        ((l2 >> 24) | (l3 << 28)),
        ((l3 >> 36) | (l4 << 16)),
    ]
}

#[inline(never)]
pub fn u260_to_u256_simd_stub(limbs: [Simd<u64, 2>; 5]) -> [Simd<u64, 2>; 4] {
    u260_to_u256_simd(limbs)
}

#[inline(always)]
pub fn parallel_sub_r256(rtz: &RTZ, a: [u64; 4], b: [u64; 4]) -> [u64; 4] {
    let a = u256_to_u260_shl2(a);
    let b = u256_to_u260_shl2(b);
    let res = u260_to_u256(parallel_sub(rtz, a, b));
    res
}

#[inline(always)]
/// Resolve the carry bits in the upper parts 12b and reduce the result to within < 3p
///
/// Note: constant time as it always performs the reduction
pub fn reduce_ct(red: [u64; 6]) -> [u64; 5] {
    // The lowest limb contains carries that still need to be applied.
    let mut borrow = (red[0] >> 52) as i64;
    let a = subarray!(red, 1, 5);

    let b = if (red[5] >> 47) & 1 == 0 {
        [0; 5]
    } else {
        U52_2P
    };

    let mut c = [0; 5];
    for i in 0..c.len() {
        let tmp = a[i] as i64 - b[i] as i64 + borrow;
        c[i] = (tmp as u64) & MASK52;
        borrow = tmp >> 52
    }

    c
}

#[inline(always)]
/// Resolve the carry bits in the upper parts 12b and reduce the result to within < 3p
pub fn reduce_ct_simd(red: [Simd<u64, 2>; 6]) -> [Simd<u64, 2>; 5] {
    // The lowest limb contains carries that still need to be applied.
    let mut borrow: Simd<i64, 2> = (red[0] >> 52).cast();
    let a = [red[1], red[2], red[3], red[4], red[5]];

    // To reduce Check whether the most significant bit is set
    let mask = (a[4] >> 47).bitand(Simd::splat(1)).simd_eq(Simd::splat(0));

    // Select values based on the mask: if mask lane is true, use zeros, else use U52_2P
    let zeros = [Simd::splat(0); 5];
    let twop = U52_2P.map(|pi| Simd::splat(pi));
    let b: [_; 5] = array::from_fn(|i| mask.select(zeros[i], twop[i]));

    let mut c = [Simd::splat(0); 5];
    for i in 0..c.len() {
        let tmp: Simd<i64, 2> = a[i].cast::<i64>() - b[i].cast() + borrow;
        c[i] = tmp.cast().bitand(Simd::splat(MASK52));
        borrow = tmp >> 52
    }

    c
}

#[inline(never)]
pub fn reduce_stub(red: [u64; 6]) -> [u64; 5] {
    reduce_ct(red)
}

#[inline(never)]
pub fn reduce_ct_simd_stub(red: [Simd<u64, 2>; 6]) -> [Simd<u64, 2>; 5] {
    reduce_ct_simd(red)
}

// Performs a lot better on MacOS (22ns vs 28 ns) but loses 2-3 ns on the Raspberry Pi compared to parallel_ref
#[inline(always)]
pub fn parallel_sub(_rtz: &RTZ, a: [u64; 5], b: [u64; 5]) -> [u64; 5] {
    let mut t: [u64; 10] = [0; 10];
    for i in 0..5 {
        t[i] = make_initial(i + 1 + 5 * heaviside(i as isize - 4), i);
        let j = 10 - 1 - i;
        t[j] = make_initial(i + 5 * (1 - heaviside(j as isize - 9)), i + 1 + 5 * 1);
    }

    let mut t = vmultadd_noinit(a, b, t);

    t[1] += t[0] >> 52;
    t[2] += t[1] >> 52;
    t[3] += t[2] >> 52;
    t[4] += t[3] >> 52;
    // These multiplications can be interleaved, each step is independent
    let r0 = smult_noinit(t[0] & MASK52, RHO_4);
    let r1 = smult_noinit(t[1] & MASK52, RHO_3);
    let r2 = smult_noinit(t[2] & MASK52, RHO_2);
    let r3 = smult_noinit(t[3] & MASK52, RHO_1);

    let s = subarray!(t, 4, 6);
    let s = addv(r3, addv(addv(s, r0), addv(r1, r2)));

    let m = s[0].wrapping_mul(U52_NP0) & MASK52;
    // Note: using a condition reduction here results in the same assembly
    // as the constant time one.
    reduce_ct(addv(s, smult_noinit(m, U52_P)))
}

pub fn parallel_sub_simd_r256(_rtz: &RTZ, a: [[u64; 4]; 2], b: [[u64; 4]; 2]) -> [[u64; 4]; 2] {
    let a = u256_to_u260_shl2_simd(transpose_u256_to_simd(a));
    let b = u256_to_u260_shl2_simd(transpose_u256_to_simd(b));

    let mut t: [Simd<u64, 2>; 10] = [Simd::splat(0); 10];
    for i in 0..5 {
        t[i] = Simd::splat(make_initial(i + 1 + 5 * heaviside(i as isize - 4), i));
        let j = 10 - 1 - i;
        t[j] = Simd::splat(make_initial(
            i + 5 * (1 - heaviside(j as isize - 9)),
            i + 1 + 5 * 1,
        ));
    }

    let mut t = vmultadd_noinit_simd(a, b, t);

    t[1] += t[0] >> 52;
    t[2] += t[1] >> 52;
    t[3] += t[2] >> 52;
    t[4] += t[3] >> 52;
    // These multiplications can be interleaved, each step is independ
    let r0 = smult_noinit_simd(t[0].bitand(Simd::splat(MASK52)), RHO_4);
    let r1 = smult_noinit_simd(t[1].bitand(Simd::splat(MASK52)), RHO_3);
    let r2 = smult_noinit_simd(t[2].bitand(Simd::splat(MASK52)), RHO_2);
    let r3 = smult_noinit_simd(t[3].bitand(Simd::splat(MASK52)), RHO_1);

    let s = [t[4], t[5], t[6], t[7], t[8], t[9]];
    // This can also be a fiveway-add in a loop, but I think the compiler already takes care of this.
    let s = addv_simd(r3, addv_simd(addv_simd(s, r0), addv_simd(r1, r2)));

    let m = (s[0] * Simd::splat(U52_NP0)).bitand(Simd::splat(MASK52));
    let mp = smult_noinit_simd(m, U52_P);

    let reduced = reduce_ct_simd(addv_simd(s, mp));

    let u256_result = u260_to_u256_simd(reduced);
    let res = transpose_simd_to_u256(u256_result);

    res
}

pub fn parallel_simd_sub(_rtz: &RTZ, a: [[u64; 5]; 2], b: [[u64; 5]; 2]) -> [[u64; 5]; 2] {
    let mut t: [Simd<u64, 2>; 10] = [Simd::splat(0); 10];
    for i in 0..5 {
        t[i] = Simd::splat(make_initial(i + 1 + 5 * heaviside(i as isize - 4), i));
        let j = 10 - 1 - i;
        t[j] = Simd::splat(make_initial(
            i + 5 * (1 - heaviside(j as isize - 9)),
            i + 1 + 5 * 1,
        ));
    }

    let mut t = trans_vmultadd_noinit_simd(a, b, t);

    t[1] += t[0] >> 52;
    t[2] += t[1] >> 52;
    t[3] += t[2] >> 52;
    t[4] += t[3] >> 52;
    // These multiplications can be interleaved, each step is independ
    let r0 = smult_noinit_simd(t[0].bitand(Simd::splat(MASK52)), RHO_4);
    let r1 = smult_noinit_simd(t[1].bitand(Simd::splat(MASK52)), RHO_3);
    let r2 = smult_noinit_simd(t[2].bitand(Simd::splat(MASK52)), RHO_2);
    let r3 = smult_noinit_simd(t[3].bitand(Simd::splat(MASK52)), RHO_1);

    let s = [t[4], t[5], t[6], t[7], t[8], t[9]];
    // This can also be a fiveway-add in a loop, but I think the compiler already takes care of this.
    let s = addv_simd(r3, addv_simd(addv_simd(s, r0), addv_simd(r1, r2)));

    let m = (s[0] * Simd::splat(U52_NP0)).bitand(Simd::splat(MASK52));
    let mp = smult_noinit_simd(m, U52_P);
    resolve_simd_add_truncate_trans(s, mp)
}

// Has no performance improvements over resolve_simd(addv_simd) and then manually truncating it.
#[inline(always)]
pub fn resolve_simd_add_truncate_trans(
    s: [Simd<u64, 2>; 6],
    mp: [Simd<u64, 2>; 6],
) -> [[u64; 5]; 2] {
    let mut out = [[0; 5]; 2];
    let mut carry = (s[0] + mp[0]) >> 52;
    for i in 0..5 {
        let tmp = s[i + 1] + mp[i + 1] + carry;
        [out[0][i], out[1][i]] = tmp.bitand(Simd::splat(MASK52)).to_array();
        carry = tmp >> 52;
    }
    out
}

#[inline(always)]
pub fn resolve_simd_add_truncate(s: [Simd<u64, 2>; 6], mp: [Simd<u64, 2>; 6]) -> [Simd<u64, 2>; 5] {
    let mut out = [Simd::splat(0); 5];
    let mut carry = (s[0] + mp[0]) >> 52;
    for i in 0..5 {
        let tmp = s[i + 1] + mp[i + 1] + carry;
        out[i] = tmp.bitand(Simd::splat(MASK52));
        carry = tmp >> 52;
    }
    out
}

#[inline(never)]
pub fn transpose_u256_to_simd_stub(limbs: [[u64; 4]; 2]) -> [Simd<u64, 2>; 4] {
    transpose_u256_to_simd(limbs)
}

#[inline(always)]
pub fn transpose_u256_to_simd(limbs: [[u64; 4]; 2]) -> [Simd<u64, 2>; 4] {
    // This does not issue multiple ldp and zip which might be marginally faster.
    [
        Simd::from_array([limbs[0][0], limbs[1][0]]),
        Simd::from_array([limbs[0][1], limbs[1][1]]),
        Simd::from_array([limbs[0][2], limbs[1][2]]),
        Simd::from_array([limbs[0][3], limbs[1][3]]),
    ]
}

#[inline(always)]
pub fn transpose_simd_to_u256(limbs: [Simd<u64, 2>; 4]) -> [[u64; 4]; 2] {
    let mut result = [[0; 4]; 2];

    for i in 0..limbs.len() {
        let tmp = limbs[i].to_array();
        result[0][i] = tmp[0];
        result[1][i] = tmp[1];
    }

    result
}

#[cfg(test)]
mod tests {

    use crate::{arith, emmart::modulus_u52, yuval};
    use block_multiplier::{
        constants::{P, R2, U52_P, U52_R2},
        rtz::RTZ,
    };
    use mod256_generator::{U256b52, U256b64};
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn parallel_round(a: U256b52) {
        let rtz = RTZ::set().unwrap();

        let a_tilde = super::parallel_ref(&rtz, a.0, U52_R2);
        let a_round = super::parallel_ref(&rtz, a_tilde, [1, 0, 0, 0, 0]);

        assert_eq!(modulus_u52(a.0, U52_P), modulus_u52(a_round, U52_P))
    }

    #[quickcheck]
    fn parallel_sub_round(a: U256b52) {
        let rtz = RTZ::set().unwrap();
        let a_tilde = super::parallel_sub(&rtz, a.0, U52_R2);
        let a_round = super::parallel_sub(&rtz, a_tilde, [1, 0, 0, 0, 0]);

        assert_eq!(modulus_u52(a.0, U52_P), modulus_u52(a_round, U52_P))
    }

    #[quickcheck]
    fn parallel_sub_simd_round(a: U256b52, b: U256b52) {
        let rtz = RTZ::set().unwrap();
        let a_arrays = [a.0, b.0];
        let r2_arrays = [U52_R2, U52_R2];
        let a_tilde = super::parallel_simd_sub(&rtz, a_arrays, r2_arrays);

        let ones_arrays = [[1, 0, 0, 0, 0], [1, 0, 0, 0, 0]];
        let a_round = super::parallel_simd_sub(&rtz, a_tilde, ones_arrays);

        assert_eq!(modulus_u52(a.0, U52_P), modulus_u52(a_round[0], U52_P));
        assert_eq!(modulus_u52(b.0, U52_P), modulus_u52(a_round[1], U52_P));
    }

    #[quickcheck]
    fn parallel_sub_simd_r256_round(a: U256b64, b: U256b64) {
        let rtz = RTZ::set().unwrap();
        let a_arrays = [a.0, b.0];
        let r2_arrays = [R2, R2];
        let a_tilde = super::parallel_sub_simd_r256(&rtz, a_arrays, r2_arrays);

        let ones_arrays = [[1, 0, 0, 0], [1, 0, 0, 0]];
        let a_round = super::parallel_sub_simd_r256(&rtz, a_tilde, ones_arrays);

        assert_eq!(arith::modulus(a.0, P), arith::modulus(a_round[0], P));
        assert_eq!(arith::modulus(b.0, P), arith::modulus(a_round[1], P));
    }

    #[quickcheck]
    fn parallel_sub_r256_round(a: U256b64) {
        let rtz = RTZ::set().unwrap();
        let a_tilde = super::parallel_sub_r256(&rtz, a.0, R2);
        let a_round = super::parallel_sub_r256(&rtz, a_tilde, [1, 0, 0, 0]);

        assert_eq!(arith::modulus(a.0, P), arith::modulus(a_round, P));
    }

    #[quickcheck]
    fn parallel_sub_r256_eq(a: U256b64) {
        let rtz = RTZ::set().unwrap();
        let a_float = super::parallel_sub_r256(&rtz, a.0, R2);
        let a_uint = yuval::parallel(a.0, R2);

        assert_eq!(arith::modulus(a_float, P), arith::modulus(a_uint, P));
    }
}
