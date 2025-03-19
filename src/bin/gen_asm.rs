#![feature(iter_intersperse)]
use std::{
    array,
    collections::{BTreeSet, HashSet, VecDeque},
    mem,
};

use montgomery_reduction::emmart;

// See if these can be reduced. Took all of these as it was a u64 before
#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
enum TReg<R> {
    X(R),
    V(R),
}

impl<R: Copy> TReg<R> {
    fn reg(&self) -> R {
        match self {
            TReg::X(r) => *r,
            TReg::V(r) => *r,
        }
    }
}

type FreshRegister = u64;
// Vec<BlockInstr> - mixing -> Vec<Instr> -> Vec<InstrDrop> -> Vec<PhysInstr>
type AtomicInstruction = Vec<Instruction<FreshRegister>>;

#[derive(Debug)]
struct Instruction<R> {
    opcode: String,
    dest: TReg<R>,
    src: Vec<TReg<R>>,
    modifiers: Mod,
}

// Proper name for this
#[derive(Debug)]
enum Mod {
    None,
    Imm(u64),
    Idx(u64),
    Cond(String),
}

// TODO This could benefit from having really different types for FreshRegister and
// Hardware Register. The output could be made different for this
impl<R: std::fmt::Display> std::fmt::Display for TReg<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TReg::X(reg) => write!(f, "x{}", reg),
            TReg::V(reg) => write!(f, "v{}", reg),
        }
    }
}

impl<R: std::fmt::Display + Copy> Instruction<R> {
    // TODO this might be better as Display and/or using Formatter
    fn format_instruction(&self) -> String {
        let mut phys_regs = vec![self.dest];
        phys_regs.append(&mut self.src.clone());

        let regs: String = phys_regs
            .iter()
            .map(|x| x.to_string())
            .intersperse(", ".to_string())
            .collect();

        let extra = match &self.modifiers {
            Mod::None => String::new(),
            Mod::Imm(imm) => format!(", #{imm}"),
            Mod::Cond(cond) => format!(", {cond}"),
            Mod::Idx(idx) => format!("[{idx}]"),
        };
        let inst = &self.opcode;
        format!("{inst} {regs}{extra}")
    }

    /// Returns all the registers mentioned in the instructions.
    /// You can't assume the order in which they are returned.
    fn extract_registers(&self) -> Vec<TReg<R>> {
        let mut out = self.src.clone();
        out.push(self.dest);
        out
    }
}

#[derive(Debug)]
/// The result of the liveness analysis and it gives commands to the
/// hardware register allocator
enum LivenessCommand {
    Instr(Instruction<FreshRegister>),
    Drop(TReg<FreshRegister>),
}

impl From<Instruction<FreshRegister>> for LivenessCommand {
    fn from(instr: Instruction<FreshRegister>) -> Self {
        LivenessCommand::Instr(instr)
    }
}

// Define a macro for generating assembler instruction methods
// Don't write directly to the assembler as we would like to use these to construct grouped instructions
macro_rules! embed_asm {
    // For opcodeructions with 3 register parameters
    ($name:ident, 3) => {
        fn $name(dst: &XReg, a: &XReg, b: &XReg) -> crate::AtomicInstruction {
            vec![crate::Instruction {
                opcode: stringify!($name).to_string(),
                dest: dst.treg(),
                src: vec![a.treg(), b.treg()],
                modifiers: Mod::None,
            }]
        }
    };

    ($name:ident, $opcode:literal, 3) => {
        fn $name(dst: &VReg, src_a: &VReg, src_b: &VReg, i: u8) -> crate::AtomicInstruction {
            vec![crate::Instruction {
                opcode: $opcode.to_string(),
                dest: dst.treg(),
                src: vec![src_a.treg(), src_b.treg()],
                modifiers: Mod::Idx(i as u64),
            }]
        }
    };

    ($name:ident, $opcode:literal, 2) => {
        fn $name(dst: &VReg, src: &VReg) -> crate::AtomicInstruction {
            vec![crate::Instruction {
                opcode: $opcode.to_string(),
                dest: dst.treg(),
                src: vec![src.treg()],
                modifiers: Mod::None,
            }]
        }
    };

    ($name:ident, $opcode:literal, 2, m) => {
        fn $name(dst: &VReg, src: &XReg) -> crate::AtomicInstruction {
            vec![crate::Instruction {
                opcode: $opcode.to_string(),
                dest: dst.treg(),
                src: vec![src.treg()],
                modifiers: Mod::None,
            }]
        }
    };

    ($name:ident, 2, m) => {
        fn $name(dst: &VReg, src: &XReg) -> crate::AtomicInstruction {
            vec![crate::Instruction {
                opcode: stringify!($name).to_string(),
                dest: dst.treg(),
                src: vec![src.treg()],
                modifiers: Mod::None,
            }]
        }
    };

    ($name:ident, 1) => {
        fn $name(dst: &XReg, val: u64) -> crate::AtomicInstruction {
            vec![crate::Instruction {
                opcode: stringify!($name).to_string(),
                dest: dst.treg(),
                src: vec![],
                modifiers: Mod::Imm(val),
            }]
        }
    };

    // For opcodeructions with 1 register and 1 string parameter (cinc)
    ($name:ident, cond) => {
        fn $name(dst: &XReg, src: &XReg, condition: &str) -> crate::AtomicInstruction {
            vec![crate::Instruction {
                opcode: stringify!($name).to_string(),
                dest: dst.treg(),
                src: vec![src.treg()],
                modifiers: Mod::Cond(condition.to_string()),
            }]
        }
    };
}

