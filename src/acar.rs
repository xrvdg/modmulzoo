// This implements most of the modular multiplication algorithms listed in Acar97
pub type U256 = [u64; 4];

// first is result, second is carry (S,C)
// This order is common in the Rust std library, but it differs from the order common in papers.
#[inline(always)]
fn carrying_mul_add(a: u64, b: u64, add: u64, carry: u64) -> (u64, u64) {
    let c: u128 = a as u128 * b as u128 + carry as u128 + add as u128;
    (c as u64, (c >> 64) as u64)
}

// direct translation of montgomery multiplication.
// Goal is to serve as a reference
// untested
fn naive(a: U256, b: U256, n: U256, np: U256) -> Vec<u64> {
    let mut t = vec![0_u64; 8];

    // multiplication a * b
    for i in 0..a.len() {
        let mut carry = 0;
        for j in 0..b.len() {
            (t[i + j], carry) = carrying_mul_add(a[i], b[j], t[i + j], carry)
        }
        t[i + b.len()] = carry;
    }

    let mut m = [0_u64; 4];
    // m = TN' mod R
    for i in 0..np.len() {
        let mut carry = 0;
        for j in 0..(np.len() - i) {
            (m[i + j], carry) = carrying_mul_add(n[i], t[j], m[i + j], carry);
        }
        // Due to this multiplication being done mod R there is no final carry placement as in
        // the above a*b multiplication
    }

    // m*n
    let mut nm = [0_u64; 8];
    for i in 0..n.len() {
        let mut carry = 0;
        for j in 0..m.len() {
            (nm[i + j], carry) = carrying_mul_add(n[i], m[j], nm[i + j], carry);
        }
        nm[i + m.len()] = carry;
    }

    // By splitting up the multiplication of m*n from the addition of t. There is no need for [adds]
    // t + m*n
    let mut out = [0_u64; 9];
    for i in 0..nm.len() {
        // Could also directly assign it to a windowing slice
        let r = t[i] as u128 + nm[i] as u128 + out[i] as u128;
        (out[i], out[i + 1]) = (r as u64, (r >> 64) as u64);
    }

    t
}

// SOS is like the naive version by fusing t + m*n
// benefit is that we can calculate a single m
pub fn sos(a: U256, b: U256, n: U256, np0: u64) -> [u64; 8] {
    let mut t = [0_u64; 8];

    // multiplication a * b
    for i in 0..a.len() {
        let mut carry = 0;
        for j in 0..b.len() {
            (t[i + j], carry) = carrying_mul_add(a[i], b[j], t[i + j], carry)
        }
        t[i + b.len()] = carry;
    }

    // t * m+n in a single loop
    // Iterate over the size. Using a.len() here to illustrate the outer loop fusion
    for i in 0..a.len() {
        let mut carry = 0;
        let m = t[i].wrapping_mul(np0);
        // iterating of n
        for j in 0..n.len() {
            (t[i + j], carry) = carrying_mul_add(m, n[j], t[i + j], carry)
        }
        // When multiplying a*b t[i+n.len()] is still empty so we can 'override' it with the content of the carry.
        // Here t[i+n.len()] already has a value and adding to that value might overflow. Therefore
        // we need to push the carry through all the way
        adds(&mut t[(i + n.len())..], carry)
    }

    t
}

// CIOS
// - SOS where the outer loop has been fused
// - and due to shifting we have a fresh spot for the last carry by which we don't need
// adds
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

        // Division by the small modulus w
        for j in 0..t.len() - 1 {
            t[j] = t[j + 1]
        }
    }
    t
}

// cios_opt is cios where the division is combined with the multiplication
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
        // Last shift can probably be skipped. This brings it very close to Yuval's numbers
        (t[n.len()], _) = carry_add(t[n.len() + 1], carry);
    }
    t
}

// FIOS is like CIOS where the inner loops for a*b and t+m*n fused.
// Due to this fusion the there are too many adds to fill the free spaces after multiplication
// Therefore adds is needed again
pub fn fios(a: U256, b: U256, n: U256, np0: u64) -> [u64; 6] {
    let mut t = [0_u64; 6];
    for i in 0..a.len() {
        let (sum, mut carry) = carrying_mul_add(a[i], b[0], t[0], 0);
        adds(&mut t[1..], carry);
        let m = sum.wrapping_mul(np0);
        let _sum;
        (_sum, carry) = carrying_mul_add(m, n[0], sum, 0);

        // Iterate over both b and n. No particular reason to choose b over n
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

#[inline(always)]
fn carry_add(lhs: u64, carry: u64) -> (u64, u64) {
    let (sum, carry) = lhs.overflowing_add(carry);
    (sum, carry.into())
}

#[inline(always)]
pub fn adds(t: &mut [u64], mut carry: u64) {
    for i in 0..t.len() {
        // Performance drops heavily when introducing this check
        // if carry == 0 {
        //     break;
        // }
        let b;
        (t[i], b) = t[i].overflowing_add(carry);
        // Add if to exit
        carry = b.into();
    }
}

#[cfg(test)]
mod tests {
    // Test if they actually give back the same result
    use super::*;
    use crate::{subtraction_step, NP0, P, R2}; // Import constants from the crate root
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn cios_round(a: Vec<u64>) -> bool {
        // Ensure vectors are length 4 by either truncating or padding with zeros
        let a: [u64; 4] = if a.len() >= 4 {
            a[..4].try_into().unwrap()
        } else {
            let mut padded = [0u64; 4];
            padded[..a.len()].copy_from_slice(&a);
            padded
        };

        // Montgomery form
        let a_tilde: [u64; 4] = cios(a, R2, P, NP0)[..4].try_into().unwrap();
        // Invert
        let a_round: [u64; 4] = cios(a_tilde, [1, 0, 0, 0], P, NP0)[..4].try_into().unwrap();

        // When we generate the input it isn't modulo N so to compare the result we have to do it here
        // We don't do it earlier as the algorithm should work with values outside of the modulus
        let mut d = a;
        let mut prev = d;
        loop {
            d = subtraction_step(d, P);
            if d == prev {
                break;
            }
            prev = d;
        }

        d == subtraction_step(a_round, P)
    }

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
