#![feature(portable_simd)]
use std::{
    hint::black_box,
    ops::Neg,
    simd::{f64x1, Simd, StdFloat},
};

fn main() {
    mul_generation();
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
