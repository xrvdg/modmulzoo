use num_traits::Unsigned;

fn main() {
    println!("Hello, world!");

    // These values are going to be outside of the allowed range
    // 255ˆ2 = 65025
    // Just see what it does and understand what the problem is
    let a = 25;
    let b = 25;

    let p: u32 = a * b;

    println!("p: {}", p);
    println!("p mod 97: {}", p % 97);
    println!("p mod 100: {}", p % 100);

    let red = montgomery_reduction(p, 97, 100);

    println!("montgomery reduction: {}", red);
    // How to undo the reduction properly?
    println!("montgomery reduction reversed: {}", red * 100 % 97);
    println!("montgomery reduction reversed: {}", red * (100 % 97));
    println!(
        "montgomery reduction reversed: {}",
        montgomery_reduction(red, 97, 100)
    );

    println!("");

    let red = montgomery_reduction_power_of_two(p, 97, 256);

    println!("montgomery reduction: {}", red);
    // How to undo the reduction properly?
    println!("montgomery reduction reversed: {}", red * 256 % 97);
    println!("montgomery reduction reversed: {}", red * (256 % 97));
    println!(
        "montgomery reduction reversed: {}",
        montgomery_reduction_power_of_two(red, 97, 256)
    );

    println!("");
    let a = 4;
    let b = 5;
    let p = a * b;

    println!("");
    let a = 25000;
    let b = 25000;

    let n = u8::MAX as u32 * 3;
    let r = u16::MAX as u32 + 1;
    println!("n: {} \t r: {}", n, r);
    let bytes: [u8; 4] = (a as u32).to_le_bytes();
    let values = [
        u16::from_le_bytes([bytes[0], bytes[1]]),
        u16::from_le_bytes([bytes[2], bytes[3]]),
    ];
    let red = montgomery_reduction_overflow(a * b, n, r);
    println!("montgomery reduction overflow: {}", red);
    let red = montgomery_reduction(a * b, n, r);
    println!("montgomery reduction: {}", red);
    let red = montgomery_reduction_base16(&values, b, n.try_into().unwrap(), r);
    println!("montgomery reduction base16: {}", red);
}

// This version works for any r
// p < nˆ2
// Don't understand the constraint on p
// looks like it for the bounding step, but for my numbers I haven't found a case where it's violated
// or is it for some integer bound
pub fn montgomery_reduction(p: u32, n: u32, r: u32) -> u32 {
    let u = montgomery_constant(n, r);
    // While it is unnecessary mathematically, it reduces the size of p

    // Would like to have widening and narrowing operations, but if you have the register size and it fills it already.
    // Things is that it was recommended on NEON to not do the widening yourself
    let q = u * (p % r) % r;
    // let q = u * p % r;
    let c = (p + n * q) / r;
    if c >= n {
        c - n
    } else {
        c
    }
}

pub fn montgomery_reduction_base16(a: &[u16], b: u32, n: u16, r: u32) -> u16 {
    let u = montgomery_constant(n as u32, r);
    // While it is unnecessary mathematically, it reduces the size of p

    // Would like to have widening and narrowing operations, but if you have the register size and it fills it already.
    // Things is that it was recommended on NEON to not do the widening yourself

    let mut c: u32 = 0;
    for i in 0..a.len() - 1 {
        c = c + a[i] as u32 * b;
        let q = (u % r) * (c % r) % r;
        // let q = u * p % r;
        c = (c + n as u32 * q) / r;
    }

    if c >= n as u32 {
        (c - (n as u32)) as u16
    } else {
        c as u16
    }
}

pub fn montgomery_reduction_overflow(p: u32, n: u32, r: u32) -> u32 {
    let u = montgomery_constant(n, r);
    // While it is unnecessary mathematically, it reduces the size of p

    // Would like to have widening and narrowing operations, but if you have the register size and it fills it already.
    // Things is that it was recommended on NEON to not do the widening yourself
    let q = (u.overflowing_mul(p).0) % r;
    // let q = u * p % r;
    let c = (p + n * q) / r;
    if c >= n {
        c - n
    } else {
        c
    }
}

// Do I ever need to reduce things in parallel?
// Do I ever need to explicitly reduce or can it be done in the multiplication step?

//blackbox compare with a power of two version?
// interesting that this is slower
pub fn montgomery_reduction_power_of_two(p: u32, n: u32, r: u32) -> u32 {
    // wouldn't lcnt be faster?
    if r.count_ones() != 1 {
        panic!("r must be a power of two");
    };
    let u = montgomery_constant(n, r);
    let bit_position = r.trailing_zeros();
    let mask = r - 1;
    // While it is unnecessary mathematically, it reduces the size of p
    let q = (u * (p & mask)) & mask;
    // let q = u * p % r;
    let c = (p + n * q) >> bit_position;
    if c >= n {
        c - n
    } else {
        c
    }
}

// how to unreduce?

// Better way to compute the inverse?
// r > n
// output and inputs are maxed out by r
fn montgomery_constant<T: Unsigned + Copy>(n: T, r: T) -> T {
    let mut u = T::one();
    // find inverse of n
    while ((u) * (n)) % r != T::one() {
        u = u + T::one();
    }
    // negate
    r - u
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        // Add your test cases here
    }
}
