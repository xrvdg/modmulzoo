#![feature(iter_intersperse)]
use std::{
    collections::{BTreeSet, HashSet, VecDeque},
    io::Write,
    mem::{self},
};

// See if these can be reduced. Took all of these as it was a u64 before

impl<S> TypedRegisterF<FreshRegister, S> {
    // Should only be seen by RegisterMapping
    // Could have been used for HardwareRegister, but it's better to convert types
    // Only for 'internal' use
    fn as_fresh(&self) -> &FreshRegister {
        match self {
            TypedRegisterF::Scalar(r) => r,
            TypedRegisterF::Vector(r, _) => r,
        }
    }
}

// Vec<BlockInstr> - mixing -> Vec<Instr> -> Vec<InstrDrop> -> Vec<PhysInstr>
pub type AtomicInstruction = Vec<InstructionF<FreshRegister>>;
pub type Instruction = InstructionF<FreshRegister>;

#[derive(Debug)]
pub struct InstructionF<R> {
    opcode: String,
    dest: TypedSizedRegister<R>,
    src: Vec<TypedSizedRegister<R>>,
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
impl<R: std::fmt::Display> std::fmt::Display for TypedRegisterF<R, VectorSizes> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypedRegisterF::Scalar(reg) => write!(f, "x{}", reg),
            TypedRegisterF::Vector(reg, vectorsize) => write!(f, "{vectorsize}{reg}"),
        }
    }
}

impl std::fmt::Display for VectorSizes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VectorSizes::V => write!(f, "v"),
            VectorSizes::D => write!(f, "d"),
        }
    }
}

impl<R: std::fmt::Display + Copy> InstructionF<R> {
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
    fn extract_registers(&self) -> Vec<TypedSizedRegister<R>> {
        let mut out = self.src.clone();
        out.push(self.dest);
        out
    }
}

impl From<InstructionF<FreshRegister>> for LivenessCommand {
    fn from(instr: InstructionF<FreshRegister>) -> Self {
        LivenessCommand::Instr(instr)
    }
}

