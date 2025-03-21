use crate::arith::{self, carrying_mul_add, school_method};
use crate::emmart::{self, make_initial};
use block_multiplier::constants::{MASK52, U52_NP0, U52_P};
use std::{
    arch::aarch64::vcvtq_f64_u64,
    ops::BitAnd,
    simd::{num::SimdFloat, Simd, StdFloat},
};

// -- [SCALAR] -------------------------------------------------------------------------------------
pub const U64_P: [u64; 4] = [
    0x43e1f593f0000001,
    0x2833e84879b97091,
    0xb85045b68181585d,
    0x30644e72e131a029,
];

pub const U64_I1: [u64; 4] = [
    0x2d3e8053e396ee4d,
    0xca478dbeab3c92cd,
    0xb2d8f06f77f52a93,
    0x24d6ba07f7aa8f04,
];
pub const U64_I2: [u64; 4] = [
    0x18ee753c76f9dc6f,
    0x54ad7e14a329e70f,
    0x2b16366f4f7684df,
    0x133100d71fdf3579,
];

pub const U64_I3: [u64; 4] = [
    0x9BACB016127CBE4E,
    0x0B2051FA31944124,
    0xB064EEA46091C76C,
    0x2B062AAA49F80C7D,
];
pub const U64_MU0: u64 = 0xc2e1f593efffffff;

#[inline(always)]
fn addv<const N: usize>(mut a: [u64; N], b: [u64; N]) -> [u64; N] {
    let mut carry = 0u64;
    for i in 0..N {
        let (sum1, overflow1) = a[i].overflowing_add(b[i]);
        let (sum2, overflow2) = sum1.overflowing_add(carry);

        a[i] = sum2;
        carry = (overflow1 as u64) + (overflow2 as u64);
    }
    a
}

#[inline(always)]
pub fn parallel(a: [u64; 4], b: [u64; 4]) -> [u64; 4] {
    let t = school_method(a, b);

    let r1 = arith::smul(t[0], crate::yuval::U64_I3);
    let r2 = arith::smul(t[1], crate::yuval::U64_I2);
    let r3 = arith::smul(t[2], crate::yuval::U64_I1);

    let s = addv(addv(t[3..].try_into().unwrap(), r1), addv(r2, r3));
    let m = crate::yuval::U64_MU0.wrapping_mul(s[0]);
    let mp = arith::smul(m, crate::yuval::U64_P);
    addv(s, mp)[1..].try_into().unwrap()
}
// -------------------------------------------------------------------------------------------------
// -- [VECTOR] -------------------------------------------------------------------------------------

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
const fn heaviside(x: isize) -> usize {
    (x >= 0) as usize
}

