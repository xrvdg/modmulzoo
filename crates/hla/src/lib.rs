#![feature(iter_intersperse)]
use std::{
    collections::{BTreeSet, HashSet, VecDeque},
    marker::PhantomData,
    mem::{self},
};

// See if these can be reduced. Took all of these as it was a u64 before

impl TypedSizedRegister<FreshRegister> {
    // Should only be seen by RegisterMapping
    // Could have been used for HardwareRegister, but it's better to convert types
    // Only for 'internal' use
    fn as_fresh(&self) -> &FreshRegister {
        &self.reg
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
impl<R: std::fmt::Display> std::fmt::Display for TypedSizedRegister<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reg = &self.reg;
        let addr = self.addressing;
        write!(f, "{addr}{reg}")
    }
}

impl std::fmt::Display for Addressing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Addressing::V => write!(f, "v"),
            Addressing::D => write!(f, "d"),
            Addressing::X => write!(f, "x"),
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
        pub fn $name(dst: &Reg<u64>, a: &Reg<u64>, b: &Reg<u64>) -> crate::AtomicInstruction {
            vec![crate::Instruction {
                opcode: stringify!($name).to_string(),
                dest: dst.to_typed_register(),
                src: vec![a.to_typed_register(), b.to_typed_register()],
                modifiers: Mod::None,
            }]
        }
    };

    ($name:ident, $opcode:literal, 3) => {
        pub fn $name(
            dst: &Reg<Simd<u64, 2>>,
            src_a: &Reg<Simd<u64, 2>>,
            src_b: &Reg<Simd<u64, 2>>,
            i: u8,
        ) -> crate::AtomicInstruction {
            vec![crate::Instruction {
                opcode: $opcode.to_string(),
                dest: dst.to_typed_register(),
                src: vec![src_a.to_typed_register(), src_b.to_typed_register()],
                modifiers: Mod::Idx(i as u64),
            }]
        }
    };

    ($name:ident, $opcode:literal, 2) => {
        pub fn $name(dst: &Reg<Simd<u64, 2>>, src: &Reg<Simd<u64, 2>>) -> crate::AtomicInstruction {
            vec![crate::Instruction {
                opcode: $opcode.to_string(),
                dest: dst.to_typed_register(),
                src: vec![src.to_typed_register()],
                modifiers: Mod::None,
            }]
        }
    };

    ($name:ident, $opcode:literal, 2, m) => {
        pub fn $name(dst: &Reg<Simd<u64, 2>>, src: &Reg<u64>) -> crate::AtomicInstruction {
            vec![crate::Instruction {
                opcode: $opcode.to_string(),
                dest: dst.to_typed_register(),
                src: vec![src.to_typed_register()],
                modifiers: Mod::None,
            }]
        }
    };

    ($name:ident, 2, m) => {
        pub fn $name<T: Reg64Bit + RegisterSource>(
            dst: &Reg<f64>,
            src: &Reg<T>,
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
        pub fn $name(dst: &Reg<u64>, val: u64) -> crate::AtomicInstruction {
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
        pub fn $name(dst: &Reg<u64>, src: &Reg<u64>, condition: &str) -> crate::AtomicInstruction {
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

pub struct Reg<T> {
    reg: FreshRegister,
    _marker: PhantomData<T>,
}

/// Define the struct ourself as to not have to import it
pub struct Simd<T, const N: usize>(PhantomData<T>);

pub trait Reg64Bit {}
impl Reg64Bit for u64 {}
impl Reg64Bit for f64 {}

/// `Reg` represents the fresh variable and has (as much as possible) the same semantics as a regular rust variable.
/// FreshRegister represent the label for the fresh variable.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct FreshRegister(u64);

impl std::fmt::Display for FreshRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u64> for FreshRegister {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

/// Vector sizes to erase the difference between address float64 or u64
/// TODO different name for addressing
#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub enum Addressing {
    // Unsigned
    X,
    // SIMD/FP
    V,
    D,
}

/// TODO new name under this construction
#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub struct TypedSizedRegister<R> {
    reg: R,
    addressing: Addressing,
}

/// The result of the liveness analysis and it gives commands to the
/// hardware register allocator
#[derive(Debug)]
pub enum LivenessCommand {
    Instr(InstructionF<FreshRegister>),
    Drop(FreshRegister),
}

impl<T> Reg<T> {
    fn new(reg: u64) -> Self {
        Self {
            reg: reg.into(),
            _marker: Default::default(),
        }
    }

    // (temporary?) indirection to bring the typing under the type itself
    fn to_typed_register(&self) -> TypedSizedRegister<FreshRegister>
    where
        T: RegisterSource,
    {
        T::to_typed_register(self.reg)
    }
}

impl Reg<Simd<u64, 2>> {
    pub fn as_f64(&self) -> &Reg<f64> {
        unsafe { std::mem::transmute(self) }
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
    pub fn fresh<T>(&mut self) -> Reg<T> {
        let x = self.fresh;
        self.fresh += 1;
        Reg::new(x)
    }

    pub fn new() -> Self {
        Self { fresh: 0 }
    }
}

impl std::fmt::Display for Reg<u64> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x{}", self.reg)
    }
}

impl std::fmt::Debug for Reg<u64> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x{}", self.reg)
    }
}

