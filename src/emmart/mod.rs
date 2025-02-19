/// Make sure to call set_round_to_zero before using any of the functions in this module
mod paper;
mod uint52;

use quickcheck::Arbitrary;

const MASK52: u64 = 2_u64.pow(52) - 1;
const MASK48: u64 = 2_u64.pow(48) - 1;

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
/// Based on CIOS_OPT from ACAR but with floating point multiplication
pub fn cios_opt(a: U256b52, b: U256b52, n: U256b52, np0: u64) -> [u64; 6] {
    let a = a.0;
    let b = b.0;
    let n = n.0;

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
pub fn cios_opt_sub(a: U256b52, b: U256b52, n: U256b52, np0: u64) -> [u64; 6] {
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

// FIOS variant of the above cios_opt_sub
// Batch all the subtractions on t[i] together
// Best performing version so far
pub fn fios_opt_sub(a: U256b52, b: U256b52, n: U256b52, np0: u64) -> [u64; 6] {
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

/// FIOS variant with the subtraction optimization
pub fn fios_opt(a: U256b52, b: U256b52, n: U256b52, np0: u64) -> [u64; 6] {
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

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct U256b64(pub [u64; 4]);
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct U256b52(pub [u64; 5]);

impl From<U256b64> for U256b52 {
    fn from(u: U256b64) -> Self {
        let U256b64(limbs) = u;
        let [l0, l1, l2, l3] = limbs;
        U256b52([
            l0 & MASK52,
            ((l0 >> 52) | (l1 << 12)) & MASK52,
            ((l1 >> 40) | (l2 << 24)) & MASK52,
            ((l2 >> 28) | (l3 << 36)) & MASK52,
            l3 >> 16,
        ])
    }
}

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
        let a = self.0;

        if a.iter().all(|&ai| ai == 0) {
            return Box::new(std::iter::empty());
        }

        // Create vector of shrunk values
        let mut shrunk = Vec::new();

        // Add zero as the smallest possible value
        shrunk.push(Self(Default::default()));

        for (i, &elem) in a.iter().enumerate() {
            if elem != 0 {
                let mut zero_limb = a.clone();
                zero_limb[i] = 0;
                let mut half_limb = a.clone();
                half_limb[i] = elem >> 1;
                shrunk.push(Self(zero_limb));
                shrunk.push(Self(half_limb));
            }
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
        let a = self.0;

        if a.iter().all(|&ai| ai == 0) {
            return Box::new(std::iter::empty());
        }

        // Create vector of shrunk values
        let mut shrunk = Vec::new();

        // Add zero as the smallest possible value
        shrunk.push(Self(Default::default()));

        for (i, &elem) in a.iter().enumerate() {
            if elem != 0 {
                let mut zero_limb = a.clone();
                zero_limb[i] = 0;
                let mut half_limb = a.clone();
                half_limb[i] = elem >> 1;
                shrunk.push(Self(zero_limb));
                shrunk.push(Self(half_limb));
            }
        }

        Box::new(shrunk.into_iter())
    }
}

#[cfg(test)]
mod tests {

    use crate::emmart::modulus_u52;
    use crate::emmart::subtraction_step_u52;
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

        a == reconverted
    }

    #[quickcheck]
    fn conv52_64(a: U256b52) -> bool {
        let converted = U256b64::from(a);
        let reconverted = U256b52::from(converted);

        a == reconverted
    }

    #[quickcheck]
    fn cios_f64_sub_round(a: U256b52) -> bool {
        set_round_to_zero();
        let a_tilde = super::cios_opt_sub(a, U256b52(U52_R2), U256b52(U52_P), U52_NP0);
        let a_round = super::cios_opt_sub(
            U256b52(a_tilde[..5].try_into().unwrap()),
            U256b52([1, 0, 0, 0, 0]),
            U256b52(U52_P),
            U52_NP0,
        );

        modulus_u52(a.0, U52_P) == subtraction_step_u52(a_round[..5].try_into().unwrap(), U52_P)
    }

    #[quickcheck]
    fn fios_f64_sub_round(a: U256b52) -> bool {
        set_round_to_zero();
        let a_tilde = super::fios_opt_sub(a, U256b52(U52_R2), U256b52(U52_P), U52_NP0);
        let a_round = super::fios_opt_sub(
            U256b52(a_tilde[..5].try_into().unwrap()),
            U256b52([1, 0, 0, 0, 0]),
            U256b52(U52_P),
            U52_NP0,
        );

        modulus_u52(a.0, U52_P) == subtraction_step_u52(a_round[..5].try_into().unwrap(), U52_P)
    }

    #[quickcheck]
    fn fios_f64_round(a: U256b52) -> bool {
        set_round_to_zero();
        let a_tilde = super::fios_opt(a, U256b52(U52_R2), U256b52(U52_P), U52_NP0);
        let a_round = super::fios_opt(
            U256b52(a_tilde[..5].try_into().unwrap()),
            U256b52([1, 0, 0, 0, 0]),
            U256b52(U52_P),
            U52_NP0,
        );

        modulus_u52(a.0, U52_P) == subtraction_step_u52(a_round[..5].try_into().unwrap(), U52_P)
    }

    #[quickcheck]
    fn cios_f64_round(a: U256b52) -> bool {
        set_round_to_zero();
        let a_tilde = super::cios_opt(a, U256b52(U52_R2), U256b52(U52_P), U52_NP0);
        let a_round = super::cios_opt(
            U256b52(a_tilde[..5].try_into().unwrap()),
            U256b52([1, 0, 0, 0, 0]),
            U256b52(U52_P),
            U52_NP0,
        );

        modulus_u52(a.0, U52_P) == subtraction_step_u52(a_round[..5].try_into().unwrap(), U52_P)
    }
}