#[inline(always)]
pub fn vmultadd_noinit_simd(
    v0_a: [[u64; 5]; 2],
    v0_b: [[u64; 5]; 2],
    mut v0_t: [Simd<u64, 2>; 10],
) -> [Simd<u64, 2>; 10] {
    let avi = Simd::from_array([v0_a[0][0] as f64, v0_a[1][0] as f64]);
    let bvj = Simd::from_array([v0_b[0][0] as f64, v0_b[1][0] as f64]);
    let p_hi = (avi).mul_add(bvj, Simd::splat(emmart::C1));
    let p_lo = (avi).mul_add(bvj, Simd::splat(emmart::C2) - p_hi);
    v0_t[0 + 0 + 1] = v0_t[0 + 0 + 1] + p_hi.to_bits();
    v0_t[0 + 0] = v0_t[0 + 0] + p_lo.to_bits();
    let bvj = Simd::from_array([v0_b[0][1] as f64, v0_b[1][1] as f64]);
    let p_hi = (avi).mul_add(bvj, Simd::splat(emmart::C1));
    let p_lo = (avi).mul_add(bvj, Simd::splat(emmart::C2) - p_hi);
    v0_t[0 + 1 + 1] = v0_t[0 + 1 + 1] + p_hi.to_bits();
    v0_t[0 + 1] = v0_t[0 + 1] + p_lo.to_bits();
    let bvj = Simd::from_array([v0_b[0][2] as f64, v0_b[1][2] as f64]);
    let p_hi = (avi).mul_add(bvj, Simd::splat(emmart::C1));
    let p_lo = (avi).mul_add(bvj, Simd::splat(emmart::C2) - p_hi);
    v0_t[0 + 2 + 1] = v0_t[0 + 2 + 1] + p_hi.to_bits();
    v0_t[0 + 2] = v0_t[0 + 2] + p_lo.to_bits();
    let bvj = Simd::from_array([v0_b[0][3] as f64, v0_b[1][3] as f64]);
    let p_hi = (avi).mul_add(bvj, Simd::splat(emmart::C1));
    let p_lo = (avi).mul_add(bvj, Simd::splat(emmart::C2) - p_hi);
    v0_t[0 + 3 + 1] = v0_t[0 + 3 + 1] + p_hi.to_bits();
    v0_t[0 + 3] = v0_t[0 + 3] + p_lo.to_bits();
    let bvj = Simd::from_array([v0_b[0][4] as f64, v0_b[1][4] as f64]);
    let p_hi = (avi).mul_add(bvj, Simd::splat(emmart::C1));
    let p_lo = (avi).mul_add(bvj, Simd::splat(emmart::C2) - p_hi);
    v0_t[0 + 4 + 1] = v0_t[0 + 4 + 1] + p_hi.to_bits();
    v0_t[0 + 4] = v0_t[0 + 4] + p_lo.to_bits();
    let avi = Simd::from_array([v0_a[0][1] as f64, v0_a[1][1] as f64]);
    let bvj = Simd::from_array([v0_b[0][0] as f64, v0_b[1][0] as f64]);
    let p_hi = (avi).mul_add(bvj, Simd::splat(emmart::C1));
    let p_lo = (avi).mul_add(bvj, Simd::splat(emmart::C2) - p_hi);
    v0_t[1 + 0 + 1] = v0_t[1 + 0 + 1] + p_hi.to_bits();
    v0_t[1 + 0] = v0_t[1 + 0] + p_lo.to_bits();
    let bvj = Simd::from_array([v0_b[0][1] as f64, v0_b[1][1] as f64]);
    let p_hi = (avi).mul_add(bvj, Simd::splat(emmart::C1));
    let p_lo = (avi).mul_add(bvj, Simd::splat(emmart::C2) - p_hi);
    v0_t[1 + 1 + 1] = v0_t[1 + 1 + 1] + p_hi.to_bits();
    v0_t[1 + 1] = v0_t[1 + 1] + p_lo.to_bits();
    let bvj = Simd::from_array([v0_b[0][2] as f64, v0_b[1][2] as f64]);
    let p_hi = (avi).mul_add(bvj, Simd::splat(emmart::C1));
    let p_lo = (avi).mul_add(bvj, Simd::splat(emmart::C2) - p_hi);
    v0_t[1 + 2 + 1] = v0_t[1 + 2 + 1] + p_hi.to_bits();
    v0_t[1 + 2] = v0_t[1 + 2] + p_lo.to_bits();
    let bvj = Simd::from_array([v0_b[0][3] as f64, v0_b[1][3] as f64]);
    let p_hi = (avi).mul_add(bvj, Simd::splat(emmart::C1));
    let p_lo = (avi).mul_add(bvj, Simd::splat(emmart::C2) - p_hi);
    v0_t[1 + 3 + 1] = v0_t[1 + 3 + 1] + p_hi.to_bits();
    v0_t[1 + 3] = v0_t[1 + 3] + p_lo.to_bits();
    let bvj = Simd::from_array([v0_b[0][4] as f64, v0_b[1][4] as f64]);
    let p_hi = (avi).mul_add(bvj, Simd::splat(emmart::C1));
    let p_lo = (avi).mul_add(bvj, Simd::splat(emmart::C2) - p_hi);
    v0_t[1 + 4 + 1] = v0_t[1 + 4 + 1] + p_hi.to_bits();
    v0_t[1 + 4] = v0_t[1 + 4] + p_lo.to_bits();
    let avi = Simd::from_array([v0_a[0][2] as f64, v0_a[1][2] as f64]);
    let bvj = Simd::from_array([v0_b[0][0] as f64, v0_b[1][0] as f64]);
    let p_hi = (avi).mul_add(bvj, Simd::splat(emmart::C1));
    let p_lo = (avi).mul_add(bvj, Simd::splat(emmart::C2) - p_hi);
    v0_t[2 + 0 + 1] = v0_t[2 + 0 + 1] + p_hi.to_bits();
    v0_t[2 + 0] = v0_t[2 + 0] + p_lo.to_bits();
    let bvj = Simd::from_array([v0_b[0][1] as f64, v0_b[1][1] as f64]);
    let p_hi = (avi).mul_add(bvj, Simd::splat(emmart::C1));
    let p_lo = (avi).mul_add(bvj, Simd::splat(emmart::C2) - p_hi);
    v0_t[2 + 1 + 1] = v0_t[2 + 1 + 1] + p_hi.to_bits();
    v0_t[2 + 1] = v0_t[2 + 1] + p_lo.to_bits();
    let bvj = Simd::from_array([v0_b[0][2] as f64, v0_b[1][2] as f64]);
    let p_hi = (avi).mul_add(bvj, Simd::splat(emmart::C1));
    let p_lo = (avi).mul_add(bvj, Simd::splat(emmart::C2) - p_hi);
    v0_t[2 + 2 + 1] = v0_t[2 + 2 + 1] + p_hi.to_bits();
    v0_t[2 + 2] = v0_t[2 + 2] + p_lo.to_bits();
    let bvj = Simd::from_array([v0_b[0][3] as f64, v0_b[1][3] as f64]);
    let p_hi = (avi).mul_add(bvj, Simd::splat(emmart::C1));
    let p_lo = (avi).mul_add(bvj, Simd::splat(emmart::C2) - p_hi);
    v0_t[2 + 3 + 1] = v0_t[2 + 3 + 1] + p_hi.to_bits();
    v0_t[2 + 3] = v0_t[2 + 3] + p_lo.to_bits();
    let bvj = Simd::from_array([v0_b[0][4] as f64, v0_b[1][4] as f64]);
    let p_hi = (avi).mul_add(bvj, Simd::splat(emmart::C1));
    let p_lo = (avi).mul_add(bvj, Simd::splat(emmart::C2) - p_hi);
    v0_t[2 + 4 + 1] = v0_t[2 + 4 + 1] + p_hi.to_bits();
    v0_t[2 + 4] = v0_t[2 + 4] + p_lo.to_bits();
    let avi = Simd::from_array([v0_a[0][3] as f64, v0_a[1][3] as f64]);
    let bvj = Simd::from_array([v0_b[0][0] as f64, v0_b[1][0] as f64]);
    let p_hi = (avi).mul_add(bvj, Simd::splat(emmart::C1));
    let p_lo = (avi).mul_add(bvj, Simd::splat(emmart::C2) - p_hi);
    v0_t[3 + 0 + 1] = v0_t[3 + 0 + 1] + p_hi.to_bits();
    v0_t[3 + 0] = v0_t[3 + 0] + p_lo.to_bits();
    let bvj = Simd::from_array([v0_b[0][1] as f64, v0_b[1][1] as f64]);
    let p_hi = (avi).mul_add(bvj, Simd::splat(emmart::C1));
    let p_lo = (avi).mul_add(bvj, Simd::splat(emmart::C2) - p_hi);
    v0_t[3 + 1 + 1] = v0_t[3 + 1 + 1] + p_hi.to_bits();
    v0_t[3 + 1] = v0_t[3 + 1] + p_lo.to_bits();
    let bvj = Simd::from_array([v0_b[0][2] as f64, v0_b[1][2] as f64]);
    let p_hi = (avi).mul_add(bvj, Simd::splat(emmart::C1));
    let p_lo = (avi).mul_add(bvj, Simd::splat(emmart::C2) - p_hi);
    v0_t[3 + 2 + 1] = v0_t[3 + 2 + 1] + p_hi.to_bits();
    v0_t[3 + 2] = v0_t[3 + 2] + p_lo.to_bits();
    let bvj = Simd::from_array([v0_b[0][3] as f64, v0_b[1][3] as f64]);
    let p_hi = (avi).mul_add(bvj, Simd::splat(emmart::C1));
    let p_lo = (avi).mul_add(bvj, Simd::splat(emmart::C2) - p_hi);
    v0_t[3 + 3 + 1] = v0_t[3 + 3 + 1] + p_hi.to_bits();
    v0_t[3 + 3] = v0_t[3 + 3] + p_lo.to_bits();
    let bvj = Simd::from_array([v0_b[0][4] as f64, v0_b[1][4] as f64]);
    let p_hi = (avi).mul_add(bvj, Simd::splat(emmart::C1));
    let p_lo = (avi).mul_add(bvj, Simd::splat(emmart::C2) - p_hi);
    v0_t[3 + 4 + 1] = v0_t[3 + 4 + 1] + p_hi.to_bits();
    v0_t[3 + 4] = v0_t[3 + 4] + p_lo.to_bits();
    let avi = Simd::from_array([v0_a[0][4] as f64, v0_a[1][4] as f64]);
    let bvj = Simd::from_array([v0_b[0][0] as f64, v0_b[1][0] as f64]);
    let p_hi = (avi).mul_add(bvj, Simd::splat(emmart::C1));
    let p_lo = (avi).mul_add(bvj, Simd::splat(emmart::C2) - p_hi);
    v0_t[4 + 0 + 1] = v0_t[4 + 0 + 1] + p_hi.to_bits();
    v0_t[4 + 0] = v0_t[4 + 0] + p_lo.to_bits();
    let bvj = Simd::from_array([v0_b[0][1] as f64, v0_b[1][1] as f64]);
    let p_hi = (avi).mul_add(bvj, Simd::splat(emmart::C1));
    let p_lo = (avi).mul_add(bvj, Simd::splat(emmart::C2) - p_hi);
    v0_t[4 + 1 + 1] = v0_t[4 + 1 + 1] + p_hi.to_bits();
    v0_t[4 + 1] = v0_t[4 + 1] + p_lo.to_bits();
    let bvj = Simd::from_array([v0_b[0][2] as f64, v0_b[1][2] as f64]);
    let p_hi = (avi).mul_add(bvj, Simd::splat(emmart::C1));
    let p_lo = (avi).mul_add(bvj, Simd::splat(emmart::C2) - p_hi);
    v0_t[4 + 2 + 1] = v0_t[4 + 2 + 1] + p_hi.to_bits();
    v0_t[4 + 2] = v0_t[4 + 2] + p_lo.to_bits();
    let bvj = Simd::from_array([v0_b[0][3] as f64, v0_b[1][3] as f64]);
    let p_hi = (avi).mul_add(bvj, Simd::splat(emmart::C1));
    let p_lo = (avi).mul_add(bvj, Simd::splat(emmart::C2) - p_hi);
    v0_t[4 + 3 + 1] = v0_t[4 + 3 + 1] + p_hi.to_bits();
    v0_t[4 + 3] = v0_t[4 + 3] + p_lo.to_bits();
    let bvj = Simd::from_array([v0_b[0][4] as f64, v0_b[1][4] as f64]);
    let p_hi = (avi).mul_add(bvj, Simd::splat(emmart::C1));
    let p_lo = (avi).mul_add(bvj, Simd::splat(emmart::C2) - p_hi);
    v0_t[4 + 4 + 1] = v0_t[4 + 4 + 1] + p_hi.to_bits();
    v0_t[4 + 4] = v0_t[4 + 4] + p_lo.to_bits();
    v0_t
}

