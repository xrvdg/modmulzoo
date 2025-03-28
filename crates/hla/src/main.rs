use std::{
    arch::{asm, global_asm},
    array,
};

use hla::*;

// In this case we know that carry_add only needs to propagate 2
// but in other situations that is not the case.
// Seeing this ahead might be nice
// with a parameter and then use slice and generalize it
// Not everything has to have perfect types
pub fn carry_add(s: [&Reg<u64>; 2], add: &Reg<u64>) -> AtomicInstruction {
    vec![adds(s[0], s[0], add), cinc(s[1], s[1], "hs")]
        .into_iter()
        .flatten()
        .collect()
}

fn interleave_test() {
    // doesn't fully do the indirect result register
    let mut asm = Allocator::new();
    let mut mapping = RegisterMapping::new();
    let mut phys_registers = RegisterBank::new();

    // Map how the element are mapped to physical registers
    // This needs in to be in part of the code that can talk about physical registers
    // Could structure this differently such that it gives a fresh reg
    let b = input(&mut asm, &mut mapping, &mut phys_registers, 0);
    let a_regs = array::from_fn(|ai| (1 + ai as u64));
    let a = a_regs.map(|pr| input(&mut asm, &mut mapping, &mut phys_registers, pr));

    let s: [Reg<u64>; 5] = array::from_fn(|_| asm.fresh());

    let sinst = smult(&mut asm, &s, a, b);
    println!("{:?}", asm);

    let old = sinst;

    let b = input(&mut asm, &mut mapping, &mut phys_registers, 5);
    let a_regs = array::from_fn(|ai| (6 + ai as u64));
    let a = a_regs.map(|pr| input(&mut asm, &mut mapping, &mut phys_registers, pr));
    let p: [Reg<u64>; 5] = array::from_fn(|_| asm.fresh());
    let p_inst = smult(&mut asm, &p, a, b);
    let new = p_inst;

    let mix = interleave(old, new);

    // Is there something we can do to tie off the outputs.
    // and to make sure it happens before drop_pass
    let mut seen = Seen::new();
    s.iter().for_each(|r| {
        seen.output_interface(r);
    });
    p.iter().for_each(|r| {
        seen.output_interface(r);
    });
    let releases = liveness_analysis(&mut seen, &mix);
    println!("\nmix: {mix:?}");

    // Mapping and phys_registers seem to go togetehr
    let out = hardware_register_allocation(&mut mapping, &mut phys_registers, mix, releases);
    print_instructions(&out);
}

fn simd_test() {
    let mut asm = Allocator::new();
    let mut mapping = RegisterMapping::new();
    let mut phys_registers = RegisterBank::new();

    let t_regs = array::from_fn(|ai| (ai as u64));
    let t = t_regs.map(|pr| input(&mut asm, &mut mapping, &mut phys_registers, pr));
    let v_regs = array::from_fn(|ai| (ai as u64));
    let v = v_regs.map(|pr| input(&mut asm, &mut mapping, &mut phys_registers, pr));
    let s = input(&mut asm, &mut mapping, &mut phys_registers, t.len() as u64);
    let ssimd = smult_noinit_simd(&mut asm, &t, s, v);
    println!("\nssimd");
    let inst: Vec<_> = ssimd.into_iter().flatten().collect();
    print_instructions(&inst);

    let mut seen = Seen::new();
    t.iter().for_each(|r| {
        seen.output_interface(r);
    });
    let releases = liveness_analysis(&mut seen, &inst);
    let out = hardware_register_allocation(&mut mapping, &mut phys_registers, inst, releases);

    println!();
    print_instructions(&out);
}

// global_asm!(include_str!("../asm/mulu128.s"));

// Doesn't work
// fn inline_asm() {
//     unsafe { asm!(include_str!("../asm/asm_test.s")) };
// }

fn gen_mulu128(c: &[Reg<u64>; 2], a: &Reg<u64>, b: &Reg<u64>) -> Vec<Instruction> {
    vec![mul(&c[0], a, b), umulh(&c[1], a, b)]
        .into_iter()
        .flatten()
        .collect()
}

#[inline(never)]
fn call_mulu128(a: u64, b: u64) -> u128 {
    let mut lo: u64;
    let mut hi: u64;
    // For now hard code since it only generated every now and then
    unsafe { asm!("bl _mulu128", in("x0") a, in("x1") b, out("x2") lo, out("x3") hi) };
    (hi as u128) << 64 | lo as u128
}

