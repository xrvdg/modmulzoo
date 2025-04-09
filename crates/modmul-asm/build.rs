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

    // Is there something we n do to tie off the outputs.
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
    let operands = backend_rust(mapping, &input_hw_registers, &[&output_hw_registers], &out);
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

fn build_u256_to_u260_shl2_simd() {
    fn input_setup(
        mut alloc: Allocator,
        mapping: &mut RegisterMapping,
        phys_registers: &mut RegisterBank,
        asm: &mut Assembler,
    ) -> (
        Vec<TypedSizedRegister<HardwareRegister>>,
        Vec<Reg<Simd<u64, 2>>>,
    ) {
        let limbs = array::from_fn(|i| input(&mut alloc, mapping, phys_registers, i as u64));

        let input_hw_registers: Vec<_> = limbs
            .iter()
            .filter_map(|reg| mapping.output_register(reg))
            .collect();

        let res = u256_to_u260_shl2_simd(&mut alloc, asm, limbs);

        (input_hw_registers, Vec::from(res))
    }
    build_func("u256_to_u260_shl2_simd", input_setup);
}

fn build_u260_to_u256_simd() {
    fn input_setup(
        mut alloc: Allocator,
        mapping: &mut RegisterMapping,
        phys_registers: &mut RegisterBank,
        asm: &mut Assembler,
    ) -> (
        Vec<TypedSizedRegister<HardwareRegister>>,
        Vec<Reg<Simd<u64, 2>>>,
    ) {
        let limbs = array::from_fn(|i| input(&mut alloc, mapping, phys_registers, i as u64));

        let input_hw_registers: Vec<_> = limbs
            .iter()
            .filter_map(|reg| mapping.output_register(reg))
            .collect();

        let res = u260_to_u256_simd(&mut alloc, asm, limbs);

        (input_hw_registers, Vec::from(res))
    }
    build_func("u260_to_u256_simd", input_setup);
}

fn build_vmultadd_noinit_simd() {
    fn input_setup(
        mut alloc: Allocator,
        mapping: &mut RegisterMapping,
        phys_registers: &mut RegisterBank,
        asm: &mut Assembler,
    ) -> (
        Vec<TypedSizedRegister<HardwareRegister>>,
        Vec<Reg<Simd<u64, 2>>>,
    ) {
        let t = array::from_fn(|i| input(&mut alloc, mapping, phys_registers, i as u64));
        let a =
            array::from_fn(|i| input(&mut alloc, mapping, phys_registers, (i + t.len()) as u64));
        let b = array::from_fn(|i| {
            input(
                &mut alloc,
                mapping,
                phys_registers,
                (i + t.len() + a.len()) as u64,
            )
        }); // Assuming b starts after a

        let input_hw_registers: Vec<_> = t
            .iter()
            .chain(a.iter())
            .chain(b.iter())
            .filter_map(|reg| mapping.output_register(reg))
            .collect();

        let res = vmultadd_noinit_simd(&mut alloc, asm, t, a, b);

        (input_hw_registers, Vec::from(res))
    }
    build_func("vmultadd_noinit_simd", input_setup);
}

fn build_single_step_simd() {
    fn input_setup(
        mut alloc: Allocator,
        mapping: &mut RegisterMapping,
        phys_registers: &mut RegisterBank,
        asm: &mut Assembler,
    ) -> (
        Vec<TypedSizedRegister<HardwareRegister>>,
        Vec<Reg<Simd<u64, 2>>>,
    ) {
        let a = array::from_fn(|i| input(&mut alloc, mapping, phys_registers, i as u64));
        let b =
            array::from_fn(|i| input(&mut alloc, mapping, phys_registers, (i + a.len()) as u64)); // Assuming b starts after a

        let input_hw_registers: Vec<_> = a
            .iter()
            .chain(b.iter())
            .filter_map(|reg| mapping.output_register(reg))
            .collect();

        let res = single_step_simd(&mut alloc, asm, a, b);

        (input_hw_registers, Vec::from(res))
    }
    build_func("single_step_simd", input_setup);
}