#[inline(always)]
fn smult_noinit_simd(s: Simd<u64, 2>, v: [u64; 5]) -> [Simd<u64, 2>; 6] {
    let mut t = [Simd::splat(0); 6];
    let s: Simd<f64, 2> = unsafe { vcvtq_f64_u64(s.into()).into() };
    let p_hi_0 = s.mul_add(Simd::splat(v[0] as f64), Simd::splat(emmart::C1));
    let p_lo_0 = s.mul_add(Simd::splat(v[0] as f64), Simd::splat(emmart::C2) - p_hi_0);
    t[1] = t[1] + p_hi_0.to_bits();
    t[0] = t[0] + p_lo_0.to_bits();
    let p_hi_1 = s.mul_add(Simd::splat(v[1] as f64), Simd::splat(emmart::C1));
    let p_lo_1 = s.mul_add(Simd::splat(v[1] as f64), Simd::splat(emmart::C2) - p_hi_1);
    t[2] = t[2] + p_hi_1.to_bits();
    t[1] = t[1] + p_lo_1.to_bits();
    let p_hi_2 = s.mul_add(Simd::splat(v[2] as f64), Simd::splat(emmart::C1));
    let p_lo_2 = s.mul_add(Simd::splat(v[2] as f64), Simd::splat(emmart::C2) - p_hi_2);
    t[3] = t[3] + p_hi_2.to_bits();
    t[2] = t[2] + p_lo_2.to_bits();
    let p_hi_3 = s.mul_add(Simd::splat(v[3] as f64), Simd::splat(emmart::C1));
    let p_lo_3 = s.mul_add(Simd::splat(v[3] as f64), Simd::splat(emmart::C2) - p_hi_3);
    t[4] = t[4] + p_hi_3.to_bits();
    t[3] = t[3] + p_lo_3.to_bits();
    let p_hi_4 = s.mul_add(Simd::splat(v[4] as f64), Simd::splat(emmart::C1));
    let p_lo_4 = s.mul_add(Simd::splat(v[4] as f64), Simd::splat(emmart::C2) - p_hi_4);
    t[5] = t[5] + p_hi_4.to_bits();
    t[4] = t[4] + p_lo_4.to_bits();
    t
}