// This might be the best approach to include it into Rust, but depends on if it destroys the order
#[inline(never)]
fn inline_call_mulu128(a: u64, b: u64) -> u128 {
    let mut lo: u64;
    let mut hi: u64;
    // For now hard code since it only generated every now and then
    unsafe {
        asm!(r#"
    mul x2, x0,x1
    umulh x3, x0, x1
    "#, in("x0") a, in("x1") b, out("x2") lo, out("x3") hi)
    };
    (hi as u128) << 64 | lo as u128
}

fn build_mulu128() {
    let mut asm = Allocator::new();
    let mut mapping = RegisterMapping::new();
    let mut register_bank = RegisterBank::new();
    let a = input(&mut asm, &mut mapping, &mut register_bank, 0);
    let b = input(&mut asm, &mut mapping, &mut register_bank, 1);
    let ret = array::from_fn(|_| asm.fresh());

    let inst = gen_mulu128(&ret, &a, &b);

    let mut seen_registers = Seen::new();
    ret.iter().for_each(|r| {
        seen_registers.output_interface(r);
    });
    let releases = liveness_analysis(&mut seen_registers, &inst);
    let physical_inst =
        hardware_register_allocation(&mut mapping, &mut register_bank, inst, releases);
    print_instructions(&physical_inst);
    ret.iter()
        .for_each(|r| println!("{}", mapping.output_register(r)));
}
#[derive(Debug)]
#[repr(C)]
pub struct U128S {
    lo: u64,
    hi: u64,
}

#[derive(Debug)]
#[repr(transparent)]
pub struct U128L([u64; 3]);

#[inline(never)]
pub extern "C" fn struct_return(a: u64, c: u64, d: u64, b: u64) -> U128L {
    let lo = a * b;
    let hi = c * b;
    let c = d * b;
    U128L([lo, hi, lo - c])
}

fn main() {
    let r = struct_return(
        std::hint::black_box(3),
        std::hint::black_box(3),
        std::hint::black_box(3),
        std::hint::black_box(6),
    );
    println!("r: {r:?}");
    let r = call_mulu128(5, 6);
    println!("r: {r:?}");
    let r = inline_call_mulu128(5, 6);
    println!("r: {r:?}");
    build_mulu128();
    interleave_test();
    simd_test();
}

// How do other allocating algorithms pass things along like Vec?
// In this algorithm the inputs are not used after
pub fn smult(
    asm: &mut Allocator,
    s: &[Reg<u64>; 5],
    a: [Reg<u64>; 4],
    b: Reg<u64>,
) -> Vec<AtomicInstruction> {
    // tmp being reused instead of a fresh variable each time.
    // should not make much of a difference
    let tmp = asm.fresh();
    vec![
        mul(&s[0], &a[0], &b),
        umulh(&s[1], &a[0], &b),
        //
        mul(&tmp, &a[1], &b),
        umulh(&s[2], &a[1], &b),
        carry_add([&s[1], &s[2]], &tmp),
        //
        mul(&tmp, &a[2], &b),
        umulh(&s[3], &a[2], &b),
        carry_add([&s[2], &s[3]], &tmp),
        //
        mul(&tmp, &a[3], &b),
        umulh(&s[4], &a[3], &b),
        carry_add([&s[3], &s[4]], &tmp),
    ]
}

#[inline(never)]
pub extern "C" fn test_input(a: [u64; 4], b: u64) -> [u64; 5] {
    let mut out = [0; 5];
    let mut sum = 0;
    for (i, ai) in a.iter().enumerate() {
        sum += ai;
        out[i] = ai * b;
    }
    out[4] = sum;
    out
}

#[inline(never)]
pub extern "C" fn c_test_input(v: *const u64, size: u64, s: u64) {}

// TODO initiliase constant
const C1: f64 = 0.;

// Whole vector is in registers, but that might not be great. Better to have it on the stack and load it from there
pub fn smult_noinit_simd(
    asm: &mut Allocator,
    _t: &[Reg<Simd<u64, 2>>; 6],
    s: Reg<Simd<u64, 2>>,
    v: [Reg<u64>; 5],
) -> Vec<AtomicInstruction> {
    // first do it as is written
    let tmp = asm.fresh();
    let splat_c1 = asm.fresh();
    let cc1 = asm.fresh();
    let fv0: Reg<Simd<u64, 2>> = asm.fresh();
    vec![
        ucvtf2d(&s, &s),
        mov(&tmp, C1.to_bits()),
        ucvtf(fv0.as_f64(), &v[0]),
        dup2d(&splat_c1, &tmp),
        mov16b(&cc1, &splat_c1),
        fmla2d(&cc1, &s, &fv0, 0),
    ]
}