// Define a macro for generating assembler instruction methods
// Don't write directly to the assembler as we would like to use these to construct grouped instructions
macro_rules! embed_asm {
    // For opcodeructions with 3 register parameters
    ($name:ident, 3) => {
        pub fn $name(dst: &XReg, a: &XReg, b: &XReg) -> crate::AtomicInstruction {
            vec![crate::Instruction {
                opcode: stringify!($name).to_string(),
                dest: dst.to_typed_register(),
                src: vec![a.to_typed_register(), b.to_typed_register()],
                modifiers: Mod::None,
            }]
        }
    };

    ($name:ident, $opcode:literal, 3) => {
        pub fn $name(dst: &VReg, src_a: &VReg, src_b: &VReg, i: u8) -> crate::AtomicInstruction {
            vec![crate::Instruction {
                opcode: $opcode.to_string(),
                dest: dst.to_typed_register(),
                src: vec![src_a.to_typed_register(), src_b.to_typed_register()],
                modifiers: Mod::Idx(i as u64),
            }]
        }
    };

    ($name:ident, $opcode:literal, 2) => {
        pub fn $name(dst: &VReg, src: &VReg) -> crate::AtomicInstruction {
            vec![crate::Instruction {
                opcode: $opcode.to_string(),
                dest: dst.to_typed_register(),
                src: vec![src.to_typed_register()],
                modifiers: Mod::None,
            }]
        }
    };

    ($name:ident, $opcode:literal, 2, m) => {
        pub fn $name(dst: &VReg, src: &XReg) -> crate::AtomicInstruction {
            vec![crate::Instruction {
                opcode: $opcode.to_string(),
                dest: dst.to_typed_register(),
                src: vec![src.to_typed_register()],
                modifiers: Mod::None,
            }]
        }
    };

    ($name:ident, 2, m) => {
        pub fn $name<T: Reg64Bit + AliasedRegister>(
            dst: &DReg,
            src: &T,
        ) -> crate::AtomicInstruction {
            vec![crate::Instruction {
                opcode: stringify!($name).to_string(),
                dest: dst.to_typed_register(),
                src: vec![src.to_typed_register()],
                modifiers: Mod::None,
            }]
        }
    };

    ($name:ident, 1) => {
        pub fn $name(dst: &XReg, val: u64) -> crate::AtomicInstruction {
            vec![crate::Instruction {
                opcode: stringify!($name).to_string(),
                dest: dst.to_typed_register(),
                src: vec![],
                modifiers: Mod::Imm(val),
            }]
        }
    };

    // For opcodeructions with 1 register and 1 string parameter (cinc)
    ($name:ident, cond) => {
        pub fn $name(dst: &XReg, src: &XReg, condition: &str) -> crate::AtomicInstruction {
            vec![crate::Instruction {
                opcode: stringify!($name).to_string(),
                dest: dst.to_typed_register(),
                src: vec![src.to_typed_register()],
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
pub struct XReg {
    // Maybe make reg a usize instead
    reg: u64,
}

pub struct VReg {
    // Maybe make reg a usize instead
    reg: u64,
}

pub struct DReg<'a> {
    reg: &'a u64,
}

pub trait Reg64Bit {}
impl Reg64Bit for XReg {}
impl<'a> Reg64Bit for DReg<'a> {}

pub trait AllocatableRegister: AllocatableRegisterSealed {}
impl AllocatableRegister for VReg {}
impl AllocatableRegister for XReg {}

pub trait AliasedRegister: AliasedRegisterSealed {}
impl AliasedRegister for XReg {}
impl<'a> AliasedRegister for DReg<'a> {}
impl AliasedRegister for VReg {}

/// Sealed traits for Aliased and Allocatable registers and LiveCommand
/// These are sealed as they have the fresh register which should not be
/// available to users, but we do need trait to handle the different registers
/// and allow for the boundary code to be able to use it.
/// Might turn out to be too limiting if we want to allow the user to construct Instructions
/// manually
mod private {

    #[derive(Debug)]
    pub enum RegisterType {
        X,
        V,
    }
    pub type FreshRegister = u64;
    #[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
    pub enum VectorSizes {
        V,
        D,
    }

    pub type TypedRegister<R> = TypedRegisterF<R, ()>;
    pub type TypedSizedRegister<R> = TypedRegisterF<R, VectorSizes>;

    #[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
    pub enum TypedRegisterF<R, Sizes> {
        Scalar(R),
        Vector(R, Sizes),
    }

    pub trait AllocatableRegisterSealed {
        // For use in the allocator
        fn new(reg: u64) -> Self;
        // For use in input
        fn register_type() -> RegisterType;
    }
    pub trait AliasedRegisterSealed {
        // internal, but on the border so input, output and inside the macros
        fn to_typed_register(&self) -> TypedSizedRegister<FreshRegister>;
    }

    use crate::InstructionF;

    /// The result of the liveness analysis and it gives commands to the
    /// hardware register allocator
    #[derive(Debug)]
    pub enum LivenessCommand {
        Instr(InstructionF<FreshRegister>),
        Drop(TypedRegisterF<FreshRegister, ()>),
    }
}
use private::*;

impl AllocatableRegisterSealed for XReg {
    fn new(reg: u64) -> Self {
        Self { reg }
    }

    fn register_type() -> RegisterType {
        RegisterType::X
    }
}

impl AliasedRegisterSealed for XReg {
    fn to_typed_register(&self) -> TypedSizedRegister<FreshRegister> {
        TypedRegisterF::Scalar(self.reg)
    }
}

impl AllocatableRegisterSealed for VReg {
    fn new(reg: u64) -> Self {
        Self { reg }
    }
    fn register_type() -> RegisterType {
        RegisterType::V
    }
}

impl AliasedRegisterSealed for VReg {
    fn to_typed_register(&self) -> TypedSizedRegister<FreshRegister> {
        TypedRegisterF::Vector(self.reg, VectorSizes::V)
    }
}

impl<'a> AliasedRegisterSealed for DReg<'a> {
    fn to_typed_register(&self) -> TypedSizedRegister<FreshRegister> {
        TypedRegisterF::Vector(*self.reg, VectorSizes::D)
    }
}

impl VReg {
    // Should this be an AsRef
    pub fn as_d<'a>(&'a self) -> DReg<'a> {
        DReg { reg: &self.reg }
    }
}

#[derive(Debug)]
pub struct Allocator {
    // It's about unique counters so we use the counter for both
    // q and v registers
    // this makes it easier to read the assembly
    fresh: u64,
}

impl Allocator {
    pub fn fresh<T: AllocatableRegister>(&mut self) -> T {
        let x = self.fresh;
        self.fresh += 1;
        T::new(x)
    }

    pub fn new() -> Self {
        Self { fresh: 0 }
    }
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
    Assigned(TypedSizedRegister<HardwareRegister>),
    Dropped,
}

// Both Reg and PhysicalReg are not supposed to be copied.
// BUt for the interface we do need to map them some way
// This can also be done as part of the initialisation
// A way out of the ordering for now is to just make it a big enough size
pub fn input<T: AllocatableRegister + AliasedRegister>(
    asm: &mut Allocator,
    mapping: &mut RegisterMapping,
    phys_registers: &mut RegisterBank,
    phys: u64,
) -> T {
    let fresh: T = asm.fresh();
    let treg = fresh.to_typed_register();

    match T::register_type() {
        RegisterType::X => {
            if !phys_registers.x.remove(&phys) {
                panic!("Register x{} is already in use", phys)
            }
            *mapping.index_mut(treg) = RegisterState::Assigned(TypedRegisterF::Scalar(phys));
        }
        RegisterType::V => {
            if !phys_registers.v.remove(&phys) {
                panic!("Register v{} is already in use", phys)
            }
            *mapping.index_mut(treg) =
                RegisterState::Assigned(TypedRegisterF::Vector(phys, VectorSizes::V));
        }
    }

    fresh
}

pub struct Seen(HashSet<TypedRegister<FreshRegister>>);

impl Seen {
    pub fn new() -> Self {
        Self(HashSet::new())
    }

    pub fn output_interface(&mut self, fresh: &impl AliasedRegister) -> bool {
        self.insert(drop_size(fresh.to_typed_register()))
    }

    fn insert(&mut self, fresh: TypedRegister<FreshRegister>) -> bool {
        self.0.insert(fresh)
    }
}

fn drop_size<R>(t: TypedSizedRegister<R>) -> TypedRegister<R> {
    match t {
        TypedRegisterF::Scalar(reg) => TypedRegisterF::Scalar(reg),
        TypedRegisterF::Vector(reg, _) => TypedRegisterF::Vector(reg, ()),
    }
}

#[derive(Debug)]
pub struct RegisterBank {
    x: BTreeSet<HardwareRegister>,
    v: BTreeSet<HardwareRegister>,
}

impl RegisterBank {
    pub fn new() -> Self {
        Self {
            x: BTreeSet::from_iter(Vec::from_iter(0..=30)),
            v: BTreeSet::from_iter(Vec::from_iter(0..=30)),
        }
    }
    /// Returns
    fn insert<S>(&mut self, register: TypedRegisterF<HardwareRegister, S>) -> bool {
        match register {
            TypedRegisterF::Scalar(reg) => self.x.insert(reg),
            TypedRegisterF::Vector(reg, _) => self.v.insert(reg),
        }
    }
}

pub fn interleave(
    lhs: Vec<AtomicInstruction>,
    rhs: Vec<AtomicInstruction>,
) -> Vec<InstructionF<FreshRegister>> {
    lhs.into_iter()
        .zip(rhs)
        .flat_map(|(a, b)| [a, b])
        .flatten()
        .collect()
}

#[derive(Debug)]
pub struct RegisterMapping(Vec<RegisterState>);

impl std::fmt::Display for RegisterMapping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Register Mapping: [")?;
        for (i, state) in self.0.iter().enumerate() {
            match state {
                RegisterState::Unassigned => write!(f, "  {}: U", i)?,
                RegisterState::Assigned(reg) => write!(f, "  {}: M{}", i, reg)?,
                RegisterState::Dropped => write!(f, "  {}: D", i)?,
            }
            write!(f, ", ")?
        }
        write!(f, "]")?;
        Ok(())
    }
}

impl RegisterMapping {
    pub fn new() -> Self {
        // TODO Needs to be equal to the number of free register in the allocator once it is finished
        // but also needs space for the elements in the beginning
        // In the beginning there can't be more than all the vector registers combined, so that can be allocated initially
        // get_or_allocate_register needs to deal with the resizing
        Self(
            std::iter::repeat_with(|| RegisterState::Unassigned)
                .take(30)
                .collect::<Vec<_>>(),
        )
    }

    // Get the physical register for a source register
    fn get_register(
        &self,
        fresh: TypedSizedRegister<FreshRegister>,
    ) -> TypedSizedRegister<HardwareRegister> {
        match *self.index(fresh) {
            RegisterState::Unassigned => unreachable!("{fresh:?} has not been assigned yet"),
            RegisterState::Assigned(reg) => reg,
            RegisterState::Dropped => unreachable!("{fresh:?} already has been dropped"),
        }
    }

    // Get or allocate a register
    fn get_or_allocate_register(
        &mut self,
        register_bank: &mut RegisterBank,
        fresh: TypedSizedRegister<FreshRegister>,
    ) -> TypedSizedRegister<HardwareRegister> {
        // Possible to do a mutable reference here
        let entry = self.index_mut(fresh);
        match *entry {
            RegisterState::Unassigned => {
                let hw_reg = match fresh {
                    TypedRegisterF::Scalar(_) => {
                        let reg = register_bank.x.pop_first().expect("ran out of registers");
                        TypedRegisterF::Scalar(reg)
                    }
                    TypedRegisterF::Vector(_, size) => {
                        let reg = register_bank.v.pop_first().expect("ran out of registers");
                        TypedRegisterF::Vector(reg, size)
                    }
                };

                *entry = RegisterState::Assigned(hw_reg);
                hw_reg
            }
            RegisterState::Assigned(reg) => reg,
            RegisterState::Dropped => unreachable!("{fresh:?} already has been dropped"),
        }
    }

    // Once a fresh register goes out of scope the hardware register that was assigned to that fresh register
    // can be returned to the register bank.
    fn free_register(
        &mut self,
        register_bank: &mut RegisterBank,
        fresh: TypedRegister<FreshRegister>,
    ) -> bool {
        let old = mem::replace(self.index_mut(fresh), RegisterState::Dropped);

        match old {
            RegisterState::Unassigned => {
                unreachable!("There should never be a drop before the register has been assigned")
            }
            RegisterState::Assigned(reg) => {
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

    // Integrate with seen?
    // This output only should output
    pub fn output_register(&self, reg: &impl AliasedRegister) -> String {
        match self.index(reg.to_typed_register()) {
            RegisterState::Unassigned => panic!("requested output register for some"),
            RegisterState::Assigned(hw_reg) => format!("{}", hw_reg),
            RegisterState::Dropped => "Dropped".to_string(),
        }
    }
}

/// We do not implement the Index Trait as that would leak the private RegisterState
impl RegisterMapping {
    fn index<S>(&self, idx: TypedRegisterF<FreshRegister, S>) -> &RegisterState {
        &self.0[*idx.as_fresh() as usize]
    }
    fn index_mut<S>(&mut self, idx: TypedRegisterF<FreshRegister, S>) -> &mut RegisterState {
        &mut self.0[*idx.as_fresh() as usize]
    }
}

pub fn liveness_analysis(
    seen_registers: &mut Seen,
    instructions: Vec<InstructionF<FreshRegister>>,
) -> VecDeque<LivenessCommand> {
    let mut commands = VecDeque::new();
    for instruction in instructions.into_iter().rev() {
        for register in instruction.extract_registers().into_iter().map(drop_size) {
            if seen_registers.insert(register) {
                commands.push_front(LivenessCommand::Drop(register));
            }
        }
        commands.push_front(instruction.into());
    }
    commands
}

pub fn hardware_register_allocation(
    mapping: &mut RegisterMapping,
    register_bank: &mut RegisterBank,
    commands: VecDeque<LivenessCommand>,
) -> Vec<InstructionF<HardwareRegister>> {
    // println!("LivenessCommand: {commands:?}");
    let f = |cmd| {
        // println!();
        // println!("mapping: {mapping}");
        // println!("bank: {register_bank:?}");
        // println!("LivenessCommand: {cmd:?}");
        std::io::stdout().flush().unwrap();
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

pub fn print_instructions<R: std::fmt::Display + Copy>(instrs: &[InstructionF<R>]) {
    instrs
        .iter()
        .for_each(|inst| println!("{}", inst.format_instruction()));
}
