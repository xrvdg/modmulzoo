#![feature(iter_intersperse)]
use std::array;

use hla::*;
use montgomery_reduction::yuval::{U64_2P, U64_I1, U64_I2, U64_I3, U64_MU0, U64_P};

/* BUILDERS */
fn build_schoolmethod() {
    fn setup(
        mut alloc: Allocator,
        mapping: &mut RegisterMapping,
        phys_registers: &mut RegisterBank,
        asm: &mut Assembler,
    ) -> (Vec<TypedSizedRegister<HardwareRegister>>, Vec<Reg<u64>>) {
        let a = array::from_fn(|i| input(&mut alloc, mapping, phys_registers, i as u64));
        let b =
            array::from_fn(|i| input(&mut alloc, mapping, phys_registers, (a.len() + i) as u64));

        let input_hw_registers: Vec<_> = a
            .iter()
            .chain(&b)
            .filter_map(|reg| mapping.output_register(reg))
            .collect();

        let s = school_method(&mut alloc, asm, &a, &b);

        (input_hw_registers, Vec::from(s))
    }

    build_func("school_method", setup);
}

fn build_func<T: RegisterSource>(
    label: &str,
    f: fn(
        alloc: Allocator,
        mapping: &mut RegisterMapping,
        phys_registers: &mut RegisterBank,
        asm: &mut Assembler,
    ) -> (Vec<TypedSizedRegister<HardwareRegister>>, Vec<Reg<T>>),
) {
    let alloc = Allocator::new();
    let mut mapping = RegisterMapping::new();
    let mut phys_registers = RegisterBank::new();

    let mut asm = Assembler::new();
    let (input_hw_registers, s) = f(alloc, &mut mapping, &mut phys_registers, &mut asm);

    let first: Vec<_> = asm.instructions.into_iter().flatten().collect();

    // Is there something we can do to tie off the outputs.
    // and to make sure it happens before drop_pass
    let mut seen = Seen::new();
    s.iter().for_each(|r| {
        seen.output_interface(r);
    });

    let releases = liveness_analysis(&mut seen, &first);

    let out = hardware_register_allocation(&mut mapping, &mut phys_registers, first, releases);

    let output_hw_registers: Vec<_> = s
        .iter()
        .filter_map(|reg| mapping.output_register(reg))
        .collect();

    let txt = backend_global(label, &out);

    // Write this info in the assembly file
    let operands = backend_rust(mapping, &input_hw_registers, &output_hw_registers, &out);
    let operands_with_semicolon: Vec<String> =
        operands.lines().map(|line| format!("//{}", line)).collect();
    let operands = format!("{}\n", operands_with_semicolon.join("\n"));

    use std::io::Write;
    let mut file = std::fs::File::create(format!("./asm/global_asm_{label}.s"))
        .expect("Unable to create file");
    file.write_all(operands.as_bytes())
        .expect("Unable to write data to file");
    file.write_all(txt.as_bytes())
        .expect("Unable to write data to file");
}

fn build_single_step() {
    fn input_setup(
        mut alloc: Allocator,
        mapping: &mut RegisterMapping,
        phys_registers: &mut RegisterBank,
        asm: &mut Assembler,
    ) -> (Vec<TypedSizedRegister<HardwareRegister>>, Vec<Reg<u64>>) {
        let a = array::from_fn(|i| input(&mut alloc, mapping, phys_registers, i as u64));
        let b =
            array::from_fn(|i| input(&mut alloc, mapping, phys_registers, (a.len() + i) as u64));

        let input_hw_registers: Vec<_> = a
            .iter()
            .chain(&b)
            .filter_map(|reg| mapping.output_register(&reg))
            .collect();

        let s = single_step(&mut alloc, asm, &a, &b);
        (input_hw_registers, Vec::from(s))
    }

    build_func("single_step", input_setup)
}

fn build_smul_add() {
    fn input_setup(
        mut alloc: Allocator,
        mapping: &mut RegisterMapping,
        phys_registers: &mut RegisterBank,
        asm: &mut Assembler,
    ) -> (Vec<TypedSizedRegister<HardwareRegister>>, Vec<Reg<u64>>) {
        let add = array::from_fn(|i| input(&mut alloc, mapping, phys_registers, i as u64));
        let a =
            array::from_fn(|i| input(&mut alloc, mapping, phys_registers, (add.len() + i) as u64));
        let b = input(
            &mut alloc,
            mapping,
            phys_registers,
            (add.len() + a.len()) as u64,
        );

        let input_hw_registers: Vec<_> = add
            .iter()
            .chain(&a)
            .chain(std::iter::once(&b))
            .filter_map(|reg| mapping.output_register(&reg))
            .collect();

        let s = smult_add(&mut alloc, asm, add, a, b);

        (input_hw_registers, Vec::from(s))
    }
    build_func("smul_add", input_setup);
}

fn main() {
    build_smul_add();
    build_schoolmethod();
    build_single_step();
}

/* GENERATORS */

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
    a: &[Reg<u64>; 4],
    b: &[Reg<u64>; 4],
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
    a: &[Reg<u64>; 4],
    b: &[Reg<u64>; 4],
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

//*******  SIMD **********/
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
