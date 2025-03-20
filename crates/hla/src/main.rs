use std::array;

use hla::*;

// In this case we know that carry_add only needs to propagate 2
// but in other situations that is not the case.
// Seeing this ahead might be nice
// with a parameter and then use slice and generalize it
// Not everything has to have perfect types
pub fn carry_add(s: [&XReg; 2], add: &XReg) -> AtomicInstruction {
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

    let s: [XReg; 5] = array::from_fn(|_| asm.fresh());

    let sinst = smult(&mut asm, &s, a, b);
    println!("{:?}", asm);

    let old = sinst;

    let b = input(&mut asm, &mut mapping, &mut phys_registers, 5);
    let a_regs = array::from_fn(|ai| (6 + ai as u64));
    let a = a_regs.map(|pr| input(&mut asm, &mut mapping, &mut phys_registers, pr));
    let p: [XReg; 5] = array::from_fn(|_| asm.fresh());
    let p_inst = smult(&mut asm, &p, a, b);
    let new = p_inst;

    let mix = interleave(old, new);

    // Is there something we can do to tie off the outputs.
    // and to make sure it happens before drop_pass
    let mut seen = Seen::new();
    s.into_iter().for_each(|r| {
        seen.output_interface(r);
    });
    p.into_iter().for_each(|r| {
        seen.output_interface(r);
    });
    let mix = liveness_analysis(&mut seen, mix);
    println!("\nmix: {mix:?}");

    // Mapping and phys_registers seem to go togetehr
    let out = hardware_register_allocation(&mut mapping, &mut phys_registers, mix);
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
    t.into_iter().for_each(|r| {
        seen.output_interface(r);
    });
    let commands = liveness_analysis(&mut seen, inst);
    let out = hardware_register_allocation(&mut mapping, &mut phys_registers, commands);

    println!();
    print_instructions(&out);
}

fn main() {
    let mut asm = Allocator::new();
    let mut mapping = RegisterMapping::new();
    let mut register_bank = RegisterBank::new();
    let x = asm.fresh();

    let inst = mul(&x, &x, &x);
    print_instructions(&inst);
    let mut seen_registers = Seen::new();
    let commands = liveness_analysis(&mut seen_registers, inst);
    let physical_inst = hardware_register_allocation(&mut mapping, &mut register_bank, commands);
    print_instructions(&physical_inst);

    interleave_test();
    simd_test();
}

// How do other allocating algorithms pass things along like Vec?
// In this algorithm the inputs are not used after
pub fn smult(asm: &mut Allocator, s: &[XReg; 5], a: [XReg; 4], b: XReg) -> Vec<AtomicInstruction> {
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

// TODO initiliase constant
const C1: f64 = 0.;

// Whole vector is in registers, but that might not be great. Better to have it on the stack and load it from there
pub fn smult_noinit_simd(
    asm: &mut Allocator,
    _t: &[VReg; 6],
    s: VReg,
    v: [XReg; 5],
) -> Vec<AtomicInstruction> {
    // first do it as is written
    let tmp = asm.fresh();
    let splat_c1 = asm.fresh();
    let cc1 = asm.fresh();
    let fv0: VReg = asm.fresh();
    vec![
        ucvtf2d(&s, &s),
        mov(&tmp, C1.to_bits()),
        ucvtf(&fv0.as_d(), &v[0]),
        dup2d(&splat_c1, &tmp),
        mov16b(&cc1, &splat_c1),
        fmla2d(&cc1, &s, &fv0, 0),
    ]
}
