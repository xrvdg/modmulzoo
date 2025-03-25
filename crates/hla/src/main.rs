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
pub fn carry_add(asm: &mut Assembler, s: [Reg<u64>; 2], add: Reg<u64>) -> [Reg<u64>; 2] {
    asm.append_instruction(vec![
        adds_inst(&s[0], &s[0], &add),
        cinc_inst(&s[1], &s[1], "hs".to_string()),
    ]);
    s
}

fn interleave_test() {
    // doesn't fully do the indirect result register
    let mut alloc = Allocator::new();
    let mut mapping = RegisterMapping::new();
    let mut phys_registers = RegisterBank::new();

    // First round
    let mut asm = Assembler::new();
    let b = input(&mut alloc, &mut mapping, &mut phys_registers, 0);
    let a_regs = array::from_fn(|ai| (1 + ai as u64));
    let a = a_regs.map(|pr| input(&mut alloc, &mut mapping, &mut phys_registers, pr));

    let s = smult(&mut alloc, &mut asm, a, b);
    println!("{:?}", alloc);

    let first = asm.instructions;

    // Second round
    let mut asm = Assembler::new();
    let b = input(&mut alloc, &mut mapping, &mut phys_registers, 5);
    let a_regs = array::from_fn(|ai| (6 + ai as u64));
    let a = a_regs.map(|pr| input(&mut alloc, &mut mapping, &mut phys_registers, pr));
    let p = smult(&mut alloc, &mut asm, a, b);
    let second = asm.instructions;

    let mix = interleave(first, second);

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

fn build_smul() {
    // doesn't fully do the indirect result register
    let mut alloc = Allocator::new();
    let mut mapping = RegisterMapping::new();
    let mut phys_registers = RegisterBank::new();

    // First round
    let mut asm = Assembler::new();
    let b = input(&mut alloc, &mut mapping, &mut phys_registers, 0);
    let a_regs = array::from_fn(|ai| (1 + ai as u64));
    let a = a_regs.map(|pr| input(&mut alloc, &mut mapping, &mut phys_registers, pr));

    let s = smult(&mut alloc, &mut asm, a, b);
    println!("{:?}", alloc);

    let first: Vec<_> = asm.instructions.into_iter().flatten().collect();

    // Is there something we can do to tie off the outputs.
    // and to make sure it happens before drop_pass
    let mut seen = Seen::new();
    s.iter().for_each(|r| {
        seen.output_interface(r);
    });

    let releases = liveness_analysis(&mut seen, &first);

    let out = hardware_register_allocation(&mut mapping, &mut phys_registers, first, releases);
    let mut file = std::fs::File::create("./asm/global_asm_smul.s").expect("Unable to create file");
    let txt = backend_global("smul".to_string(), out);
    s.iter().for_each(|r| {
        println!("{}", mapping.output_register(r));
    });
    use std::io::Write;
    file.write_all(txt.as_bytes())
        .expect("Unable to write data to file");
}

fn simd_test() {
    let mut alloc = Allocator::new();
    let mut asm = Assembler::new();
    let mut mapping = RegisterMapping::new();
    let mut phys_registers = RegisterBank::new();

    let v_regs = array::from_fn(|ai| (ai as u64));
    let v = v_regs.map(|pr| input(&mut alloc, &mut mapping, &mut phys_registers, pr));
    let s = input(
        &mut alloc,
        &mut mapping,
        &mut phys_registers,
        v.len() as u64,
    );
    let t = smult_noinit_simd(&mut alloc, &mut asm, s, v);
    println!("\nssimd");
    let ssimd = asm.instructions;
    let inst: Vec<_> = ssimd.into_iter().flatten().collect();
    print_instructions(&inst);

    let mut seen = Seen::new();
    seen.output_interface(&t);
    let releases = liveness_analysis(&mut seen, &inst);
    let out = hardware_register_allocation(&mut mapping, &mut phys_registers, inst, releases);

    println!();
    print_instructions(&out);
}

// global_asm!(include_str!("../asm/mulu128.s"));

global_asm!(include_str!("../asm/global_asm_smul.s"));
// Doesn't work
// fn inline_asm() {
//     unsafe { asm!(include_str!("../asm/asm_test.s")) };
// }

fn gen_mulu128(
    alloc: &mut Allocator,
    asm: &mut Assembler,
    a: &Reg<u64>,
    b: &Reg<u64>,
) -> [Reg<u64>; 2] {
    [mul(alloc, asm, a, b), umulh(alloc, asm, a, b)]
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
    let mut alloc = Allocator::new();
    let mut asm = Assembler::new();
    let mut mapping = RegisterMapping::new();
    let mut register_bank = RegisterBank::new();
    let a = input(&mut alloc, &mut mapping, &mut register_bank, 0);
    let b = input(&mut alloc, &mut mapping, &mut register_bank, 1);

    let ret = gen_mulu128(&mut alloc, &mut asm, &a, &b);

    let mut seen_registers = Seen::new();
    ret.iter().for_each(|r| {
        seen_registers.output_interface(r);
    });
    let inst: Vec<_> = asm.instructions.into_iter().flatten().collect();

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
    // let r = struct_return(
    //     std::hint::black_box(3),
    //     std::hint::black_box(3),
    //     std::hint::black_box(3),
    //     std::hint::black_box(6),
    // );
    // // println!("r: {r:?}");
    // // let r = call_mulu128(5, 6);
    // // println!("r: {r:?}");
    // // let r = inline_call_mulu128(5, 6);
    // // println!("r: {r:?}");
    // build_mulu128();
    // interleave_test();
    // simd_test();
    build_smul();
    let r = call_smul([1, 2, 3, 4], 5);
    println!("r: {r:?}");
}

#[inline(never)]
fn call_smul(a: [u64; 4], b: u64) -> [u64; 5] {
    let mut out = [0; 5];
    unsafe {
        asm!(
            "bl _smul",
            in("x0") b, in("x1") a[0], in("x2") a[1], in("x3") a[2], in("x4") a[3],
            lateout("x5") out[0], lateout("x1") out[1], lateout("x2") out[2], lateout("x3") out[3], lateout("x0") out[4]
        )
    };
    out
}

// How do other allocating algorithms pass things along like Vec?
// In this algorithm the inputs are not used after
pub fn smult(
    alloc: &mut Allocator,
    asm: &mut Assembler,
    a: [Reg<u64>; 4],
    b: Reg<u64>,
) -> [Reg<u64>; 5] {
    let t0 = mul(alloc, asm, &a[0], &b);
    let t1 = umulh(alloc, asm, &a[0], &b);
    //
    let tmp = mul(alloc, asm, &a[1], &b);
    let t2 = umulh(alloc, asm, &a[1], &b);
    let [t1, t2] = carry_add(asm, [t1, t2], tmp);
    //
    let tmp = mul(alloc, asm, &a[2], &b);
    let t3 = umulh(alloc, asm, &a[2], &b);
    let [t2, t3] = carry_add(asm, [t2, t3], tmp);
    //
    let tmp = mul(alloc, asm, &a[3], &b);
    let t4 = umulh(alloc, asm, &a[3], &b);
    let [t3, t4] = carry_add(asm, [t3, t4], tmp);
    [t0, t1, t2, t3, t4]
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
    alloc: &mut Allocator,
    asm: &mut Assembler,
    s: Reg<Simd<u64, 2>>,
    v: [Reg<u64>; 5],
) -> Reg<Simd<f64, 2>> {
    // first do it as is written
    let s = ucvtf2d(alloc, asm, &s);

    let tmp = mov(alloc, asm, C1.to_bits());
    let v0 = ucvtf(alloc, asm, &v[0]);
    let splat_c1 = dup2d(alloc, asm, &tmp);
    let cc1 = mov16b(alloc, asm, &splat_c1);
    let t0 = fmla2d(alloc, asm, cc1.into_(), &s, &v0.as_simd(), 0);
    t0
}

#[cfg(test)]
mod tests {
    use montgomery_reduction::arith;
    use quickcheck_macros::quickcheck;

    use crate::call_smul;

    #[quickcheck]
    fn smul(a0: u64, a1: u64, a2: u64, a3: u64, b: u64) -> bool {
        let a = [a0, a1, a2, a3];
        arith::smul(b, a) == call_smul(a, b)
    }
}
