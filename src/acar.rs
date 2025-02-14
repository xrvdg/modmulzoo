#![feature(bigint_helper_methods)]

pub type U256 = [u64; 4];

// first is result, second is carry (S,C)
// we keep the same signature as the rust libraries instead of what is common in literature
#[inline(always)]
fn carrying_mul_add(a: u64, b: u64, add: u64, carry: u64) -> (u64, u64) {
    let c: u128 = a as u128 * b as u128 + carry as u128 + add as u128;
    (c as u64, (c >> 64) as u64)
}

// direct implementation
fn naive(a: U256, b: U256, n: U256, np: U256) -> Vec<u64> {
    // Can transmute do splitting?
    // The plus one is for the addition of t + m*n
    let mut ab = vec![0_u64; 8];

    // TODO: try to write an iterator version of this. I don't think you can't as you refer back to an element you just created

    // multiplication a * b
    for i in 0..a.len() {
        let mut carry = 0;
        for j in 0..b.len() {
            (ab[i + j], carry) = carrying_mul_add(a[i], b[j], ab[i + j], carry)
        }
        ab[i + b.len()] = carry;
    }

    // multiplication with scalar, optimized to only take the lower portion
    // map formulation
    // let m = ab.iter().take(ab.len() / 2).map(|t| t.wrapping_mul(n0));
    // ab[i] * n0 mod W
    // let mut m: U256 = [0_64; 4];
    // for i in 0..m.len() {
    //     // THIS can be a scalar vector optimization
    //     // This is probably wrong
    //     m[i] = ab[i].wrapping_mul(n0);
    // }

    let t = ab;
    let mut m = [0_u64; 4];
    // m = TN' mod R
    for i in 0..np.len() {
        let mut carry = 0;
        for j in 0..(np.len() - i) {
            (m[i + j], carry) = carrying_mul_add(n[i], t[j], m[i + j], carry);
        }
        // interesting that there is no carry over here
    }

    // Move loops around and do loop fusion with the above is what sos does
    // t + m*n
    // Doing a multiply add results in a adds here. You can prevent this by going double space.
    // How do the product based ones handle this?
    let mut nm = [0_64; 8];
    for i in 0..n.len() {
        let mut carry = 0;
        for j in 0..m.len() {
            (nm[i + j], carry) = carrying_mul_add(n[i], m[j], nm[i + j], carry);
        }
        nm[i + m.len()] = carry;
    }

    let mut out = [0_64; 9];
    // Extra loop to do the addition that makes just a single pass for the carry
    for i in 0..nm.len() {
        // Could also directly assign it to a windowing slice
        let r = t[i] as u128 + nm[i] as u128 + out[i] as u128;
        (out[i], out[i + 1]) = (r as u64, (r >> 64) as u64);
    }

    // let (t, u) = out.split_at_mut(4);
    // let mut b = 0;
    // for i in 0..t.len() {
    //     let r = u[i] as u128 - n[i] as u128 - b as u128;

    //     (t[i], b) = (r as u64, (r >> 64) as u64);
    // }
    // b = ((u[t.len() + 1] as u128 - b as u128) >> 64) as u64;
    // // not interested in the result itself

    // if b == 0 {
    //     t.try_into().unwrap()
    // } else {
    //     u[..u.len() - 1].try_into().unwrap()
    // }

    t
}

pub fn sos(a: U256, b: U256, n: U256, n0: u64) -> [u64; 8] {
    // Can transmute do splitting?
    // The plus one is for the addition of t + m*n
    let mut t = [0_u64; 8];

    // TODO: try to write an iterator version of this. I don't think you can't as you refer back to an element you just created

    // multiplication a * b
    for i in 0..a.len() {
        let mut carry = 0;
        for j in 0..b.len() {
            (t[i + j], carry) = carrying_mul_add(a[i], b[j], t[i + j], carry)
        }
        t[i + b.len()] = carry;
    }

    for i in 0..(n.len()) {
        let mut carry = 0;
        let m = t[i].wrapping_mul(n0);
        for j in 0..n.len() {
            (t[i + j], carry) = carrying_mul_add(m, n[j], t[i + j], carry)
        }
        adds(&mut t[(i + n.len())..], carry)
    }

    // let (t, u) = t.split_at_mut(4);
    // // Always calculate T-N which should be as fast as checking
    // let mut b = 0;
    // for i in 0..t.len() {
    //     let r = u[i] as u128 - n[i] as u128 - b as u128;

    //     (t[i], b) = (r as u64, (r >> 64) as u64);
    // }
    // b = ((u[t.len() + 1] as u128 - b as u128) >> 64) as u64;
    // // not interested in the result itself

    // if b == 0 {
    //     t.try_into().unwrap()
    // } else {
    //     u[..u.len() - 1].try_into().unwrap()
    // }

    t
}

