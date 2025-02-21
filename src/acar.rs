use crate::arith::{adds, carry_add, carrying_mul_add};

// This implements most of the modular multiplication algorithms listed in Acar97
pub type U256 = [u64; 4];

// direct translation of montgomery multiplication.
// Goal is to serve as a reference
// untested
#[allow(dead_code)]
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

use seq_macro::seq;
// cios_opt is cios where the division is combined with the multiplication
#[inline(always)]
pub fn cios_opt_seq(a: U256, b: U256, n: U256, np0: u64) -> [u64; 6] {
    let mut t = [0_u64; 6];
    seq! (i in 0..4
        {
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
    );
    t
}

// Does not provide any improvements
// Likely because both multipliers are used for the 128 multiplication
pub fn cios_opt_sat(a: U256, b: U256, c: U256, d: U256, n: U256, np0: u64) -> [[u64; 6]; 2] {
    let mut out = [[0; 6]; 2];

    for i in 0..a.len() {
        let mut carry = 0;
        let mut carry_cd = 0;
        for j in 0..b.len() {
            (out[0][j], carry) = carrying_mul_add(a[i], b[j], out[0][j], carry);
            (out[1][j], carry_cd) = carrying_mul_add(c[i], d[j], out[1][j], carry_cd);
        }
        (out[0][b.len()], out[0][b.len() + 1]) = carry_add(out[0][b.len()], carry);
        (out[1][d.len()], out[1][d.len() + 1]) = carry_add(out[1][d.len()], carry_cd);

        let mut carry = 0;
        let m = out[0][0].wrapping_mul(np0);
        (out[0][0], carry) = carrying_mul_add(m, n[0], out[0][0], carry);

        let mut carry_cd = 0;
        let m_cd = out[1][0].wrapping_mul(np0);
        (out[1][0], carry_cd) = carrying_mul_add(m_cd, n[0], out[1][0], carry_cd);

        for j in 1..n.len() {
            (out[0][j - 1], carry) = carrying_mul_add(m, n[j], out[0][j], carry);
            (out[1][j - 1], carry_cd) = carrying_mul_add(m_cd, n[j], out[1][j], carry_cd);
        }
        (out[0][n.len() - 1], carry) = carry_add(out[0][n.len()], carry);
        (out[0][n.len()], _) = carry_add(out[0][n.len() + 1], carry);
        (out[1][n.len() - 1], carry_cd) = carry_add(out[1][n.len()], carry_cd);
        (out[1][n.len()], _) = carry_add(out[1][n.len() + 1], carry_cd);
    }

    out
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        arith::{modulus, subtraction_step},
        gen::U256b64,
        NP0, P, R2,
    };
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    /// Test whether montgomery multiplication gives the same result as repeatedly subtraction
    fn cios_roundtrip(a: U256b64) -> bool {
        let a = a.0;
        // Montgomery form
        let a_tilde: [u64; 4] = cios(a, R2, P, NP0)[..4].try_into().unwrap();
        // and back
        let a_round: [u64; 4] = cios(a_tilde, [1, 0, 0, 0], P, NP0)[..4].try_into().unwrap();

        let d = modulus(a, P);

        d == subtraction_step(a_round, P)
    }

    // All remaining tests check equivalence with cios
    #[quickcheck]
    fn cios_sos(a: U256b64, b: U256b64) -> bool {
        let a = a.0;
        let b = b.0;

        cios(a, b, P, NP0)[..4] == sos(a, b, P, NP0)[4..]
    }

    #[quickcheck]
    fn cios_ciosopt(a: U256b64, b: U256b64) -> bool {
        let a = a.0;
        let b = b.0;
        cios(a, b, P, NP0)[..5] == cios_opt(a, b, P, NP0)[..5]
    }

    #[quickcheck]
    fn cios_ciosoptseq(a: U256b64, b: U256b64) -> bool {
        let a = a.0;
        let b = b.0;
        cios(a, b, P, NP0)[..5] == cios_opt_seq(a, b, P, NP0)[..5]
    }

    #[quickcheck]
    fn fios_ciosopt(a: U256b64, b: U256b64) -> bool {
        let a = a.0;
        let b = b.0;
        fios(a, b, P, NP0)[..5] == cios_opt(a, b, P, NP0)[..5]
    }

    #[quickcheck]
    fn ciossat_ciosopt(a: U256b64, b: U256b64, c: U256b64, d: U256b64) -> bool {
        let a = a.0;
        let b = b.0;
        let c = c.0;
        let d = d.0;

        let res = cios_opt_sat(a, b, c, d, P, NP0);

        res[0][..5] == cios_opt(a, b, P, NP0)[..5] && res[1][..5] == cios_opt(c, d, P, NP0)[..5]
    }
}