#[inline(always)]
fn addv_simd<const N: usize>(
    mut va: [Simd<u64, 2>; N],
    vb: [Simd<u64, 2>; N],
) -> [Simd<u64, 2>; N] {
    va[0] = va[0] + vb[0];
    va[1] = va[1] + vb[1];
    va
}

#[inline(always)]
pub fn resolve_simd_add_truncate(s: [Simd<u64, 2>; 6], mp: [Simd<u64, 2>; 6]) -> [[u64; 5]; 2] {
    let mut out = [[0; 5]; 2];
    let mut carry = (s[0] + mp[0]) >> 52;

    let tmp_0 = s[1] + mp[1] + carry;
    [out[0][0], out[1][0]] = tmp_0.bitand(Simd::splat(MASK52)).to_array();
    carry = tmp_0 >> 52;

    let tmp_1 = s[2] + mp[2] + carry;
    [out[0][1], out[1][1]] = tmp_1.bitand(Simd::splat(MASK52)).to_array();
    carry = tmp_1 >> 52;

    let tmp_2 = s[3] + mp[3] + carry;
    [out[0][2], out[1][2]] = tmp_2.bitand(Simd::splat(MASK52)).to_array();
    carry = tmp_2 >> 52;

    let tmp_3 = s[4] + mp[4] + carry;
    [out[0][3], out[1][3]] = tmp_3.bitand(Simd::splat(MASK52)).to_array();
    carry = tmp_3 >> 52;

    let tmp_4 = s[5] + mp[5] + carry;
    [out[0][4], out[1][4]] = tmp_4.bitand(Simd::splat(MASK52)).to_array();
    carry = tmp_4 >> 52;
    out
}