embed_asm!(mov, 1);
embed_asm!(mul, 3);
embed_asm!(umulh, 3);
embed_asm!(adds, 3);
embed_asm!(adcs, 3);
embed_asm!(cinc, cond);
// mov now doesn't support immediates. Not sure if mov16 actually ever can
embed_asm!(mov16b, "mov.16b", 2);
embed_asm!(ucvtf2d, "ucvtf.2d", 2);
embed_asm!(dup2d, "dup.2d", 2, m);
// Could use another but this works too
embed_asm!(ucvtf, 2, m);
embed_asm!(fmla2d, "fmla.2d", 3);

// Different types as I don't want to have type erasure on the
// functions themselves
struct XReg {
    // Maybe make reg a usize instead
    reg: u64,
}

struct VReg {
    // Maybe make reg a usize instead
    reg: u64,
}

trait Reg {
    fn new(reg: u64) -> Self;
    // Created for input and output tying
    fn treg(&self) -> TReg<FreshRegister>;
    fn register_type() -> RegisterType;
}

#[derive(Debug)]
enum RegisterType {
    X,
    V,
}

impl Reg for XReg {
    fn treg(&self) -> TReg<FreshRegister> {
        TReg::X(self.reg)
    }

    fn new(reg: u64) -> Self {
        Self { reg }
    }

    fn register_type() -> RegisterType {
        RegisterType::X
    }
}

impl Reg for VReg {
    fn new(reg: u64) -> Self {
        Self { reg }
    }
    fn treg(&self) -> TReg<FreshRegister> {
        TReg::V(self.reg)
    }

    fn register_type() -> RegisterType {
        RegisterType::V
    }
}

#[derive(Debug)]
struct Allocator {
    // It's about unique counters so we use the counter for both
    // q and v registers
    // this makes it easier to read the assembly
    fresh: u64,
}

impl Allocator {
    fn fresh<T: Reg>(&mut self) -> T {
        let x = self.fresh;
        self.fresh += 1;
        T::new(x)
    }

    fn new() -> Self {
        Self { fresh: 0 }
    }
}

// How do other allocating algorithms pass things along like Vec?
// In this algorithm the inputs are not used after
fn smult(asm: &mut Allocator, s: &[XReg; 5], a: [XReg; 4], b: XReg) -> Vec<AtomicInstruction> {
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

// In this case we know that carry_add only needs to propagate 2
// but in other situations that is not the case.
// Seeing this ahead might be nice
// with a parameter and then use slice and generalize it
// Not everything has to have perfect types
fn carry_add(s: [&XReg; 2], add: &XReg) -> AtomicInstruction {
    vec![adds(s[0], s[0], add), cinc(s[1], s[1], "hs")]
        .into_iter()
        .flatten()
        .collect()
}

// Whole vector is in registers, but that might not be great. Better to have it on the stack and load it from there
fn smult_noinit_simd(
    asm: &mut Allocator,
    _t: &[VReg; 6],
    s: VReg,
    v: [XReg; 5],
) -> Vec<AtomicInstruction> {
    // first do it as is written
    let tmp = asm.fresh();
    let splat_c1 = asm.fresh();
    let cc1 = asm.fresh();
    let fv0 = asm.fresh();
    vec![
        ucvtf2d(&s, &s),
        mov(&tmp, emmart::C1.to_bits()),
        ucvtf(&fv0, &v[0]),
        dup2d(&splat_c1, &tmp),
        mov16b(&cc1, &splat_c1),
        fmla2d(&cc1, &s, &fv0, 0),
    ]
}

impl std::fmt::Display for XReg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x{}", self.reg)
    }
}

