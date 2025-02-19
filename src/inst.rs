#![feature(portable_simd)]
use std::{
    hint::black_box,
    ops::Neg,
    simd::{f64x1, Simd, StdFloat},
};

use num_traits::MulAdd;

fn main() {
    mul_generation();
    u_generation();
}

fn mul_generation() {
    let a: f64 = black_box(4.0);
    let b = black_box(2.0);
    let c = black_box(3.0);
    let d = a * b + c;
    println!("{}", d);
    // Floating points are not optimized to use fma
    //inst[0x100001aa0] <+68>:  fmul   d0, d0, d1
    //inst[0x100001aa4] <+72>:  fadd   d0, d0, d2

    // This does add the intrinsic
    // Using mul_add may be more performant than an unfused multiply-add if the target architecture has a dedicated fma CPU instruction. However, this is not always true, and will be heavily dependant on designing algorithms with specific target hardware in mind.
    // Should give the inst
    let a: f64 = black_box(4.0);
    let b = black_box(2.0);
    let c = black_box(3.0);
    let e: f64 = a.mul_add(b, c);
    println!("{}", e);

    // Does this give fmsub? No it gives fnmsub which is the correct instruction. It's just named a bit confusing
    let f: f64 = a.mul_add(b, -1.0 * c);
    println!("{}", f);

    // This gives fmsub
    let g: f64 = a.mul_add(-1.0 * b, c);
    println!("{}", g);

    // first a multiplication and then add
    let h: f64 = a.mul_add(b, -2.0 * c);
    println!("{}", h);

    let a: f64x1 = black_box(f64x1::splat(a));
    let b: f64x1 = black_box(f64x1::splat(b));
    let c: f64x1 = black_box(f64x1::splat(c));

    // fmadd
    let i = a.mul_add(b, c);
    println!("{:?}", i);

    // fmnsub
    let j = a.mul_add(b, f64x1::splat(-1.0) * c);
    println!("{:?}", j);

    // fmsub
    let l = a.mul_add(f64x1::splat(-1.0) * b, c);
    println!("{:?}", l);

    // This is tricky to use on singles
    // The portable simd is a nicer programming experience, but it doesn't give you the sub directly. Still I think with the
    // naming it is confusing to do otherwise
    // let f = unsafe { vfma_f64(a.into(), b.into(), c) };
    // println!("{}", f);
    let a: f64 = black_box(4.0);
    let b = black_box(2.0);
    let c = black_box(3.0);

    let a = black_box(Simd::<_, 2>::splat(a));
    let b = black_box(Simd::splat(b));
    let c = black_box(Simd::splat(c));

    // fmla, why not fmlal?
    let m = a.mul_add(b, c);
    println!("{:?}", m);

    // fneg followed by fmla
    // both give the same result.
    // let n = a.mul_add(b, Simd::splat(-1.0) * c);
    let n = a.mul_add(b, c.neg());
    println!("{:?}", n);

    // fmls
    let k = a.mul_add(Simd::splat(-1.0) * b, c);
    println!("{:?}", k);

    // Doing singles and doing doubles
}

// Redeclares a b c to not have any sharing between the calculations
// SIMD by default does wrapping multiplication
fn u_generation() {
    let a: u64 = black_box(4);
    let b: u64 = black_box(2);
    let c: u64 = black_box(3);

    // It does make use of madd, but it didn't in a previous version it shared with the next calculation and that caused
    // the mul to be split
    let d = a * b + c;
    println!("{}", d);

    let a: u64 = black_box(6);
    let b: u64 = black_box(7);
    let c: u64 = black_box(8);
    let d: u64 = black_box(9);

    // umulh, mul, adds, cinc for a*b+c
    // umulh, mul, adds, cinc, adds, cinc for a*b+c+d
    let e = a as u128 * b as u128 + c as u128 + d as u128;
    // Does it handle the splitting cleanly? It does
    let (el, eh) = (e as u64, (e >> 64) as u64);
    println!("{} {} {}", e, el, eh);

    let a: u32 = black_box(6);
    let b: u32 = black_box(7);
    let c: u32 = black_box(8);
    let d: u32 = black_box(9);

    // umaddl, add
    let e2 = a as u64 * b as u64 + c as u64 + d as u64;
    // Does it handle the splitting cleanly? It does
    let (el2, eh2) = (e as u32, (e >> 32) as u32);
    println!("{} {} {}", e2, el2, eh2);

    let a: u64 = black_box(9);
    let b: u64 = black_box(10);
    let c: u64 = black_box(11);

    // The mul_add here com from num_traits a package outside of std

    // Uses madd
    let f: u64 = a.mul_add(b, c);
    println!("{}", f);

    let a: i64 = black_box(9);
    let b: i64 = black_box(10);
    let c: i64 = black_box(11);

    // neg madd
    let g = a.mul_add(b, -c);
    println!("{}", g);

    // msub
    let h = a.mul_add(-b, c);
    println!("{}", h);

    // What we need are widening mulls which the portable SIMD library doesn't support
    // This could be a nice addition to write
}