// Add another struct to prevent things from being created
// Make a struct around here such that it can't be copied
// THe phys_register file is the one that creates them
#[derive(PartialEq, Debug, Ord, PartialOrd, Eq, Clone, Copy)]
pub struct HardwareRegister(u64);

impl std::fmt::Display for HardwareRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
// No Clone as the state of one free reg
// does not make sense as the state of another free reg
#[derive(PartialEq, Debug)]
enum RegisterState {
    Unassigned,
    Assigned(TypedSizedRegister<HardwareRegister>),
    Dropped,
}

type RegisterPool = BTreeSet<HardwareRegister>;

// TODO different name than RegisterSource
pub trait RegisterSource {
    fn get_register_pool(pools: &mut RegisterBank) -> &mut RegisterPool;
    fn to_typed_register<R>(reg: R) -> TypedSizedRegister<R>;
}

impl RegisterSource for u64 {
    fn get_register_pool(pools: &mut RegisterBank) -> &mut RegisterPool {
        &mut pools.x
    }

    fn to_typed_register<R>(reg: R) -> TypedSizedRegister<R> {
        TypedSizedRegister {
            reg,
            addressing: Addressing::X,
        }
    }
}

impl RegisterSource for f64 {
    fn get_register_pool(pools: &mut RegisterBank) -> &mut RegisterPool {
        &mut pools.v
    }

    fn to_typed_register<R>(reg: R) -> TypedSizedRegister<R> {
        TypedSizedRegister {
            reg,
            addressing: Addressing::D,
        }
    }
}

impl RegisterSource for Simd<u64, 2> {
    fn get_register_pool(pools: &mut RegisterBank) -> &mut RegisterPool {
        &mut pools.v
    }

    fn to_typed_register<R>(reg: R) -> TypedSizedRegister<R> {
        TypedSizedRegister {
            reg,
            addressing: Addressing::V,
        }
    }
}

pub fn input<T>(
    asm: &mut Allocator,
    mapping: &mut RegisterMapping,
    phys_registers: &mut RegisterBank,
    phys: u64,
) -> Reg<T>
where
    T: RegisterSource,
{
    let fresh = asm.fresh();

    let hw_reg = HardwareRegister(phys);

    let pool = T::get_register_pool(phys_registers);
    if !pool.remove(&hw_reg) {
        panic!("{:?} is already in use", phys)
    }

    *mapping.index_mut(fresh.reg) = RegisterState::Assigned(T::to_typed_register(hw_reg));

    fresh
}

pub struct Seen(HashSet<FreshRegister>);

impl Seen {
    pub fn new() -> Self {
        Self(HashSet::new())
    }

    pub fn output_interface<T: RegisterSource>(&mut self, fresh: &Reg<T>) -> bool {
        self.seen(fresh.reg)
    }

    fn seen(&mut self, fresh: FreshRegister) -> bool {
        self.0.insert(fresh)
    }
}

#[derive(Debug)]
pub struct RegisterBank {
    x: RegisterPool,
    v: RegisterPool,
}

impl RegisterBank {
    pub fn new() -> Self {
        Self {
            x: BTreeSet::from_iter((0..=30).map(HardwareRegister)),
            v: BTreeSet::from_iter((0..=30).map(HardwareRegister)),
        }
    }

    fn get_register_pool(&mut self, addr: Addressing) -> &mut RegisterPool {
        match addr {
            Addressing::X => &mut self.x,
            Addressing::V | Addressing::D => &mut self.v,
        }
    }

    /// Return the hardware register back into the register pool
    fn insert(&mut self, register: TypedSizedRegister<HardwareRegister>) -> bool {
        self.get_register_pool(register.addressing)
            .insert(register.reg)
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
        match *self.index(*fresh.as_fresh()) {
            RegisterState::Unassigned => unreachable!("{fresh:?} has not been assigned yet"),
            RegisterState::Assigned(reg) => reg,
            RegisterState::Dropped => unreachable!("{fresh:?} already has been dropped"),
        }
    }