#[inline(always)]
pub fn parallel_simd_sub(a: [[u64; 5]; 2], b: [[u64; 5]; 2]) -> [[u64; 5]; 2] {
    let mut t: [Simd<u64, 2>; 10] = [Simd::splat(0); 10];
    t[0] = Simd::splat(make_initial(1, 0));
    t[9] = Simd::splat(make_initial(0, 6));
    t[1] = Simd::splat(make_initial(2, 1));
    t[8] = Simd::splat(make_initial(6, 7));
    t[2] = Simd::splat(make_initial(3, 2));
    t[7] = Simd::splat(make_initial(7, 8));
    t[3] = Simd::splat(make_initial(4, 3));
    t[6] = Simd::splat(make_initial(8, 9));
    t[4] = Simd::splat(make_initial(10, 4));
    t[5] = Simd::splat(make_initial(9, 10));

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

    let s: [Simd<u64, 2>; 6] = t[4..].try_into().unwrap();

    // This can also be a fiveway-add in a loop, but I think the compiler already takes care of this.
    let s = addv_simd(r3, addv_simd(addv_simd(s, r0), addv_simd(r1, r2)));

    let m = (s[0] * Simd::splat(U52_NP0)).bitand(Simd::splat(MASK52));
    let mp = smult_noinit_simd(m, U52_P);
    resolve_simd_add_truncate(s, mp)
}
// -------------------------------------------------------------------------------------------------

pub struct InterleavedRes {
    s0: [u64; 4],
    v0: [[u64; 5]; 2],
}