pub fn cios(a: U256, b: U256, n: U256, np0: u64) -> [u64; 6] {
    let mut t = [0_u64; 6];
    for i in 0..a.len() {
        let mut carry = 0;
        for j in 0..b.len() {
            (t[j], carry) = carrying_mul_add(a[i], b[j], t[j], carry);
        }
        (t[b.len()], t[b.len() + 1]) = carry_add(t[b.len()], carry);

        let mut carry = 0;
        let m = t[0].wrapping_mul(np0);
        for j in 0..n.len() {
            (t[j], carry) = carrying_mul_add(m, n[j], t[j], carry);
        }
        (t[n.len()], carry) = carry_add(t[n.len()], carry);
        (t[n.len() + 1], _) = carry_add(t[b.len() + 1], carry);

        // Division by w
        for j in 0..t.len() - 1 {
            t[j] = t[j + 1]
        }
    }
    t
}

pub fn cios_opt(a: U256, b: U256, n: U256, np0: u64) -> [u64; 6] {
    let mut t = [0_u64; 6];
    for i in 0..a.len() {
        let mut carry = 0;
        for j in 0..b.len() {
            (t[j], carry) = carrying_mul_add(a[i], b[j], t[j], carry);
        }
        (t[b.len()], t[b.len() + 1]) = carry_add(t[b.len()], carry);

        let mut carry = 0;
        let m = t[0].wrapping_mul(np0);
        (t[0], carry) = carrying_mul_add(m, n[0], t[0], carry);

        for j in 1..n.len() {
            (t[j - 1], carry) = carrying_mul_add(m, n[j], t[j], carry);
        }
        (t[n.len() - 1], carry) = carry_add(t[n.len()], carry);
        (t[n.len()], _) = carry_add(t[n.len() + 1], carry);
    }
    t
}

pub fn fios(a: U256, b: U256, n: U256, np0: u64) -> [u64; 6] {
    let mut t = [0_u64; 6];
    for i in 0..a.len() {
        let (sum, mut carry) = carrying_mul_add(a[i], b[0], t[0], 0);
        adds(&mut t[1..], carry);
        let m = sum.wrapping_mul(np0);
        let _sum;
        (_sum, carry) = carrying_mul_add(m, n[0], sum, 0);

        // could also be n
        for j in 1..b.len() {
            let (sum, carry2) = carrying_mul_add(a[i], b[j], t[j], carry);
            adds(&mut t[j + 1..], carry2);
            (t[j - 1], carry) = carrying_mul_add(m, n[j], sum, 0);
        }
        (t[n.len() - 1], carry) = carry_add(t[n.len()], carry);
        (t[n.len()], _) = carry_add(t[n.len() + 1], carry);
        t[n.len() + 1] = 0;
    }

    t
}

// TODO When chaining does this actually give the proper instructions?
#[inline(always)]
fn carry_add(lhs: u64, carry: u64) -> (u64, u64) {
    let (sum, carry) = lhs.overflowing_add(carry);
    (sum, carry.into())
}

// Adds can probably be removed if you allow for a bigger carry
// Only the first addition is u64 the later are single bit increase
// How is this solved in the latter ones?
fn adds(t: &mut [u64], mut carry: u64) {
    for i in 0..t.len() {
        let b;
        (t[i], b) = t[i].overflowing_add(carry);
        // Add if to exit
        carry = b.into();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{NP0, P}; // Import constants from the crate root
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn cios_sos(a: Vec<u64>, b: Vec<u64>) -> bool {
        // Ensure vectors are length 4 by either truncating or padding with zeros
        let a: [u64; 4] = if a.len() >= 4 {
            a[..4].try_into().unwrap()
        } else {
            let mut padded = [0u64; 4];
            padded[..a.len()].copy_from_slice(&a);
            padded
        };

        let b: [u64; 4] = if b.len() >= 4 {
            b[..4].try_into().unwrap()
        } else {
            let mut padded = [0u64; 4];
            padded[..b.len()].copy_from_slice(&b);
            padded
        };

        cios(a, b, P, NP0)[..4] == sos(a, b, P, NP0)[4..]
    }
    #[quickcheck]
    fn cios_ciosopt(a: Vec<u64>, b: Vec<u64>) -> bool {
        // Ensure vectors are length 4 by either truncating or padding with zeros
        let a: [u64; 4] = if a.len() >= 4 {
            a[..4].try_into().unwrap()
        } else {
            let mut padded = [0u64; 4];
            padded[..a.len()].copy_from_slice(&a);
            padded
        };

        let b: [u64; 4] = if b.len() >= 4 {
            b[..4].try_into().unwrap()
        } else {
            let mut padded = [0u64; 4];
            padded[..b.len()].copy_from_slice(&b);
            padded
        };

        cios(a, b, P, NP0)[..5] == cios_opt(a, b, P, NP0)[..5]
    }

    #[quickcheck]
    fn fios_ciosopt(a: Vec<u64>, b: Vec<u64>) -> bool {
        // Ensure vectors are length 4 by either truncating or padding with zeros
        let a: [u64; 4] = if a.len() >= 4 {
            a[..4].try_into().unwrap()
        } else {
            let mut padded = [0u64; 4];
            padded[..a.len()].copy_from_slice(&a);
            padded
        };

        let b: [u64; 4] = if b.len() >= 4 {
            b[..4].try_into().unwrap()
        } else {
            let mut padded = [0u64; 4];
            padded[..b.len()].copy_from_slice(&b);
            padded
        };

        fios(a, b, P, NP0)[..5] == cios_opt(a, b, P, NP0)[..5]
    }
}
