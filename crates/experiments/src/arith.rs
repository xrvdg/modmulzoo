/// All these method operate on b64

#[inline]
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

pub fn smul(s: u64, v: [u64; 4]) -> [u64; 5] {
    let mut ab = [0_u64; 5];
    for j in 0..v.len() {
        (ab[j], ab[j + 1]) = carrying_mul_add(s, v[j], ab[j], 0)
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

/// Adds two u64 arrays together, treating them as multi-precision integers
///
/// # Arguments
///
/// * `a` - First multi-precision integer
/// * `b` - Second multi-precision integer
///
/// # Returns
///
/// The sum of the two multi-precision integers with carry propagation
pub fn addv<const N: usize>(mut a: [u64; N], b: [u64; N]) -> [u64; N] {
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
pub fn sub<const N: usize>(a: [u64; N], b: [u64; N]) -> [u64; N] {
    let mut borrow: i128 = 0;
    let mut c = [0; N];
    for i in 0..N {
        let tmp = a[i] as i128 - b[i] as i128 + borrow as i128;
        c[i] = tmp as u64;
        borrow = tmp >> 64
    }
    c
}

#[inline]
/// returns a if a < b else return a - b
/// single step of modulo operation
pub fn modulus_subtraction_step<const N: usize>(a: [u64; N], b: [u64; N]) -> [u64; N] {
    let mut borrow: i128 = 0;
    let mut c = [0; N];
    for i in 0..N {
        let tmp = a[i] as i128 - b[i] as i128 + borrow as i128;
        c[i] = tmp as u64;
        borrow = tmp >> 64
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
        d = modulus_subtraction_step(d, b);
        if d == prev {
            break;
        }
        prev = d;
    }
    d
}