    // Get or allocate a register
    fn get_or_allocate_register(
        &mut self,
        register_bank: &mut RegisterBank,
        typed_register: TypedSizedRegister<FreshRegister>,
    ) -> TypedSizedRegister<HardwareRegister> {
        // Possible to do a mutable reference here
        let entry = self.index_mut(*typed_register.as_fresh());
        match *entry {
            RegisterState::Unassigned => {
                let addr = typed_register.addressing;
                let pool = register_bank.get_register_pool(addr);

                let hw_reg = pool.pop_first().expect("ran out of registers");

                let typed_hw_reg = TypedSizedRegister {
                    reg: hw_reg,
                    addressing: addr,
                };

                *entry = RegisterState::Assigned(typed_hw_reg);
                typed_hw_reg
            }
            RegisterState::Assigned(reg) => reg,
            RegisterState::Dropped => unreachable!("{typed_register:?} already has been dropped"),
        }
    }

    // Once a fresh register goes out of scope the hardware register that was assigned to that fresh register
    // can be returned to the register bank.
    fn free_register(&mut self, register_bank: &mut RegisterBank, fresh: FreshRegister) -> bool {
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
    pub fn output_register<T: RegisterSource>(&self, reg: &Reg<T>) -> String {
        // Todo this could go from Reg to index instead of to_type_registers
        match self.index(reg.reg) {
            RegisterState::Unassigned => panic!("requested output register for some"),
            RegisterState::Assigned(hw_reg) => format!("{}", hw_reg),
            RegisterState::Dropped => "Dropped".to_string(),
        }
    }
}

/// We do not implement the Index Trait as that would leak the private RegisterState
impl RegisterMapping {
    fn index(&self, idx: FreshRegister) -> &RegisterState {
        &self.0[idx.0 as usize]
    }
    fn index_mut(&mut self, idx: FreshRegister) -> &mut RegisterState {
        &mut self.0[idx.0 as usize]
    }
}

// TODO optimise
// The invariant is that the hashset will only contain the sources and therefore always free to deallocate
// because if not it means that it's either been used earlier so it would not show up in release.
// The other way it shows up if the source
pub fn liveness_analysis(
    seen_registers: &mut Seen,
    instructions: &[Instruction],
) -> VecDeque<HashSet<FreshRegister>> {
    let mut commands = VecDeque::new();
    for instruction in instructions.iter().rev() {
        // Add check whether the source is released here.
        // If we don't want to check for that later it is required that the instruction is filtered out here
        // otherwise we need a special structure that checks for both
        let registers: HashSet<_> = instruction
            .extract_registers()
            .into_iter()
            .map(|tr| *tr.as_fresh())
            .collect();
        // The difference could be mutable
        let release: HashSet<_> = registers.difference(&seen_registers.0).cloned().collect();
        if release.contains(instruction.dest.as_fresh()) {
            // Better way to give feedback? Now the user doesn't know where it comes from
            // We view an unused instruction as a problem
            panic!("{instruction:?} does not use the destination")
        }; // The union could be mutable
        seen_registers.0 = seen_registers.0.union(&registers).cloned().collect();
        commands.push_front(release);
    }
    commands
}

pub fn hardware_register_allocation(
    mapping: &mut RegisterMapping,
    register_bank: &mut RegisterBank,
    instructions: Vec<Instruction>,
    // Change this into a Seen?
    releases: VecDeque<HashSet<FreshRegister>>,
) -> Vec<InstructionF<HardwareRegister>> {
    assert_eq!(
        instructions.len(),
        releases.len(),
        "The instructions and release collections need to be the same lenght"
    );

    let f = |(instruction, release): (Instruction, HashSet<_>)| {
        // println!();
        // println!("mapping: {mapping}");
        // println!("bank: {register_bank:?}");
        // println!("instruction: {instruction:?}");
        // println!("release: {release:?}");
        // std::io::stdout().flush().unwrap();

        let src = instruction
            .src
            .into_iter()
            .map(|s| mapping.get_register(s))
            .collect();
        // assert on the return of free register?
        release.into_iter().for_each(|fresh| {
            mapping.free_register(register_bank, fresh);
        });
        let dest = mapping.get_or_allocate_register(register_bank, instruction.dest);
        InstructionF {
            opcode: instruction.opcode,
            dest,
            src,
            modifiers: instruction.modifiers,
        }
    };

    instructions.into_iter().zip(releases).map(f).collect()
}

pub fn print_instructions<R: std::fmt::Display + Copy>(instrs: &[InstructionF<R>]) {
    instrs
        .iter()
        .for_each(|inst| println!("{}", inst.format_instruction()));
}