pub fn interleaved(
    s0_a: [u64; 4],
    s0_b: [u64; 4],
    v0_a: [[u64; 5]; 2],
    v0_b: [[u64; 5]; 2],
) -> InterleavedRes {
    // -- [VECTOR] ---------------------------------------------------------------------------------
    let mut v0_t: [Simd<u64, 2>; 10] = [Simd::splat(0); 10];

    v0_t[0] = Simd::splat(make_initial(1, 0));
    v0_t[9] = Simd::splat(make_initial(0, 6));
    v0_t[1] = Simd::splat(make_initial(2, 1));
    v0_t[8] = Simd::splat(make_initial(6, 7));
    v0_t[2] = Simd::splat(make_initial(3, 2));
    v0_t[7] = Simd::splat(make_initial(7, 8));
    v0_t[3] = Simd::splat(make_initial(4, 3));
    v0_t[6] = Simd::splat(make_initial(8, 9));
    v0_t[4] = Simd::splat(make_initial(10, 4));
    v0_t[5] = Simd::splat(make_initial(9, 10));

    let avi = Simd::from_array([v0_a[0][0] as f64, v0_a[1][0] as f64]);
    let bvj = Simd::from_array([v0_b[0][0] as f64, v0_b[1][0] as f64]);
    let p_hi = (avi).mul_add(bvj, Simd::splat(emmart::C1));
    let p_lo = (avi).mul_add(bvj, Simd::splat(emmart::C2) - p_hi);
    v0_t[0 + 0 + 1] = v0_t[0 + 0 + 1] + p_hi.to_bits();
    v0_t[0 + 0] = v0_t[0 + 0] + p_lo.to_bits();
    let bvj = Simd::from_array([v0_b[0][1] as f64, v0_b[1][1] as f64]);
    let p_hi = (avi).mul_add(bvj, Simd::splat(emmart::C1));
    let p_lo = (avi).mul_add(bvj, Simd::splat(emmart::C2) - p_hi);
    v0_t[0 + 1 + 1] = v0_t[0 + 1 + 1] + p_hi.to_bits();
    v0_t[0 + 1] = v0_t[0 + 1] + p_lo.to_bits();
    let bvj = Simd::from_array([v0_b[0][2] as f64, v0_b[1][2] as f64]);
    let p_hi = (avi).mul_add(bvj, Simd::splat(emmart::C1));
    let p_lo = (avi).mul_add(bvj, Simd::splat(emmart::C2) - p_hi);
    v0_t[0 + 2 + 1] = v0_t[0 + 2 + 1] + p_hi.to_bits();
    v0_t[0 + 2] = v0_t[0 + 2] + p_lo.to_bits();
    let bvj = Simd::from_array([v0_b[0][3] as f64, v0_b[1][3] as f64]);
    let p_hi = (avi).mul_add(bvj, Simd::splat(emmart::C1));
    let p_lo = (avi).mul_add(bvj, Simd::splat(emmart::C2) - p_hi);
    v0_t[0 + 3 + 1] = v0_t[0 + 3 + 1] + p_hi.to_bits();
    v0_t[0 + 3] = v0_t[0 + 3] + p_lo.to_bits();
    let bvj = Simd::from_array([v0_b[0][4] as f64, v0_b[1][4] as f64]);
    let p_hi = (avi).mul_add(bvj, Simd::splat(emmart::C1));
    let p_lo = (avi).mul_add(bvj, Simd::splat(emmart::C2) - p_hi);
    v0_t[0 + 4 + 1] = v0_t[0 + 4 + 1] + p_hi.to_bits();
    v0_t[0 + 4] = v0_t[0 + 4] + p_lo.to_bits();
    let avi = Simd::from_array([v0_a[0][1] as f64, v0_a[1][1] as f64]);
    let bvj = Simd::from_array([v0_b[0][0] as f64, v0_b[1][0] as f64]);
    let p_hi = (avi).mul_add(bvj, Simd::splat(emmart::C1));
    let p_lo = (avi).mul_add(bvj, Simd::splat(emmart::C2) - p_hi);
    v0_t[1 + 0 + 1] = v0_t[1 + 0 + 1] + p_hi.to_bits();
    v0_t[1 + 0] = v0_t[1 + 0] + p_lo.to_bits();
    let bvj = Simd::from_array([v0_b[0][1] as f64, v0_b[1][1] as f64]);
    let p_hi = (avi).mul_add(bvj, Simd::splat(emmart::C1));
    let p_lo = (avi).mul_add(bvj, Simd::splat(emmart::C2) - p_hi);
    v0_t[1 + 1 + 1] = v0_t[1 + 1 + 1] + p_hi.to_bits();
    v0_t[1 + 1] = v0_t[1 + 1] + p_lo.to_bits();
    let bvj = Simd::from_array([v0_b[0][2] as f64, v0_b[1][2] as f64]);
    let p_hi = (avi).mul_add(bvj, Simd::splat(emmart::C1));
    let p_lo = (avi).mul_add(bvj, Simd::splat(emmart::C2) - p_hi);
    v0_t[1 + 2 + 1] = v0_t[1 + 2 + 1] + p_hi.to_bits();
    v0_t[1 + 2] = v0_t[1 + 2] + p_lo.to_bits();
    let bvj = Simd::from_array([v0_b[0][3] as f64, v0_b[1][3] as f64]);
    let p_hi = (avi).mul_add(bvj, Simd::splat(emmart::C1));
    let p_lo = (avi).mul_add(bvj, Simd::splat(emmart::C2) - p_hi);
    v0_t[1 + 3 + 1] = v0_t[1 + 3 + 1] + p_hi.to_bits();
    v0_t[1 + 3] = v0_t[1 + 3] + p_lo.to_bits();
    let bvj = Simd::from_array([v0_b[0][4] as f64, v0_b[1][4] as f64]);
    let p_hi = (avi).mul_add(bvj, Simd::splat(emmart::C1));
    let p_lo = (avi).mul_add(bvj, Simd::splat(emmart::C2) - p_hi);
    v0_t[1 + 4 + 1] = v0_t[1 + 4 + 1] + p_hi.to_bits();
    v0_t[1 + 4] = v0_t[1 + 4] + p_lo.to_bits();
    let avi = Simd::from_array([v0_a[0][2] as f64, v0_a[1][2] as f64]);
    let bvj = Simd::from_array([v0_b[0][0] as f64, v0_b[1][0] as f64]);
    let p_hi = (avi).mul_add(bvj, Simd::splat(emmart::C1));
    let p_lo = (avi).mul_add(bvj, Simd::splat(emmart::C2) - p_hi);
    v0_t[2 + 0 + 1] = v0_t[2 + 0 + 1] + p_hi.to_bits();
    v0_t[2 + 0] = v0_t[2 + 0] + p_lo.to_bits();
    let bvj = Simd::from_array([v0_b[0][1] as f64, v0_b[1][1] as f64]);
    let p_hi = (avi).mul_add(bvj, Simd::splat(emmart::C1));
    let p_lo = (avi).mul_add(bvj, Simd::splat(emmart::C2) - p_hi);
    v0_t[2 + 1 + 1] = v0_t[2 + 1 + 1] + p_hi.to_bits();
    v0_t[2 + 1] = v0_t[2 + 1] + p_lo.to_bits();
    let bvj = Simd::from_array([v0_b[0][2] as f64, v0_b[1][2] as f64]);
    let p_hi = (avi).mul_add(bvj, Simd::splat(emmart::C1));
    let p_lo = (avi).mul_add(bvj, Simd::splat(emmart::C2) - p_hi);
    v0_t[2 + 2 + 1] = v0_t[2 + 2 + 1] + p_hi.to_bits();
    v0_t[2 + 2] = v0_t[2 + 2] + p_lo.to_bits();
    let bvj = Simd::from_array([v0_b[0][3] as f64, v0_b[1][3] as f64]);
    let p_hi = (avi).mul_add(bvj, Simd::splat(emmart::C1));
    let p_lo = (avi).mul_add(bvj, Simd::splat(emmart::C2) - p_hi);
    v0_t[2 + 3 + 1] = v0_t[2 + 3 + 1] + p_hi.to_bits();
    v0_t[2 + 3] = v0_t[2 + 3] + p_lo.to_bits();
    let bvj = Simd::from_array([v0_b[0][4] as f64, v0_b[1][4] as f64]);
    let p_hi = (avi).mul_add(bvj, Simd::splat(emmart::C1));
    let p_lo = (avi).mul_add(bvj, Simd::splat(emmart::C2) - p_hi);
    v0_t[2 + 4 + 1] = v0_t[2 + 4 + 1] + p_hi.to_bits();
    v0_t[2 + 4] = v0_t[2 + 4] + p_lo.to_bits();
    let avi = Simd::from_array([v0_a[0][3] as f64, v0_a[1][3] as f64]);
    let bvj = Simd::from_array([v0_b[0][0] as f64, v0_b[1][0] as f64]);
    let p_hi = (avi).mul_add(bvj, Simd::splat(emmart::C1));
    let p_lo = (avi).mul_add(bvj, Simd::splat(emmart::C2) - p_hi);
    v0_t[3 + 0 + 1] = v0_t[3 + 0 + 1] + p_hi.to_bits();
    v0_t[3 + 0] = v0_t[3 + 0] + p_lo.to_bits();
    let bvj = Simd::from_array([v0_b[0][1] as f64, v0_b[1][1] as f64]);
    let p_hi = (avi).mul_add(bvj, Simd::splat(emmart::C1));
    let p_lo = (avi).mul_add(bvj, Simd::splat(emmart::C2) - p_hi);
    v0_t[3 + 1 + 1] = v0_t[3 + 1 + 1] + p_hi.to_bits();
    v0_t[3 + 1] = v0_t[3 + 1] + p_lo.to_bits();
    let bvj = Simd::from_array([v0_b[0][2] as f64, v0_b[1][2] as f64]);
    let p_hi = (avi).mul_add(bvj, Simd::splat(emmart::C1));
    let p_lo = (avi).mul_add(bvj, Simd::splat(emmart::C2) - p_hi);
    v0_t[3 + 2 + 1] = v0_t[3 + 2 + 1] + p_hi.to_bits();
    v0_t[3 + 2] = v0_t[3 + 2] + p_lo.to_bits();
    let bvj = Simd::from_array([v0_b[0][3] as f64, v0_b[1][3] as f64]);
    let p_hi = (avi).mul_add(bvj, Simd::splat(emmart::C1));
    let p_lo = (avi).mul_add(bvj, Simd::splat(emmart::C2) - p_hi);
    v0_t[3 + 3 + 1] = v0_t[3 + 3 + 1] + p_hi.to_bits();
    v0_t[3 + 3] = v0_t[3 + 3] + p_lo.to_bits();
    let bvj = Simd::from_array([v0_b[0][4] as f64, v0_b[1][4] as f64]);
    let p_hi = (avi).mul_add(bvj, Simd::splat(emmart::C1));
    let p_lo = (avi).mul_add(bvj, Simd::splat(emmart::C2) - p_hi);
    v0_t[3 + 4 + 1] = v0_t[3 + 4 + 1] + p_hi.to_bits();
    v0_t[3 + 4] = v0_t[3 + 4] + p_lo.to_bits();
    let avi = Simd::from_array([v0_a[0][4] as f64, v0_a[1][4] as f64]);
    let bvj = Simd::from_array([v0_b[0][0] as f64, v0_b[1][0] as f64]);
    let p_hi = (avi).mul_add(bvj, Simd::splat(emmart::C1));
    let p_lo = (avi).mul_add(bvj, Simd::splat(emmart::C2) - p_hi);
    v0_t[4 + 0 + 1] = v0_t[4 + 0 + 1] + p_hi.to_bits();
    v0_t[4 + 0] = v0_t[4 + 0] + p_lo.to_bits();
    let bvj = Simd::from_array([v0_b[0][1] as f64, v0_b[1][1] as f64]);
    let p_hi = (avi).mul_add(bvj, Simd::splat(emmart::C1));
    let p_lo = (avi).mul_add(bvj, Simd::splat(emmart::C2) - p_hi);
    v0_t[4 + 1 + 1] = v0_t[4 + 1 + 1] + p_hi.to_bits();
    v0_t[4 + 1] = v0_t[4 + 1] + p_lo.to_bits();
    let bvj = Simd::from_array([v0_b[0][2] as f64, v0_b[1][2] as f64]);
    let p_hi = (avi).mul_add(bvj, Simd::splat(emmart::C1));
    let p_lo = (avi).mul_add(bvj, Simd::splat(emmart::C2) - p_hi);
    v0_t[4 + 2 + 1] = v0_t[4 + 2 + 1] + p_hi.to_bits();
    v0_t[4 + 2] = v0_t[4 + 2] + p_lo.to_bits();
    let bvj = Simd::from_array([v0_b[0][3] as f64, v0_b[1][3] as f64]);
    let p_hi = (avi).mul_add(bvj, Simd::splat(emmart::C1));
    let p_lo = (avi).mul_add(bvj, Simd::splat(emmart::C2) - p_hi);
    v0_t[4 + 3 + 1] = v0_t[4 + 3 + 1] + p_hi.to_bits();
    v0_t[4 + 3] = v0_t[4 + 3] + p_lo.to_bits();
    let bvj = Simd::from_array([v0_b[0][4] as f64, v0_b[1][4] as f64]);
    let p_hi = (avi).mul_add(bvj, Simd::splat(emmart::C1));
    let p_lo = (avi).mul_add(bvj, Simd::splat(emmart::C2) - p_hi);
    v0_t[4 + 4 + 1] = v0_t[4 + 4 + 1] + p_hi.to_bits();
    v0_t[4 + 4] = v0_t[4 + 4] + p_lo.to_bits();

    v0_t[1] += v0_t[0] >> 52;
    v0_t[2] += v0_t[1] >> 52;
    v0_t[3] += v0_t[2] >> 52;
    v0_t[4] += v0_t[3] >> 52;
    let v0_r0 = smult_noinit_simd(v0_t[0].bitand(Simd::splat(MASK52)), RHO_4);
    let v0_r1 = smult_noinit_simd(v0_t[1].bitand(Simd::splat(MASK52)), RHO_3);
    let v0_r2 = smult_noinit_simd(v0_t[2].bitand(Simd::splat(MASK52)), RHO_2);
    let v0_r3 = smult_noinit_simd(v0_t[3].bitand(Simd::splat(MASK52)), RHO_1);

    let v0_s: [Simd<u64, 2>; 6] = v0_t[4..].try_into().unwrap();

    let v0_s = addv_simd(
        v0_r3,
        addv_simd(addv_simd(v0_s, v0_r0), addv_simd(v0_r1, v0_r2)),
    );

    let v0_m = (v0_s[0] * Simd::splat(U52_NP0)).bitand(Simd::splat(MASK52));
    let v0_mp = smult_noinit_simd(v0_m, U52_P);
    let v0 = resolve_simd_add_truncate(v0_s, v0_mp);
    // ---------------------------------------------------------------------------------------------
    // -- [SCALAR] ---------------------------------------------------------------------------------
    let s0_t = school_method(s0_a, s0_b);

    let mut s0_r1 = [0_u64; 5];
    (s0_r1[0], s0_r1[1]) = carrying_mul_add(s0_t[0], crate::yuval::U64_I3[0], s0_r1[0], 0);
    (s0_r1[1], s0_r1[2]) = carrying_mul_add(s0_t[0], crate::yuval::U64_I3[1], s0_r1[1], 0);
    (s0_r1[2], s0_r1[3]) = carrying_mul_add(s0_t[0], crate::yuval::U64_I3[2], s0_r1[2], 0);
    (s0_r1[3], s0_r1[4]) = carrying_mul_add(s0_t[0], crate::yuval::U64_I3[3], s0_r1[3], 0);

    let mut s0_r2 = [0_u64; 5];
    (s0_r2[0], s0_r2[1]) = carrying_mul_add(s0_t[1], crate::yuval::U64_I2[0], s0_r2[0], 0);
    (s0_r2[1], s0_r2[2]) = carrying_mul_add(s0_t[1], crate::yuval::U64_I2[1], s0_r2[1], 0);
    (s0_r2[2], s0_r2[3]) = carrying_mul_add(s0_t[1], crate::yuval::U64_I2[2], s0_r2[2], 0);
    (s0_r2[3], s0_r2[4]) = carrying_mul_add(s0_t[1], crate::yuval::U64_I2[3], s0_r2[3], 0);

    let mut s0_r3 = [0_u64; 5];
    (s0_r3[0], s0_r3[1]) = carrying_mul_add(s0_t[2], crate::yuval::U64_I1[0], s0_r3[0], 0);
    (s0_r3[1], s0_r3[2]) = carrying_mul_add(s0_t[2], crate::yuval::U64_I1[1], s0_r3[1], 0);
    (s0_r3[2], s0_r3[3]) = carrying_mul_add(s0_t[2], crate::yuval::U64_I1[2], s0_r3[2], 0);
    (s0_r3[3], s0_r3[4]) = carrying_mul_add(s0_t[2], crate::yuval::U64_I1[3], s0_r3[3], 0);

    let s0_s = addv(
        addv(s0_t[3..].try_into().unwrap(), s0_r1),
        addv(s0_r2, s0_r3),
    );
    let s0_m = crate::yuval::U64_MU0.wrapping_mul(s0_s[0]);
    let s0_mp = arith::smul(s0_m, crate::yuval::U64_P);
    let s0 = addv(s0_s, s0_mp)[1..].try_into().unwrap();
    // ---------------------------------------------------------------------------------------------

    InterleavedRes { s0, v0 }
}

pub fn interleaved_naive(
    s0_a: [u64; 4],
    s0_b: [u64; 4],
    v0_a: [[u64; 5]; 2],
    v0_b: [[u64; 5]; 2],
) -> InterleavedRes {
    let v0 = parallel_simd_sub(v0_a, v0_b);
    let s0 = parallel(s0_a, s0_b);
    InterleavedRes { s0, v0 }
}