impl std::fmt::Debug for XReg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x{}", self.reg)
    }
}

// Add another struct to prevent things from being created
// Make a struct around here such that it can't be copied
// THe phys_register file is the one that creates them
type HardwareRegister = u64;

// No Clone as the state of one free reg
// does not make sense as the state of another free reg
#[derive(PartialEq, Debug)]
enum RegisterState {
    Unassigned,
    Map(TReg<HardwareRegister>),
    Dropped,
}

// Both Reg and PhysicalReg are not supposed to be copied.
// BUt for the interface we do need to map them some way
// This can also be done as part of the initialisation
// A way out of the ordering for now is to just make it a big enough size
fn input<T: Reg>(
    asm: &mut Allocator,
    mapping: &mut RegisterMapping,
    phys_registers: &mut RegisterBank,
    phys: u64,
) -> T {
    let fresh: T = asm.fresh();
    let treg = fresh.treg();

    match T::register_type() {
        RegisterType::X => {
            if !phys_registers.x.remove(&phys) {
                panic!("Register x{} is already in use", phys)
            }
            mapping[treg] = RegisterState::Map(TReg::X(phys));
        }
        RegisterType::V => {
            if !phys_registers.v.remove(&phys) {
                panic!("Register v{} is already in use", phys)
            }
            mapping[treg] = RegisterState::Map(TReg::V(phys));
        }
    }

    fresh
}

type Seen = HashSet<TReg<FreshRegister>>;

fn output_interface(seen: &mut Seen, fresh: impl Reg) {
    seen.insert(fresh.treg());
}

#[derive(Debug)]
struct RegisterBank {
    x: BTreeSet<HardwareRegister>,
    v: BTreeSet<HardwareRegister>,
}

impl RegisterBank {
    fn new() -> Self {
        Self {
            x: BTreeSet::from_iter(Vec::from_iter(0..=30)),
            v: BTreeSet::from_iter(Vec::from_iter(0..=30)),
        }
    }
    /// Returns
    fn insert(&mut self, phys_reg: TReg<HardwareRegister>) -> bool {
        match phys_reg {
            TReg::X(phys_reg) => self.x.insert(phys_reg),
            TReg::V(phys_reg) => self.v.insert(phys_reg),
        }
    }
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
    let mut seen = HashSet::new();
    s.into_iter().for_each(|r| output_interface(&mut seen, r));
    p.into_iter().for_each(|r| output_interface(&mut seen, r));
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
    let inst = ssimd.into_iter().flatten().collect();
    print_instructions(&inst);

    let mut seen = HashSet::new();
    // t.into_iter().for_each(|r| output_interface(&mut seen, r));
    let out = hardware_register_allocation(
        &mut mapping,
        &mut phys_registers,
        liveness_analysis(&mut seen, inst),
    );

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
    let mut seen_registers = HashSet::new();
    let commands = liveness_analysis(&mut seen_registers, inst);
    let physical_inst = hardware_register_allocation(&mut mapping, &mut register_bank, commands);
    print_instructions(&physical_inst);

    simd_test();
}

fn interleave(
    lhs: Vec<AtomicInstruction>,
    rhs: Vec<AtomicInstruction>,
) -> Vec<Instruction<FreshRegister>> {
    lhs.into_iter()
        .zip(rhs)
        .flat_map(|(a, b)| [a, b])
        .flatten()
        .collect()
}

#[derive(Debug)]
struct RegisterMapping(Vec<RegisterState>);

impl RegisterMapping {
    fn new() -> Self {
        // Needs to be equal to the number of free register in the allocator once it is finished
        Self(
            std::iter::repeat_with(|| RegisterState::Unassigned)
                .take(30)
                .collect::<Vec<_>>(),
        )
    }

