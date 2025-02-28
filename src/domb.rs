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
fn addv<const N: usize>(mut va: [u64; N], vb: [u64; N]) -> [u64; N] {
    for i in 0..va.len() {
        va[i] += vb[i];
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

pub fn parallel_sub(a: [u64; 5], b: [u64; 5]) -> [u64; 5] {
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

#[cfg(test)]
mod tests {
    use std::hint::black_box;

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
}
