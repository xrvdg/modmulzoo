#![feature(iter_intersperse)]
use std::array;

use block_multiplier::{constants::*, make_initial};
use hla::*;
// TODO don't rely on montgomery_reduction for anything other than tests
// Possible not even then
use montgomery_reduction::{
    domb::heaviside,
    yuval::{U64_2P, U64_I1, U64_I2, U64_I3, U64_MU0, U64_P},
};

/* BUILDERS */

fn setup_schoolmethod(
    alloc: &mut Allocator,
    mapping: &mut RegisterMapping,
    phys_registers: &mut RegisterBank,
    asm: &mut Assembler,
) -> (
    Vec<Vec<TypedSizedRegister<HardwareRegister>>>,
    Vec<Reg<u64>>,
) {
    let a = array::from_fn(|i| input(alloc, mapping, phys_registers, i as u64));
    let b = array::from_fn(|i| input(alloc, mapping, phys_registers, (a.len() + i) as u64));

    let input_hw_registers_a = a
        .iter()
        .filter_map(|reg| mapping.output_register(reg))
        .collect();

    let input_hw_registers_b = b
        .iter()
        .filter_map(|reg| mapping.output_register(reg))
        .collect();

    let s = school_method(alloc, asm, &a, &b);

    (
        vec![input_hw_registers_a, input_hw_registers_b],
        Vec::from(s),
    )
}

