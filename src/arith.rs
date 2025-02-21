/// All these method operate on b64
/// TODO: generalise these functions to work on b52 without performance loss for either b52 or b64

pub fn school_method(a: [u64; 4], b: [u64; 4]) -> [u64; 8] {
    let mut ab = [0_u64; 8];
    for i in 0..a.len() {
        let mut carry = 0;
        for j in 0..b.len() {
            (ab[i + j], carry) = carrying_mul_add(a[i], b[j], ab[i + j], carry)
        }
        ab[i + b.len()] = carry;
    }
    ab
}

// first is result, second is carry (S,C)
// This order is common in the Rust std library, but it differs from the order common in papers.
// Could have used the intrinsic version, but this doesn't give any warnings
#[inline(always)]
pub fn carrying_mul_add(a: u64, b: u64, add: u64, carry: u64) -> (u64, u64) {
    let c: u128 = a as u128 * b as u128 + carry as u128 + add as u128;
    (c as u64, (c >> 64) as u64)
}

#[inline(always)]
pub fn carrying_mul_add_sat(a: u64, b: u64, add: u64, carry: u64) -> (u64, u64) {
    let c: u128 = a as u128 * b as u128 + carry as u128 + add as u128;
    (c as u64, (c >> 64) as u64)
}

#[inline(always)]
pub fn carry_add(lhs: u64, carry: u64) -> (u64, u64) {
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

// TODO: Generalize this so that it can deal with multiple
#[inline]
pub fn subtraction_step<const N: usize>(a: [u64; N], b: [u64; N]) -> [u64; N] {
    let mut borrow: i64 = 0;
    let mut c = [0; N];
    for i in 0..N {
        let tmp = a[i] as i128 - b[i] as i128 + borrow as i128;
        c[i] = tmp as u64;
        borrow = (tmp >> 64) as i64
    }

    if borrow != 0 {
        a
    } else {
        c
    }
}

/// Modulus operation by repeatedly performing a:=(a-b) until a < b
pub fn modulus<const N: usize>(a: [u64; N], b: [u64; N]) -> [u64; N] {
    let mut d = a;
    let mut prev = d;
    loop {
        d = subtraction_step(d, b);
        if d == prev {
            break;
        }
        prev = d;
    }
    d
}
