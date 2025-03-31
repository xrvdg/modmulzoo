#![feature(iter_intersperse)]
use std::{arch::global_asm, array};

use hla::*;
use montgomery_reduction::yuval::{U64_2P, U64_I1, U64_I2, U64_I3, U64_MU0, U64_P};

// adds can be confusng as it has a similar shape to s
pub fn carry_add(asm: &mut Assembler, s: [Reg<u64>; 2], add: &Reg<u64>) -> [Reg<u64>; 2] {
    asm.append_instruction(vec![
        adds_inst(&s[0], &s[0], add),
        cinc_inst(&s[1], &s[1], "hs".to_string()),
    ]);
    s
}

pub fn carry_cmn(asm: &mut Assembler, s: [Reg<u64>; 2], add: &Reg<u64>) -> Reg<u64> {
    asm.append_instruction(vec![
        cmn_inst(&s[0], add),
        cinc_inst(&s[1], &s[1], "hs".to_string()),
    ]);
    let [_, out] = s;
    out
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

    assert_eq!(mapping.allocated(), s.len());

    use std::io::Write;
    file.write_all(txt.as_bytes())
        .expect("Unable to write data to file");
}

fn build_schoolmethod() {
    let mut alloc = Allocator::new();
    let mut mapping = RegisterMapping::new();
    let mut phys_registers = RegisterBank::new();

    let mut asm = Assembler::new();
    let a = array::from_fn(|i| input(&mut alloc, &mut mapping, &mut phys_registers, i as u64));
    let b = array::from_fn(|i| {
        input(
            &mut alloc,
            &mut mapping,
            &mut phys_registers,
            (a.len() + i) as u64,
        )
    });

    let s = school_method(&mut alloc, &mut asm, a, b);

    let first: Vec<_> = asm.instructions.into_iter().flatten().collect();

    // Is there something we can do to tie off the outputs.
    // and to make sure it happens before drop_pass
    let mut seen = Seen::new();
    s.iter().for_each(|r| {
        seen.output_interface(r);
    });

    let releases = liveness_analysis(&mut seen, &first);

    let out = hardware_register_allocation(&mut mapping, &mut phys_registers, first, releases);
    let mut file =
        std::fs::File::create("./asm/global_asm_schoolmethod.s").expect("Unable to create file");
    let txt = backend_global("schoolmethod".to_string(), out);
    let outputs: String = s
        .iter()
        .enumerate()
        .map(|(i, r)| format!("lateout(\"{}\") out[{}]", mapping.output_register(r), i))
        .intersperse(", ".to_string())
        .collect();

    println!("{}", outputs);

    assert_eq!(mapping.allocated(), s.len());

    use std::io::Write;
    file.write_all(txt.as_bytes())
        .expect("Unable to write data to file");
}

fn build_single_step() {
    let mut alloc = Allocator::new();
    let mut mapping = RegisterMapping::new();
    let mut phys_registers = RegisterBank::new();

    let mut asm = Assembler::new();
    let a = array::from_fn(|i| input(&mut alloc, &mut mapping, &mut phys_registers, i as u64));
    let b = array::from_fn(|i| {
        input(
            &mut alloc,
            &mut mapping,
            &mut phys_registers,
            (a.len() + i) as u64,
        )
    });

    let s = single_step(&mut alloc, &mut asm, a, b);

    let first: Vec<_> = asm.instructions.into_iter().flatten().collect();

    // Is there something we can do to tie off the outputs.
    // and to make sure it happens before drop_pass
    let mut seen = Seen::new();
    s.iter().for_each(|r| {
        seen.output_interface(r);
    });

    let releases = liveness_analysis(&mut seen, &first);

    let out = hardware_register_allocation(&mut mapping, &mut phys_registers, first, releases);
    let mut file =
        std::fs::File::create("./asm/global_asm_single_step.s").expect("Unable to create file");
    let txt = backend_global("single_step".to_string(), out);

    // Write this info in the assembly file
    let outputs: String = s
        .iter()
        .enumerate()
        .map(|(i, r)| format!("lateout(\"{}\") out[{}]", mapping.output_register(r), i))
        .intersperse(", ".to_string())
        .collect();

    println!("{}", outputs);

    assert_eq!(mapping.allocated(), s.len());

    use std::io::Write;
    file.write_all(txt.as_bytes())
        .expect("Unable to write data to file");
}

