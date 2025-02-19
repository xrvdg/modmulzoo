#![allow(dead_code)]
/// 52 bit integer reference implementation
use super::MASK52;

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

pub fn sos_u52(a: [u64; 5], b: [u64; 5], n: [u64; 5], n0: u64) -> [u64; 10] {
    let mut t = [0_u64; 10];

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
pub fn cios_opt(a: [u64; 5], b: [u64; 5], n: [u64; 5], np0: u64) -> [u64; 7] {
    let mut t = [0_u64; 7];
    for i in 0..a.len() {
        let mut carry = 0;
        for j in 0..b.len() {
            (t[j], carry) = carrying_mul_add_u104(a[i], b[j], t[j], carry);
        }
        (t[b.len()], t[b.len() + 1]) = carry_add_u52(t[b.len()], carry);

        let mut carry = 0;
        let m = t[0].wrapping_mul(np0) & MASK52;
        // Outside of the loop because the loop does shifting
        (_, carry) = carrying_mul_add_u104(m, n[0], t[0], carry);

        for j in 1..n.len() {
            (t[j - 1], carry) = carrying_mul_add_u104(m, n[j], t[j], carry);
        }
        (t[n.len() - 1], carry) = carry_add_u52(t[n.len()], carry);
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

    use crate::{emmart::subtraction_step_u52, gen::U256b52, U52_NP0, U52_P, U52_R2};
    #[quickcheck]
    fn sos_round(a: U256b52) -> bool {
        let a_tilde = super::sos_u52(a.0, U52_R2, U52_P, U52_NP0);
        let a_round = super::sos_u52(
            a_tilde[5..].try_into().unwrap(),
            [1, 0, 0, 0, 0],
            U52_P,
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
        let a_tilde = super::cios_opt(a.0, U52_R2, U52_P, U52_NP0);
        let a_round = super::cios_opt(
            a_tilde[..5].try_into().unwrap(),
            [1, 0, 0, 0, 0],
            U52_P,
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