fn build_reduce_ct_simd() {
    fn input_setup(
        mut alloc: Allocator,
        mapping: &mut RegisterMapping,
        phys_registers: &mut RegisterBank,
        asm: &mut Assembler,
    ) -> (
        Vec<TypedSizedRegister<HardwareRegister>>,
        Vec<Reg<Simd<u64, 2>>>,
    ) {
        let red = array::from_fn(|i| input(&mut alloc, mapping, phys_registers, i as u64));

        let input_hw_registers: Vec<_> = red
            .iter()
            .filter_map(|reg| mapping.output_register(reg))
            .collect();

        let res = reduce_ct_simd(&mut alloc, asm, red);

        (input_hw_registers, Vec::from(res))
    }
    build_func("reduce_ct_simd", input_setup);
}

fn build_interleaved(label: &str) {
    let mut alloc = Allocator::new();
    let mut mapping = RegisterMapping::new();
    let mut phys_registers = RegisterBank::new();

    let mut fst_asm = Assembler::new();
    let (mut fst_input_hw_registers, fst_regs) = {
        let a = array::from_fn(|i| input(&mut alloc, &mut mapping, &mut phys_registers, i as u64));
        let b = array::from_fn(|i| {
            input(
                &mut alloc,
                &mut mapping,
                &mut phys_registers,
                (a.len() + i) as u64,
            )
        });

        let input_hw_registers: Vec<_> = a
            .iter()
            .chain(&b)
            .filter_map(|reg| mapping.output_register(&reg))
            .collect();

        let s = single_step(&mut alloc, &mut fst_asm, &a, &b);
        (input_hw_registers, Vec::from(s))
    };

    let mut snd_asm = Assembler::new();

    let (snd_input_hw_registers, snd_regs) = {
        let offset = fst_input_hw_registers.len();
        let a = array::from_fn(|i| {
            input(
                &mut alloc,
                &mut mapping,
                &mut phys_registers,
                (offset + i) as u64,
            )
        });
        let b = array::from_fn(|i| {
            input(
                &mut alloc,
                &mut mapping,
                &mut phys_registers,
                (offset + i + a.len()) as u64,
            )
        }); // Assuming b starts after a

        let input_hw_registers: Vec<_> = a
            .iter()
            .chain(b.iter())
            .filter_map(|reg| mapping.output_register(reg))
            .collect();

        let res = single_step_simd(&mut alloc, &mut snd_asm, a, b);

        (input_hw_registers, Vec::from(res))
    };

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

    let releases = liveness_analysis(&mut seen, &mixed);

    let out = hardware_register_allocation(&mut mapping, &mut phys_registers, mixed, releases);

    fst_input_hw_registers.extend(snd_input_hw_registers);

    let input_hw_registers = fst_input_hw_registers;

    let fst_output_hw_registers: Vec<_> = fst_regs
        .iter()
        .filter_map(|reg| mapping.output_register(reg))
        .collect();

    let snd_output_hw_registers: Vec<_> = snd_regs
        .iter()
        .filter_map(|reg| mapping.output_register(reg))
        .collect();

    let txt = backend_global(label, &out);

    // Write this info in the assembly file
    let operands = backend_rust(
        mapping,
        &input_hw_registers,
        &[&fst_output_hw_registers, &snd_output_hw_registers],
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
    build_smul_add();
    build_schoolmethod();
    build_single_step();
    build_u256_to_u260_shl2_simd();
    build_u260_to_u256_simd();
    build_vmultadd_noinit_simd();
    build_single_step_simd();
    build_reduce_ct_simd();
    build_interleaved("single_step_interleaved");
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
    limbs: [Reg<Simd<u64, 2>>; 4],
) -> [Reg<Simd<u64, 2>>; 5] {
    let [l0, l1, l2, l3] = limbs;
    let mask = {
        let val = mov(alloc, asm, MASK52);
        let mask = dup2d(alloc, asm, &val);
        mask
    };

    let shifted_l1 = shl2d(alloc, asm, &l1, 14);
    let shifted_l2 = shl2d(alloc, asm, &l2, 26);
    let shifted_l3 = shl2d(alloc, asm, &l3, 38);

    let shifted_ol0 = shl2d(alloc, asm, &l0, 2);
    let shifted_ol1 = usra2d(alloc, asm, shifted_l1, &l0, 50);
    let shifted_ol2 = usra2d(alloc, asm, shifted_l2, &l1, 38);
    let shifted_ol3 = usra2d(alloc, asm, shifted_l3, &l2, 26);

    [
        and16(alloc, asm, &shifted_ol0, &mask),
        and16(alloc, asm, &shifted_ol1, &mask),
        and16(alloc, asm, &shifted_ol2, &mask),
        and16(alloc, asm, &shifted_ol3, &mask),
        ushr2d(alloc, asm, &l3, 14),
    ]
}

fn u260_to_u256_simd(
    alloc: &mut Allocator,
    asm: &mut Assembler,
    limbs: [Reg<Simd<u64, 2>>; 5],
) -> [Reg<Simd<u64, 2>>; 4] {
    let [l0, l1, l2, l3, l4] = limbs;

    let shifted_l1 = shl2d(alloc, asm, &l1, 52);
    let shifted_l2 = shl2d(alloc, asm, &l2, 40);
    let shifted_l3 = shl2d(alloc, asm, &l3, 28);
    let shifted_l4 = shl2d(alloc, asm, &l4, 16);

    [
        orr16(alloc, asm, &l0, &shifted_l1),
        usra2d(alloc, asm, shifted_l2, &l1, 12),
        usra2d(alloc, asm, shifted_l3, &l2, 24),
        usra2d(alloc, asm, shifted_l4, &l3, 36),
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
    mut t: [Reg<Simd<u64, 2>>; 10],
    a: [Reg<Simd<u64, 2>>; 5],
    b: [Reg<Simd<u64, 2>>; 5],
) -> [Reg<Simd<u64, 2>>; 10] {
    let c1 = mov(alloc, asm, C1.to_bits());
    let c1 = dup2d(alloc, asm, &c1);

    // Alternative is c2 = c1 + 1; This requires a change to add to support immediate
    let c2 = load_const(alloc, asm, C2.to_bits());
    let c2 = dup2d(alloc, asm, &c2);

    for i in 0..a.len() {
        let ai = ucvtf2d(alloc, asm, &a[i]);
        for j in 0..b.len() {
            let bj = ucvtf2d(alloc, asm, &b[j]);
            let lc1 = mov16b(alloc, asm, &c1);
            let lc2 = mov16b(alloc, asm, &c2);

            let hi = fmla2d(alloc, asm, lc1.into_(), &ai, &bj);
            let tmp = fsub2d(alloc, asm, &lc2.into_(), &hi);
            let lo = fmla2d(alloc, asm, tmp, &ai, &bj);

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
    s: Reg<Simd<u64, 2>>,
    v: [Reg<u64>; 5],
) -> [Reg<Simd<u64, 2>>; 6] {
    let c1 = mov(alloc, asm, C1.to_bits());
    let c1 = dup2d(alloc, asm, &c1);

    // Alternative is c2 = c1 + 1; This requires a change to add to support immediate
    let c2 = load_const(alloc, asm, C2.to_bits());
    let c2 = dup2d(alloc, asm, &c2);

    let s = ucvtf2d(alloc, asm, &s);

    for i in 0..v.len() {
        let vi = ucvtf(alloc, asm, &v[i]);
        let lc1 = mov16b(alloc, asm, &c1);
        let lc2 = mov16b(alloc, asm, &c2);

        let hi = fmla2d(alloc, asm, lc1.into_(), &s, &vi.as_simd()._0());
        let tmp = fsub2d(alloc, asm, &lc2.into_(), &hi);
        let lo = fmla2d(alloc, asm, tmp, &s, &vi.as_simd()._0());

        t[i + 1] = add2d(alloc, asm, &t[i + 1], &hi.into_());
        t[i] = add2d(alloc, asm, &t[i], &lo.into_());
    }
    t
}

/// Misses the transposing to make it easier on the registers for the next steps
fn single_step_simd(
    alloc: &mut Allocator,
    asm: &mut Assembler,
    a: [Reg<Simd<u64, 2>>; 4],
    b: [Reg<Simd<u64, 2>>; 4],
) -> [Reg<Simd<u64, 2>>; 4] {
    let a = u256_to_u260_shl2_simd(alloc, asm, a);
    let b = u256_to_u260_shl2_simd(alloc, asm, b);
    let t = make_initials(alloc, asm);

    let [t0, t1, t2, t3, t4, t5, t6, t7, t8, t9] = vmultadd_noinit_simd(alloc, asm, t, a, b);

    let t1 = usra2d(alloc, asm, t1, &t0, 52);
    let t2 = usra2d(alloc, asm, t2, &t1, 52);
    let t3 = usra2d(alloc, asm, t3, &t2, 52);
    let t4 = usra2d(alloc, asm, t4, &t3, 52);

    let t6_10 = [t4, t5, t6, t7, t8, t9];

    let mask = mov(alloc, asm, MASK52);
    let mask_simd = dup2d(alloc, asm, &mask);

    let t0 = and16(alloc, asm, &t0, &mask_simd);
    let t1 = and16(alloc, asm, &t1, &mask_simd);
    let t2 = and16(alloc, asm, &t2, &mask_simd);
    let t3 = and16(alloc, asm, &t3, &mask_simd);

    // loading rho interleaved with multiplication to prevent to prevent allocation a lot of X-registers
    let rho_4 = RHO_4.map(|c| load_const(alloc, asm, c));
    let r0 = smultadd_noinit_simd(alloc, asm, t6_10, t0, rho_4);
    let rho_3 = RHO_3.map(|c| load_const(alloc, asm, c));
    let r1 = smultadd_noinit_simd(alloc, asm, r0, t1, rho_3);
    let rho_2 = RHO_2.map(|c| load_const(alloc, asm, c));
    let r2 = smultadd_noinit_simd(alloc, asm, r1, t2, rho_2);
    let rho_1 = RHO_1.map(|c| load_const(alloc, asm, c));
    let s = smultadd_noinit_simd(alloc, asm, r2, t3, rho_1);

    // Could be replaced with fmul, but the rust compiler generates this
    let u52_np0 = load_const(alloc, asm, U52_NP0);
    let s00 = umov(alloc, asm, &s[0]._d0());
    let s01 = umov(alloc, asm, &s[0]._d1());
    let m0 = mul(alloc, asm, &s00, &u52_np0);
    let m1 = mul(alloc, asm, &s01, &u52_np0);

    let m0 = and(alloc, asm, &m0, &mask);
    let m1 = and(alloc, asm, &m1, &mask);
    let m = load_tuple(alloc, asm, m0, m1);

    let u52_p = U52_P.map(|i| load_const(alloc, asm, i));

    let s = smultadd_noinit_simd(alloc, asm, s, m, u52_p);

    let rs = reduce_ct_simd(alloc, asm, s);

    u260_to_u256_simd(alloc, asm, rs)
}

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

    let mask52 = mov(alloc, asm, MASK52);
    let mask52 = dup2d(alloc, asm, &mask52);

    let mut c = array::from_fn(|_| alloc.fresh());
    let [prev, minuend @ ..] = red;
    let mut prev = prev.into_();

    for i in 0..c.len() {
        let tmp = sub2d(alloc, asm, minuend[i].as_(), &subtrahend[i].as_());
        // tmp + (prev >> 52)
        let tmp_plus_borrow = ssra2d(alloc, asm, tmp, &prev, 52);
        c[i] = and16(alloc, asm, tmp_plus_borrow.as_(), &mask52);
        prev = tmp_plus_borrow;
    }

    c.map(|ci| ci.into_())
}