fn build_smul_add() {
    let mut alloc = Allocator::new();
    let mut mapping = RegisterMapping::new();
    let mut phys_registers = RegisterBank::new();

    let mut asm = Assembler::new();
    let add = array::from_fn(|i| input(&mut alloc, &mut mapping, &mut phys_registers, i as u64));
    let a = array::from_fn(|i| {
        input(
            &mut alloc,
            &mut mapping,
            &mut phys_registers,
            (add.len() + i) as u64,
        )
    });
    let b = input(
        &mut alloc,
        &mut mapping,
        &mut phys_registers,
        (add.len() + a.len()) as u64,
    );

    let s = smult_add(&mut alloc, &mut asm, add, a, b);

    let first: Vec<_> = asm.instructions.into_iter().flatten().collect();

    // Is there something we can do to tie off the outputs.
    // and to make sure it happens before drop_pass
    let mut seen = Seen::new();
    s.iter().for_each(|r| {
        seen.output_interface(r);
    });

    let releases = liveness_analysis(&mut seen, &first);

    let out = hardware_register_allocation(&mut mapping, &mut phys_registers, first, releases);
    let mut file =
        std::fs::File::create("./asm/global_asm_smul_add.s").expect("Unable to create file");
    let txt = backend_global("smul_add".to_string(), out);
    let outputs: String = s
        .iter()
        .enumerate()
        .map(|(i, r)| format!("lateout(\"{}\") out[{}]", mapping.output_register(r), i))
        .intersperse(", ".to_string())
        .collect();

    println!("{}", outputs);

    assert_eq!(mapping.allocated(), s.len());

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

global_asm!(include_str!("../../asm/mulu128.s"));

global_asm!(include_str!("../../asm/global_asm_smul.s"));
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
    // build_mulu128();
    // interleave_test();
    // simd_test();
    // build_smul();
    // build_schoolmethod();
    // build_smul_add();
    build_single_step();
}

// How do other allocating algorithms pass things along like Vec?
// In this algorithm the inputs are not used after
pub fn smult(
    alloc: &mut Allocator,
    asm: &mut Assembler,
    a: [Reg<u64>; 4],
    b: Reg<u64>,
) -> [Reg<u64>; 5] {
    // Allocates unnecessary fresh registers
    let mut t: [Reg<u64>; 5] = array::from_fn(|_| alloc.fresh());
    // Ouside of the loop because there is no carry add for the left most dword
    [t[0], t[1]] = mul_u128(alloc, asm, &a[0], &b);
    for i in 1..a.len() {
        let lohi = mul_u128(alloc, asm, &a[i], &b);
        [t[i], t[i + 1]] = carry_add(asm, lohi, &t[i]);
    }

    t
}

pub fn smult_add(
    alloc: &mut Allocator,
    asm: &mut Assembler,
    mut t: [Reg<u64>; 5],
    a: [Reg<u64>; 4],
    b: Reg<u64>,
) -> [Reg<u64>; 5] {
    // Allocates unnecessary fresh registers

    let mut carry;
    // first multiplication of a carry chain doesn't have a carry to add,
    // but it does have a value already from a previous round
    let tmp = mul_u128(alloc, asm, &a[0], &b);
    [t[0], carry] = carry_add(asm, tmp, &t[0]);
    for i in 1..a.len() {
        let tmp = mul_u128(alloc, asm, &a[i], &b);
        let tmp = carry_add(asm, tmp, &carry);
        [t[i], carry] = carry_add(asm, tmp, &t[i]);
    }
    t[a.len()] = add(alloc, asm, &t[a.len()], &carry);

    t
}

// There is an add truncate to satisfy the assembler
// using smult_add would result in an instruction that gives a
// source that isn't used
pub fn smult_add_truncate(
    alloc: &mut Allocator,
    asm: &mut Assembler,
    mut t: [Reg<u64>; 5],
    a: [Reg<u64>; 4],
    b: Reg<u64>,
) -> [Reg<u64>; 4] {
    // Allocates unnecessary fresh registers

    // first multiplication of a carry chain doesn't have a carry to add,
    // but it does have a value already from a previous round
    let tmp = mul_u128(alloc, asm, &a[0], &b);
    let mut carry = carry_cmn(asm, tmp, &t[0]);
    for i in 1..a.len() {
        let tmp = mul_u128(alloc, asm, &a[i], &b);
        let tmp = carry_add(asm, tmp, &carry);
        [t[i], carry] = carry_add(asm, tmp, &t[i]);
    }
    t[a.len()] = add(alloc, asm, &t[a.len()], &carry);

    let [_, out @ ..] = t;
    out
}