fn build_func<T: RegisterSource>(
    label: &str,
    f: fn(
        alloc: &mut Allocator,
        mapping: &mut RegisterMapping,
        phys_registers: &mut RegisterBank,
        asm: &mut Assembler,
    ) -> (Vec<Vec<TypedSizedRegister<HardwareRegister>>>, Vec<Reg<T>>),
) {
    let mut alloc = Allocator::new();
    let mut mapping = RegisterMapping::new();
    let mut phys_registers = RegisterBank::new();

    let mut asm = Assembler::new();
    let (input_hw_registers, s) = f(&mut alloc, &mut mapping, &mut phys_registers, &mut asm);

    let first: Vec<_> = asm.instructions.into_iter().flatten().collect();

    // Is there something we n do to tie off the outputs.
    // and to make sure it happens before drop_pass
    let mut seen = Seen::new();
    s.iter().for_each(|r| {
        seen.output_interface(r);
    });

    let (releases, lifetimes) = liveness_analysis(&mut seen, &first, alloc.fresh as usize);

    s.iter().enumerate().for_each(|(idx, r)| {
        pin_register(&mut mapping, &mut phys_registers, &lifetimes, r, idx as u64);
    });

    let out = hardware_register_allocation(
        &mut mapping,
        &mut phys_registers,
        first,
        releases,
        lifetimes,
    );

    let output_hw_registers: Vec<_> = s
        .iter()
        .filter_map(|reg| mapping.output_register(reg))
        .collect();

    let txt = backend_global(label, &out);

    // Write this info in the assembly file
    let operands = backend_rust(mapping, input_hw_registers, vec![output_hw_registers], &out);
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

fn setup_single_step(
    alloc: &mut Allocator,
    mapping: &mut RegisterMapping,
    phys_registers: &mut RegisterBank,
    asm: &mut Assembler,
) -> (
    Vec<Vec<TypedSizedRegister<HardwareRegister>>>,
    Vec<Reg<u64>>,
) {
    let a = array::from_fn(|i| input(alloc, mapping, phys_registers, i as u64));
    let b = array::from_fn(|i| input(alloc, mapping, phys_registers, (a.len() + i) as u64));

    let input_hw_registers_a: Vec<_> = a
        .iter()
        .filter_map(|reg| mapping.output_register(&reg))
        .collect();

    let input_hw_registers_b: Vec<_> = b
        .iter()
        .filter_map(|reg| mapping.output_register(&reg))
        .collect();

    let s = single_step(alloc, asm, &a, &b);
    (
        vec![input_hw_registers_a, input_hw_registers_b],
        Vec::from(s),
    )
}

fn setup_single_step_split(
    alloc: &mut Allocator,
    mapping: &mut RegisterMapping,
    phys_registers: &mut RegisterBank,
    asm: &mut Assembler,
) -> (
    Vec<Vec<TypedSizedRegister<HardwareRegister>>>,
    Vec<Reg<u64>>,
) {
    let a = array::from_fn(|i| input(alloc, mapping, phys_registers, i as u64));
    let b = array::from_fn(|i| input(alloc, mapping, phys_registers, (a.len() + i) as u64));

    let input_hw_registers_a: Vec<_> = a
        .iter()
        .filter_map(|reg| mapping.output_register(&reg))
        .collect();

    let input_hw_registers_b: Vec<_> = b
        .iter()
        .filter_map(|reg| mapping.output_register(&reg))
        .collect();

    let s = single_step_split(alloc, asm, &a, &b);
    (
        vec![input_hw_registers_a, input_hw_registers_b],
        Vec::from(s),
    )
}

fn build_single_step_split() {
    build_func("single_step_split", setup_single_step_split)
}

fn setup_smul_add(
    alloc: &mut Allocator,
    mapping: &mut RegisterMapping,
    phys_registers: &mut RegisterBank,
    asm: &mut Assembler,
) -> (
    Vec<Vec<TypedSizedRegister<HardwareRegister>>>,
    Vec<Reg<u64>>,
) {
    let add = array::from_fn(|i| input(alloc, mapping, phys_registers, i as u64));
    let a = array::from_fn(|i| input(alloc, mapping, phys_registers, (add.len() + i) as u64));
    let b = input(alloc, mapping, phys_registers, (add.len() + a.len()) as u64);

    let input_hw_registers_add: Vec<_> = add
        .iter()
        .filter_map(|reg| mapping.output_register(&reg))
        .collect();

    let input_hw_registers_a: Vec<_> = a
        .iter()
        .filter_map(|reg| mapping.output_register(&reg))
        .collect();

    let input_hw_registers_b: Vec<_> = vec![mapping.output_register(&b).unwrap()];

    let s = smult_add(alloc, asm, add, a, b);

    (
        vec![
            input_hw_registers_add,
            input_hw_registers_a,
            input_hw_registers_b,
        ],
        Vec::from(s),
    )
}

fn setup_u256_to_u260_shl2_imd(
    alloc: &mut Allocator,
    mapping: &mut RegisterMapping,
    phys_registers: &mut RegisterBank,
    asm: &mut Assembler,
) -> (
    Vec<Vec<TypedSizedRegister<HardwareRegister>>>,
    Vec<Reg<Simd<u64, 2>>>,
) {
    let limbs = array::from_fn(|i| input(alloc, mapping, phys_registers, i as u64));

    let input_hw_registers: Vec<_> = limbs
        .iter()
        .filter_map(|reg| mapping.output_register(reg))
        .collect();

    let mask = mov(alloc, asm, MASK52);
    let mask_simd = dup2d(alloc, asm, &mask);

    let res = u256_to_u260_shl2_simd(alloc, asm, &mask_simd, limbs);

    (vec![input_hw_registers], Vec::from(res))
}

fn setup_u260_to_u256_simd(
    alloc: &mut Allocator,
    mapping: &mut RegisterMapping,
    phys_registers: &mut RegisterBank,
    asm: &mut Assembler,
) -> (
    Vec<Vec<TypedSizedRegister<HardwareRegister>>>,
    Vec<Reg<Simd<u64, 2>>>,
) {
    let limbs = array::from_fn(|i| input(alloc, mapping, phys_registers, i as u64));

    let input_hw_registers: Vec<_> = limbs
        .iter()
        .filter_map(|reg| mapping.output_register(reg))
        .collect();

    let res = u260_to_u256_simd(alloc, asm, limbs);

    (vec![input_hw_registers], Vec::from(res))
}

fn setup_vmultadd_noinit_simd(
    alloc: &mut Allocator,
    mapping: &mut RegisterMapping,
    phys_registers: &mut RegisterBank,
    asm: &mut Assembler,
) -> (
    Vec<Vec<TypedSizedRegister<HardwareRegister>>>,
    Vec<Reg<Simd<u64, 2>>>,
) {
    let t = array::from_fn(|i| input(alloc, mapping, phys_registers, i as u64));
    let a = array::from_fn(|i| input(alloc, mapping, phys_registers, (i + t.len()) as u64));
    let b = array::from_fn(|i| {
        input(
            alloc,
            mapping,
            phys_registers,
            (i + t.len() + a.len()) as u64,
        )
    }); // Assuming b starts after a

    let input_hw_registers_t: Vec<_> = t
        .iter()
        .filter_map(|reg| mapping.output_register(reg))
        .collect();
    let input_hw_registers_a: Vec<_> = a
        .iter()
        .filter_map(|reg| mapping.output_register(reg))
        .collect();
    let input_hw_registers_b: Vec<_> = b
        .iter()
        .filter_map(|reg| mapping.output_register(reg))
        .collect();

    let c1 = mov(alloc, asm, C1.to_bits());
    let c1 = dup2d(alloc, asm, &c1);

    // Alternative is c2 = c1 + 1; This requires a change to add to support immediate
    let c2 = load_const(alloc, asm, C2.to_bits());
    let c2 = dup2d(alloc, asm, &c2);

    let res = vmultadd_noinit_simd(alloc, asm, &c1, &c2, t, a, b);

    (
        vec![
            input_hw_registers_t,
            input_hw_registers_a,
            input_hw_registers_b,
        ],
        Vec::from(res),
    )
}

fn setup_single_step_simd(
    alloc: &mut Allocator,
    mapping: &mut RegisterMapping,
    phys_registers: &mut RegisterBank,
    asm: &mut Assembler,
) -> (
    Vec<Vec<TypedSizedRegister<HardwareRegister>>>,
    Vec<Reg<Simd<u64, 2>>>,
) {
    let a = array::from_fn(|i| input(alloc, mapping, phys_registers, i as u64));
    let b = array::from_fn(|i| input(alloc, mapping, phys_registers, (i + a.len()) as u64)); // Assuming b starts after a

    let input_hw_registers_a: Vec<_> = a
        .iter()
        .filter_map(|reg| mapping.output_register(reg))
        .collect();

    let input_hw_registers_b: Vec<_> = b
        .iter()
        .filter_map(|reg| mapping.output_register(reg))
        .collect();

    let res = single_step_simd(alloc, asm, a, b);

    (
        vec![input_hw_registers_a, input_hw_registers_b],
        Vec::from(res),
    )
}

fn setup_reduce_ct_simd(
    alloc: &mut Allocator,
    mapping: &mut RegisterMapping,
    phys_registers: &mut RegisterBank,
    asm: &mut Assembler,
) -> (
    Vec<Vec<TypedSizedRegister<HardwareRegister>>>,
    Vec<Reg<Simd<u64, 2>>>,
) {
    let red = array::from_fn(|i| input(alloc, mapping, phys_registers, i as u64));

    let input_hw_registers: Vec<_> = red
        .iter()
        .filter_map(|reg| mapping.output_register(reg))
        .collect();

    let mask = mov(alloc, asm, MASK52);
    let mask52 = dup2d(alloc, asm, &mask);

    let res = reduce_ct_simd(alloc, asm, red).map(|reg| and16(alloc, asm, &reg, &mask52));

    (vec![input_hw_registers], Vec::from(res))
}

fn build_interleaved(label: &str) {
    let mut alloc = Allocator::new();
    let mut mapping = RegisterMapping::new();
    let mut phys_registers = RegisterBank::new();

    let mut fst_asm = Assembler::new();
    let (fst_input_hw_registers, fst_regs) =
        setup_single_step(&mut alloc, &mut mapping, &mut phys_registers, &mut fst_asm);

    let mut snd_asm = Assembler::new();

    let (snd_input_hw_registers, snd_regs) =
        setup_single_step_simd(&mut alloc, &mut mapping, &mut phys_registers, &mut snd_asm);

    let mixed: Vec<_> = interleave(fst_asm.instructions, snd_asm.instructions)
        .into_iter()
        .flatten()
        .collect();

    // Is there something we n do to tie off the outputs.
    // and to make sure it happens before drop_pass
    let mut seen = Seen::new();
    fst_regs.iter().for_each(|r| {
        seen.output_interface(r);
    });
    snd_regs.iter().for_each(|r| {
        seen.output_interface(r);
    });

    let (releases, lifetimes) = liveness_analysis(&mut seen, &mixed, alloc.fresh as usize);

    let out = hardware_register_allocation(
        &mut mapping,
        &mut phys_registers,
        mixed,
        releases,
        lifetimes,
    );

    let fst_output_hw_registers: Vec<_> = fst_regs
        .iter()
        .filter_map(|reg| mapping.output_register(reg))
        .collect();

    let snd_output_hw_registers: Vec<_> = snd_regs
        .iter()
        .filter_map(|reg| mapping.output_register(reg))
        .collect();

    let txt = backend_global(label, &out);

    let mut input_hw_registers = fst_input_hw_registers;
    input_hw_registers.extend(snd_input_hw_registers);

    // Write this info in the assembly file
    let operands = backend_rust(
        mapping,
        input_hw_registers,
        vec![fst_output_hw_registers, snd_output_hw_registers],
        &out,
    );
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

fn main() {
    build_func("smul_add", setup_smul_add);
    build_func("school_method", setup_schoolmethod);
    build_func("single_step", setup_single_step);
    build_func("single_step_split", setup_single_step_split);
    build_func("u256_to_u260_shl2_simd", setup_u256_to_u260_shl2_imd);
    build_func("u260_to_u256_simd", setup_u260_to_u256_simd);
    build_func("vmultadd_noinit_simd", setup_vmultadd_noinit_simd);
    build_func("single_step_simd", setup_single_step_simd);
    build_func("reduce_ct_simd", setup_reduce_ct_simd);
    build_interleaved("single_step_interleaved");
}

/* GENERATORS */

// adds can be confusng as it has a similar shape to s
pub fn carry_add(
    alloc: &mut Allocator,
    asm: &mut Assembler,
    s: &[Reg<u64>; 2],
    add: &Reg<u64>,
) -> [Reg<u64>; 2] {
    let ret = array::from_fn(|_| alloc.fresh());
    asm.append_instruction(vec![
        adds_inst(&ret[0], &s[0], add),
        cinc_inst(&ret[1], &s[1], "hs".to_string()),
    ]);
    ret
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
        [t[i], t[i + 1]] = carry_add(alloc, asm, &lohi, &t[i]);
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
    let mut carry;
    // first multiplication of a carry chain doesn't have a carry to add,
    // but it does have a value already from a previous round
    let tmp = mul_u128(alloc, asm, &a[0], &b);
    [t[0], carry] = carry_add(alloc, asm, &tmp, &t[0]);
    for i in 1..a.len() {
        let tmp = mul_u128(alloc, asm, &a[i], &b);
        let tmp = carry_add(alloc, asm, &tmp, &carry);
        [t[i], carry] = carry_add(alloc, asm, &tmp, &t[i]);
    }
    t[a.len()] = add(alloc, asm, &t[a.len()], &carry);

    t
}

pub fn addv(
    alloc: &mut Allocator,
    asm: &mut Assembler,
    a: [Reg<u64>; 5],
    b: [Reg<u64>; 5],
) -> [Reg<u64>; 5] {
    let t: [Reg<u64>; 5] = array::from_fn(|_| alloc.fresh());
    let n: usize = t.len();

    let mut instructions = Vec::new();
    instructions.push(adds_inst(&t[0], &a[0], &b[0]));
    for i in 1..n - 1 {
        instructions.push(adcs_inst(&t[i], &a[i], &b[i]));
    }
    instructions.push(adc_inst(&t[n - 1], &a[n - 1], &b[n - 1]));
    asm.append_instruction(instructions);

    t
}

pub fn addv_truncate(
    alloc: &mut Allocator,
    asm: &mut Assembler,
    a: [Reg<u64>; 5],
    b: [Reg<u64>; 5],
) -> [Reg<u64>; 4] {
    let t: [Reg<u64>; 4] = array::from_fn(|_| alloc.fresh());

    let mut instructions = Vec::new();
    instructions.push(cmn_inst(&a[0], &b[0]));
    for i in 1..a.len() {
        instructions.push(adcs_inst(&t[i - 1], &a[i], &b[i]));
    }
    instructions.push(adc_inst(&t[3], &a[4], &b[4]));
    asm.append_instruction(instructions);

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
        let tmp = carry_add(alloc, asm, &tmp, &carry);
        [t[i], carry] = carry_add(alloc, asm, &tmp, &t[i]);
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
        [t[i], carry] = carry_add(alloc, asm, &tmp, &carry);
    }
    t[a.len()] = carry;

    // 2nd and later carry chain
    for j in 1..b.len() {
        let mut carry;
        // first multiplication of a carry chain doesn't have a carry to add,
        // but it does have a value already from a previous round
        let tmp = mul_u128(alloc, asm, &a[0], &b[j]);
        [t[j], carry] = carry_add(alloc, asm, &tmp, &t[j]);
        for i in 1..a.len() {
            let tmp = mul_u128(alloc, asm, &a[i], &b[j]);
            let tmp = carry_add(alloc, asm, &tmp, &carry);
            [t[i + j], carry] = carry_add(alloc, asm, &tmp, &t[i + j]);
        }
        t[j + a.len()] = carry;
    }

    t
}

// TODO make load_const smart that it knowns when to use mov and when to use a sequence of movk?
// That would require checking if only one of the 16 bit libs is zero.
pub fn load_const(alloc: &mut Allocator, asm: &mut Assembler, val: u64) -> Reg<u64> {
    // The first load we do with mov instead of movk because of the optimization that leaves moves out.
    let l0 = val as u16;
    let reg = mov(alloc, asm, l0 as u64);

    for i in 1..4 {
        let vali = (val >> (i * 16)) as u16;
        // If the value for limb i is zero then we do not have to emit an instruction.
        if vali != 0 {
            asm.append_instruction(vec![movk_inst(&reg, vali, i * 16)])
        }
    }
    reg
}

pub fn load_floating_simd(
    alloc: &mut Allocator,
    asm: &mut Assembler,
    val: f64,
) -> Reg<Simd<f64, 2>> {
    let c = load_const(alloc, asm, val.to_bits());
    dup2d(alloc, asm, &c).into_()
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

pub fn single_step_split(
    alloc: &mut Allocator,
    asm: &mut Assembler,
    a: &[Reg<u64>; 4],
    b: &[Reg<u64>; 4],
) -> [Reg<u64>; 4] {
    let t = school_method(alloc, asm, a, b);
    // let [t0, t1, t2, s @ ..] = t;
    let [t0, t1, t2, s @ ..] = t;

    let i3 = U64_I3.map(|val| load_const(alloc, asm, val));
    let r1 = smult(alloc, asm, i3, t0);

    let i2 = U64_I2.map(|val| load_const(alloc, asm, val));
    let r2 = smult(alloc, asm, i2, t1);

    let i1 = U64_I1.map(|val| load_const(alloc, asm, val));
    let r3 = smult(alloc, asm, i1, t2);

    let r4 = addv(alloc, asm, r1, r2);
    let r5 = addv(alloc, asm, r4, r3);
    let r6 = addv(alloc, asm, r5, s);

    let mu0 = load_const(alloc, asm, U64_MU0);
    let m = mul(alloc, asm, &mu0, &r6[0]);

    let p = U64_P.map(|val| load_const(alloc, asm, val));
    let r7 = smult(alloc, asm, p, m);
    let r8 = addv_truncate(alloc, asm, r7, r6);

    reduce(alloc, asm, r8)
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
fn load_tuple(
    alloc: &mut Allocator,
    asm: &mut Assembler,
    fst: Reg<u64>,
    snd: Reg<u64>,
) -> Reg<Simd<u64, 2>> {
    let fresh: Reg<Simd<u64, 2>> = alloc.fresh();
    asm.append_instruction(vec![
        ins_inst(fresh._d0(), &fst),
        ins_inst(fresh._d1(), &snd),
    ]);
    fresh
}

fn transpose_u256_to_simd(
    alloc: &mut Allocator,
    asm: &mut Assembler,
    limbs: [[Reg<u64>; 4]; 2],
) -> [Reg<Simd<u64, 2>>; 4] {
    let [[l00, l01, l02, l03], [l10, l11, l12, l13]] = limbs;
    [
        load_tuple(alloc, asm, l00, l10),
        load_tuple(alloc, asm, l01, l11),
        load_tuple(alloc, asm, l02, l12),
        load_tuple(alloc, asm, l03, l13),
    ]
}

fn u256_to_u260_shl2_simd(
    alloc: &mut Allocator,
    asm: &mut Assembler,
    mask52: &Reg<Simd<u64, 2>>,
    limbs: [Reg<Simd<u64, 2>>; 4],
) -> [Reg<Simd<u64, 2>>; 5] {
    let [l0, l1, l2, l3] = limbs;

    let shifted_l1 = shl2d(alloc, asm, &l1, 14);
    let shifted_l2 = shl2d(alloc, asm, &l2, 26);
    let shifted_l3 = shl2d(alloc, asm, &l3, 38);
    // The input and output interface share the same registers. By moving this operation to somewhere in the beginning
    // we can free up the hardware register tied to l3.
    let last = ushr2d(alloc, asm, &l3, 14);

    let shifted_ol0 = shl2d(alloc, asm, &l0, 2);
    let shifted_ol1 = usra2d(alloc, asm, shifted_l1, &l0, 50);
    let shifted_ol2 = usra2d(alloc, asm, shifted_l2, &l1, 38);
    let shifted_ol3 = usra2d(alloc, asm, shifted_l3, &l2, 26);

    [
        and16(alloc, asm, &shifted_ol0, &mask52),
        and16(alloc, asm, &shifted_ol1, &mask52),
        and16(alloc, asm, &shifted_ol2, &mask52),
        and16(alloc, asm, &shifted_ol3, &mask52),
        last,
    ]
}

fn load_const_simd(alloc: &mut Allocator, asm: &mut Assembler, val: u64) -> Reg<Simd<u64, 2>> {
    let val = load_const(alloc, asm, val);
    let mask = dup2d(alloc, asm, &val);
    mask
}

// Embed the initials as instructions
// TODO with larger block size this loading can be kept outside and copied
// This is very specific to parallel_sub_simd_r256 is might be better inlined
fn make_initials(alloc: &mut Allocator, asm: &mut Assembler) -> [Reg<Simd<u64, 2>>; 10] {
    let mut t: [Reg<Simd<u64, 2>>; 10] = array::from_fn(|_| alloc.fresh());

    for i in 0..5 {
        let lower_val = make_initial(i + 1 + 5 * heaviside(i as isize - 4), i);
        let lower_val = mov(alloc, asm, lower_val);

        t[i] = dup2d(alloc, asm, &lower_val);

        let j = 10 - 1 - i;

        let upper_val = make_initial(i + 5 * (1 - heaviside(j as isize - 9)), i + 1 + 5 * 1);
        let upper_val = mov(alloc, asm, upper_val);
        t[j] = dup2d(alloc, asm, &upper_val);
    }

    t
}

fn vmultadd_noinit_simd(
    alloc: &mut Allocator,
    asm: &mut Assembler,
    c1: &Reg<Simd<u64, 2>>,
    c2: &Reg<Simd<u64, 2>>,
    mut t: [Reg<Simd<u64, 2>>; 10],
    a: [Reg<Simd<u64, 2>>; 5],
    b: [Reg<Simd<u64, 2>>; 5],
) -> [Reg<Simd<u64, 2>>; 10] {
    let a = a.map(|ai| ucvtf2d(alloc, asm, &ai));
    let b = b.map(|bi| ucvtf2d(alloc, asm, &bi));
    for i in 0..a.len() {
        for j in 0..b.len() {
            let lc1 = mov16b(alloc, asm, c1);

            let hi = fmla2d(alloc, asm, lc1.into_(), &a[i], &b[j]);
            let tmp = fsub2d(alloc, asm, &c2.as_(), &hi);
            let lo = fmla2d(alloc, asm, tmp, &a[i], &b[j]);

            t[i + j + 1] = add2d(alloc, asm, &t[i + j + 1], &hi.into_());
            t[i + j] = add2d(alloc, asm, &t[i + j], &lo.into_());
        }
    }
    t
}

// Whole vector is in registers, but that might not be great. Better to have it on the stack and load it from there
pub fn smultadd_noinit_simd(
    alloc: &mut Allocator,
    asm: &mut Assembler,
    mut t: [Reg<Simd<u64, 2>>; 6],
    c1: &Reg<Simd<u64, 2>>,
    c2: &Reg<Simd<u64, 2>>,
    s: Reg<Simd<u64, 2>>,
    v: [u64; 5],
) -> [Reg<Simd<u64, 2>>; 6] {
    let s = ucvtf2d(alloc, asm, &s);

    // This ordering is the fastest that I've found. Any change or breaking up into parts seem
    // to inhibit bypass
    for i in 0..v.len() {
        // skip ucvtf by loading the constant directly as (simd) floating point
        // No measurable difference in loading the vector v completely outside or per element inside the load
        let vs = load_floating_simd(alloc, asm, v[i] as f64);
        let lc1 = mov16b(alloc, asm, &c1);

        let hi = fmla2d(alloc, asm, lc1.into_(), &s, &vs);
        let tmp = fsub2d(alloc, asm, c2.as_(), &hi);
        let lo = fmla2d(alloc, asm, tmp, &s, &vs);

        t[i + 1] = add2d(alloc, asm, &t[i + 1], hi.as_());
        t[i] = add2d(alloc, asm, &t[i], lo.as_());
    }
    t
}

/// Constants that are used across functions
/// Misses the transposing to make it easier on the registers for the next steps
fn single_step_simd(
    alloc: &mut Allocator,
    asm: &mut Assembler,
    a: [Reg<Simd<u64, 2>>; 4],
    b: [Reg<Simd<u64, 2>>; 4],
) -> [Reg<Simd<u64, 2>>; 4] {
    let mask = mov(alloc, asm, MASK52);
    let mask52 = dup2d(alloc, asm, &mask);

    let a = u256_to_u260_shl2_simd(alloc, asm, &mask52, a);
    let b = u256_to_u260_shl2_simd(alloc, asm, &mask52, b);
    let t = make_initials(alloc, asm);

    let c1 = mov(alloc, asm, C1.to_bits());
    let c1 = dup2d(alloc, asm, &c1);

    // Alternative is c2 = c1 + 1; This requires a change to add to support immediate
    let c2 = load_const(alloc, asm, C2.to_bits());
    let c2 = dup2d(alloc, asm, &c2);

    let [t0, t1, t2, t3, t4, t5, t6, t7, t8, t9] =
        vmultadd_noinit_simd(alloc, asm, &c1, &c2, t, a, b);

    let t1 = usra2d(alloc, asm, t1, &t0, 52);
    let t2 = usra2d(alloc, asm, t2, &t1, 52);
    let t3 = usra2d(alloc, asm, t3, &t2, 52);
    let t4 = usra2d(alloc, asm, t4, &t3, 52);

    let t4_10 = [t4, t5, t6, t7, t8, t9];

    let t0 = and16(alloc, asm, &t0, &mask52);
    let t1 = and16(alloc, asm, &t1, &mask52);
    let t2 = and16(alloc, asm, &t2, &mask52);
    let t3 = and16(alloc, asm, &t3, &mask52);

    // loading rho interleaved with multiplication to prevent to prevent allocation a lot of X-registers
    let r0 = smultadd_noinit_simd(alloc, asm, t4_10, &c1, &c2, t0, RHO_4);
    let r1 = smultadd_noinit_simd(alloc, asm, r0, &c1, &c2, t1, RHO_3);
    let r2 = smultadd_noinit_simd(alloc, asm, r1, &c1, &c2, t2, RHO_2);
    let s = smultadd_noinit_simd(alloc, asm, r2, &c1, &c2, t3, RHO_1);

    // Could be replaced with fmul, but the rust compiler generates something close to this
    let u52_np0 = load_const(alloc, asm, U52_NP0);
    let s00 = umov(alloc, asm, &s[0]._d0());
    let s01 = umov(alloc, asm, &s[0]._d1());
    let m0 = mul(alloc, asm, &s00, &u52_np0);
    let m1 = mul(alloc, asm, &s01, &u52_np0);

    let m0 = and(alloc, asm, &m0, &mask);
    let m1 = and(alloc, asm, &m1, &mask);
    let m = load_tuple(alloc, asm, m0, m1);

    let s = smultadd_noinit_simd(alloc, asm, s, &c1, &c2, m, U52_P);

    let rs = reduce_ct_simd(alloc, asm, s);

    u260_to_u256_simd(alloc, asm, rs)
}

fn u260_to_u256_simd(
    alloc: &mut Allocator,
    asm: &mut Assembler,
    limbs: [Reg<Simd<u64, 2>>; 5],
) -> [Reg<Simd<u64, 2>>; 4] {
    let [l0, l1, l2, l3, l4] = limbs;

    let shifted_l1 = ushr2d(alloc, asm, &l1, 12);
    let shifted_l2 = ushr2d(alloc, asm, &l2, 24);
    let shifted_l3 = ushr2d(alloc, asm, &l3, 36);

    [
        sli2d(alloc, asm, l0, &l1, 52),
        sli2d(alloc, asm, shifted_l1, &l2, 40),
        sli2d(alloc, asm, shifted_l2, &l3, 28),
        sli2d(alloc, asm, shifted_l3, &l4, 16),
    ]
}

/// Does reduction with -2p, but DOESN'T return a clean 52 bit limbs.
/// It doesn't clean up the carries in the upper 52bit. u260-to-u256 takes care of that.
/// This allows us to drop 5 vector instructions.
fn reduce_ct_simd(
    alloc: &mut Allocator,
    asm: &mut Assembler,
    red: [Reg<Simd<u64, 2>>; 6],
) -> [Reg<Simd<u64, 2>>; 5] {
    // Set cmp to zero if the msb (4x52 + 47) is set.
    let msb_mask = mov(alloc, asm, 1 << 47);
    let msb_mask = dup2d(alloc, asm, &msb_mask);
    let msb = and16(alloc, asm, &red[5], &msb_mask);
    // The comparison state is stored in a vector register instead of NCVF
    // Therefore these operations can be interleaved without making it atomic
    let cmp = cmeq2d(alloc, asm, &msb, 0);

    let subtrahend: [Reg<Simd<_, 2>>; 5] = U52_2P.map(|i| {
        let p = load_const_simd(alloc, asm, i);
        // p & (~cmp) -> if msb is set return p else return 0
        bic16(alloc, asm, &p, &cmp)
    });

    let mut c = array::from_fn(|_| alloc.fresh());
    let [prev, minuend @ ..] = red;
    let mut prev = prev.as_();

    for i in 0..c.len() {
        let tmp = sub2d(alloc, asm, minuend[i].as_(), &subtrahend[i].as_());
        // tmp + (prev >> 52)
        let tmp_plus_borrow = ssra2d(alloc, asm, tmp, &prev, 52);
        c[i] = tmp_plus_borrow;
        prev = &c[i];
    }

    c.map(|ci| ci.into_())
}