    // Get the physical register for a source register
    fn get_register(&self, fresh: TReg<FreshRegister>) -> TReg<HardwareRegister> {
        match &self[fresh] {
            RegisterState::Unassigned => unreachable!("{fresh:?} has not been assigned yet"),
            RegisterState::Map(reg) => *reg,
            RegisterState::Dropped => unreachable!("{fresh:?} already has been dropped"),
        }
    }

    // Get or allocate a register
    fn get_or_allocate_register(
        &mut self,
        register_bank: &mut RegisterBank,
        fresh: TReg<FreshRegister>,
    ) -> TReg<HardwareRegister> {
        match &self[fresh] {
            RegisterState::Unassigned => match fresh {
                TReg::X(_) => {
                    let reg = register_bank.x.pop_first().expect("ran out of registers");
                    self[fresh] = RegisterState::Map(TReg::X(reg));
                    TReg::X(reg)
                }
                TReg::V(_) => {
                    let reg = register_bank.v.pop_first().expect("ran out of registers");
                    self[fresh] = RegisterState::Map(TReg::V(reg));
                    TReg::V(reg)
                }
            },
            RegisterState::Map(reg) => *reg,
            RegisterState::Dropped => unreachable!("{fresh:?} already has been dropped"),
        }
    }

    // Once a fresh register goes out of scope the hardware register that was assigned to that fresh register
    // can be returned to the register bank.
    fn free_register(
        &mut self,
        register_bank: &mut RegisterBank,
        fresh: TReg<FreshRegister>,
    ) -> bool {
        let old = mem::replace(&mut self[fresh], RegisterState::Dropped);

        match old {
            RegisterState::Unassigned => {
                unreachable!("There should never be a drop before the register has been assigned")
            }
            RegisterState::Map(reg) => {
                let new = register_bank.insert(reg);
                assert!(
                    new,
                    "hardware:{reg} is assigned to more than one fresh register. "
                );
                new
            }
            RegisterState::Dropped => {
                unreachable!("A register that has been dropped can't be dropped again")
            }
        }
    }
}

impl std::ops::Index<TReg<FreshRegister>> for RegisterMapping {
    type Output = RegisterState;

    fn index(&self, idx: TReg<FreshRegister>) -> &Self::Output {
        &self.0[idx.reg() as usize]
    }
}

impl std::ops::IndexMut<TReg<FreshRegister>> for RegisterMapping {
    fn index_mut(&mut self, idx: TReg<FreshRegister>) -> &mut Self::Output {
        &mut self.0[idx.reg() as usize]
    }
}

fn liveness_analysis(
    seen_registers: &mut Seen,
    instructions: Vec<Instruction<FreshRegister>>,
) -> VecDeque<LivenessCommand> {
    let mut commands = VecDeque::new();
    for instruction in instructions.into_iter().rev() {
        for register in instruction.extract_registers() {
            if seen_registers.insert(register) {
                commands.push_front(LivenessCommand::Drop(register));
            }
        }
        commands.push_front(instruction.into());
    }
    commands
}

fn hardware_register_allocation(
    mapping: &mut RegisterMapping,
    register_bank: &mut RegisterBank,
    commands: VecDeque<LivenessCommand>,
) -> Vec<Instruction<HardwareRegister>> {
    let f = |cmd| {
        // println!("LivenessCommand: {cmd:?}");
        // println!("mapping: {mapping:?}");
        // println!("bank: {register_bank:?}");
        match cmd {
            LivenessCommand::Instr(mut inst) => {
                // Resolve registers to physical hardware registers
                inst.dest = mapping.get_or_allocate_register(register_bank, inst.dest);
                inst.src = inst
                    .src
                    .into_iter()
                    .map(|s| mapping.get_register(s))
                    .collect();
                Some(inst)
            }
            LivenessCommand::Drop(fresh) => {
                mapping.free_register(register_bank, fresh);
                None
            }
        }
    };

    commands.into_iter().filter_map(f).collect()
}

fn print_instructions<R: std::fmt::Display + Copy>(instrs: &Vec<Instruction<R>>) {
    instrs
        .iter()
        .for_each(|inst| println!("{}", inst.format_instruction()));
}