pub fn school_method(
    alloc: &mut Allocator,
    asm: &mut Assembler,
    a: [Reg<u64>; 4],
    b: [Reg<u64>; 4],
) -> [Reg<u64>; 8] {
    let mut t: [Reg<u64>; 8] = array::from_fn(|_| alloc.fresh());
    let mut carry;
    // The first carry chain is separated out as t doesn't have any values to add
    // first multiplication of a carry chain doesn't not have a carry to add
    [t[0], carry] = mul_u128(alloc, asm, &a[0], &b[0]);
    for i in 1..a.len() {
        let tmp = mul_u128(alloc, asm, &a[i], &b[0]);
        [t[i], carry] = carry_add(asm, tmp, &carry);
    }
    t[a.len()] = carry;

    // 2nd and later carry chain
    for j in 1..b.len() {
        let mut carry;
        // first multiplication of a carry chain doesn't have a carry to add,
        // but it does have a value already from a previous round
        let tmp = mul_u128(alloc, asm, &a[0], &b[j]);
        [t[j], carry] = carry_add(asm, tmp, &t[j]);
        for i in 1..a.len() {
            let tmp = mul_u128(alloc, asm, &a[i], &b[j]);
            let tmp = carry_add(asm, tmp, &carry);
            [t[i + j], carry] = carry_add(asm, tmp, &t[i + j]);
        }
        t[j + a.len()] = carry;
    }

    t
}

pub fn load_const(alloc: &mut Allocator, asm: &mut Assembler, val: u64) -> Reg<u64> {
    let reg = alloc.fresh();

    for i in 0..4 {
        asm.append_instruction(vec![movk_inst(&reg, (val >> (i * 16)) as u16, i * 16)])
    }
    reg
}

pub fn subv(
    alloc: &mut Allocator,
    asm: &mut Assembler,
    a: &[Reg<u64>; 4],
    b: &[Reg<u64>; 4],
) -> [Reg<u64>; 4] {
    let out = array::from_fn(|_| alloc.fresh());
    // Due to carry chain this needs to be an atomic block.
    asm.append_instruction(vec![
        subs_inst(&out[0], &a[0], &b[0]),
        sbcs_inst(&out[1], &a[1], &b[1]),
        sbcs_inst(&out[2], &a[2], &b[2]),
        sbcs_inst(&out[3], &a[3], &b[3]),
    ]);
    out
}

// Reduce within 256-2p
pub fn reduce(alloc: &mut Allocator, asm: &mut Assembler, a: [Reg<u64>; 4]) -> [Reg<u64>; 4] {
    let p2 = U64_2P.map(|val| load_const(alloc, asm, val));
    let red = subv(alloc, asm, &a, &p2);
    let out = array::from_fn(|_| alloc.fresh());
    asm.append_instruction(vec![
        tst_inst(&a[3], 1 << 63),
        csel_inst(&out[0], &red[0], &a[0], "mi".to_string()),
        csel_inst(&out[1], &red[1], &a[1], "mi".to_string()),
        csel_inst(&out[2], &red[2], &a[2], "mi".to_string()),
        csel_inst(&out[3], &red[3], &a[3], "mi".to_string()),
    ]);
    out
}

pub fn single_step(
    alloc: &mut Allocator,
    asm: &mut Assembler,
    a: [Reg<u64>; 4],
    b: [Reg<u64>; 4],
) -> [Reg<u64>; 4] {
    let t = school_method(alloc, asm, a, b);
    // let [t0, t1, t2, s @ ..] = t;
    let [t0, t1, t2, s @ ..] = t;

    let i3 = U64_I3.map(|val| load_const(alloc, asm, val));
    let r1 = smult_add(alloc, asm, s, i3, t0);

    let i2 = U64_I2.map(|val| load_const(alloc, asm, val));
    let r2 = smult_add(alloc, asm, r1, i2, t1);

    let i1 = U64_I1.map(|val| load_const(alloc, asm, val));
    let r3 = smult_add(alloc, asm, r2, i1, t2);

    let mu0 = load_const(alloc, asm, U64_MU0);
    let m = mul(alloc, asm, &mu0, &r3[0]);

    let p = U64_P.map(|val| load_const(alloc, asm, val));
    let r4 = smult_add_truncate(alloc, asm, r3, p, m);

    reduce(alloc, asm, r4)
}

pub fn mul_u128(
    alloc: &mut Allocator,
    asm: &mut Assembler,
    a: &Reg<u64>,
    b: &Reg<u64>,
) -> [Reg<u64>; 2] {
    [mul(alloc, asm, a, b), umulh(alloc, asm, a, b)]
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
