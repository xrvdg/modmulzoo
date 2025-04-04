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

// This instruction models both aliases and regular instructions
// The option on destination can be removed, but that would require
// implementing the aliases such CMP, CMN ourselves.
// This would require introducing
// Destination{
// XZR
// TR(TypeSizedRegister<R>)
// }
// for dest.
// and then write the aliases as instruction as the current design.
// It requires more changes if we want the user to be able to use XZR.
// The best way to do that would likely be a trait and a zero sized type for XZR
#[derive(Debug)]
pub struct InstructionF<R> {
    opcode: String,
    dest: Option<TypedSizedRegister<R>>,
    src: Vec<TypedSizedRegister<R>>,
    modifiers: Mod,
}

// Proper name for this
#[derive(Debug)]
enum Mod {
    None,
    Imm(u64),
    ImmLSL(u16, u8),
    Idx(u8),
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
        let regs: String = self
            .extract_registers()
            .map(|x| x.to_string())
            .intersperse(", ".to_string())
            .collect();

        let extra = match &self.modifiers {
            Mod::None => String::new(),
            Mod::Imm(imm) => format!(", #{imm}"),
            Mod::Cond(cond) => format!(", {cond}"),
            Mod::Idx(idx) => format!("[{idx}]"),
            Mod::ImmLSL(imm, shift) => format!(", #{imm}, lsl {shift}"),
        };
        let inst = &self.opcode;
        format!("{inst} {regs}{extra}")
    }

    fn extract_registers(&self) -> impl Iterator<Item = &TypedSizedRegister<R>> {
        self.dest.iter().chain(&self.src)
    }
}

impl From<InstructionF<FreshRegister>> for LivenessCommand {
    fn from(instr: InstructionF<FreshRegister>) -> Self {
        LivenessCommand::Instr(instr)
    }
}

pub struct Assembler {
    pub instructions: Vec<AtomicInstruction>,
}

impl Assembler {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
        }
    }
    pub fn append_instruction(&mut self, inst: AtomicInstruction) {
        self.instructions.push(inst)
    }
}

use paste::paste;
macro_rules! embed_asm {
    ($name:ident, $opcode:literal, ($($arg:ident : $arg_ty:ty),*) -> $ret_ty:ty) => {
        paste! {
            pub fn $name(alloc: &mut Allocator, asm: &mut Assembler, $($arg: &Reg<$arg_ty>),*) -> Reg<$ret_ty> {
                let ret = alloc.fresh();
                asm.append_instruction(vec![ [<$name _inst>](&ret, $($arg),*) ]);
                ret
            }

            pub fn [<$name _inst>](dest: &Reg<$ret_ty>, $($arg: &Reg<$arg_ty>),*) -> Instruction {
                InstructionF {
                    opcode: $opcode.to_string(),
                    dest: Some(dest.to_typed_register()),
                    src: vec![$($arg.to_typed_register()),*],
                    modifiers: Mod::None,
                }
            }
        }
    };
}

// To quickly write these kind of macros just write the general structure
// with embed_asm and then inline all the macros and modify accordingly

pub fn mov(alloc: &mut Allocator, asm: &mut Assembler, imm: u64) -> Reg<u64> {
    let ret = alloc.fresh();
    asm.append_instruction(vec![mov_inst(&ret, imm)]);
    ret
}

pub fn mov_inst(dest: &Reg<u64>, imm: u64) -> Instruction {
    InstructionF {
        opcode: "mov".to_string(),
        dest: Some(dest.to_typed_register()),
        src: vec![],
        modifiers: Mod::Imm(imm),
    }
}

// operations that use or set a flag should never be used as an atomic.
// interleaving can potentially result in invalid code. Therefore these are only available as inst
pub fn tst_inst(a: &Reg<u64>, imm: u64) -> Instruction {
    InstructionF {
        opcode: "tst".to_string(),
        dest: None,
        src: vec![a.to_typed_register()],
        modifiers: Mod::Imm(imm),
    }
}

pub fn csel_inst(dest: &Reg<u64>, a: &Reg<u64>, b: &Reg<u64>, cond: String) -> Instruction {
    InstructionF {
        opcode: "csel".to_string(),
        dest: Some(dest.to_typed_register()),
        src: vec![a.to_typed_register(), b.to_typed_register()],
        modifiers: Mod::Cond(cond),
    }
}

pub fn cmn_inst(a: &Reg<u64>, b: &Reg<u64>) -> Instruction {
    InstructionF {
        opcode: "cmn".to_string(),
        dest: None,
        src: vec![a.to_typed_register(), b.to_typed_register()],
        modifiers: Mod::None,
    }
}

pub fn cinc_inst(dest: &Reg<u64>, a: &Reg<u64>, cond: String) -> Instruction {
    InstructionF {
        opcode: "cinc".to_string(),
        dest: Some(dest.to_typed_register()),
        src: vec![a.to_typed_register()],
        modifiers: Mod::Cond(cond),
    }
}

// END flag operations

pub fn movk(alloc: &mut Allocator, asm: &mut Assembler, imm: u16, shift: u8) -> Reg<u64> {
    let ret = alloc.fresh();
    asm.append_instruction(vec![movk_inst(&ret, imm, shift)]);
    ret
}

pub fn movk_inst(dest: &Reg<u64>, imm: u16, shift: u8) -> Instruction {
    InstructionF {
        opcode: "movk".to_string(),
        dest: Some(dest.to_typed_register()),
        src: vec![],
        modifiers: Mod::ImmLSL(imm, shift),
    }
}

// Create a new type for b that takes into account the index
pub fn fmla2d(
    _alloc: &mut Allocator, // Done to have the same pattern as the rest
    asm: &mut Assembler,
    add: Reg<Simd<f64, 2>>,
    a: &Reg<Simd<f64, 2>>,
    b: &Reg<Simd<f64, 2>>,
    idx: u8,
) -> Reg<Simd<f64, 2>> {
    asm.append_instruction(vec![fmla2d_inst(&add, a, b, idx)]);
    add
}

pub fn fmla2d_inst(
    dest_add: &Reg<Simd<f64, 2>>,
    a: &Reg<Simd<f64, 2>>,
    b: &Reg<Simd<f64, 2>>,
    idx: u8,
) -> Instruction {
    InstructionF {
        opcode: "fmla.2d".to_string(),
        dest: Some(dest_add.to_typed_register()),
        src: vec![a.to_typed_register(), b.to_typed_register()],
        modifiers: Mod::Idx(idx),
    }
}

embed_asm!(mul, "mul", (a: u64, b: u64) -> u64);
embed_asm!(umulh, "umulh", (a: u64, b: u64) -> u64);

embed_asm!(add, "add", (a: u64, b: u64) -> u64);
// TODO: These operations set flags and should only make their inst available
embed_asm!(adds, "adds", (a: u64, b: u64) -> u64);
embed_asm!(subs, "subs", (a: u64, b: u64) -> u64);
embed_asm!(sbcs, "sbcs", (a: u64, b: u64) -> u64);

// Doesn't support immediates
embed_asm!(mov16b, "mov.16b", (a: Simd<u64,2>) -> Simd<u64,2>);
embed_asm!(ucvtf2d, "uvctf.2d", (a: Simd<u64,2>) -> Simd<f64,2>);
embed_asm!(dup2d, "dup.2d", (a: u64) -> Simd<u64,2>);
embed_asm!(ucvtf, "uvctf", (a: u64) -> f64);

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
#[derive(Debug, Eq, PartialOrd, Ord, Hash, PartialEq, Clone, Copy)]
pub enum Addressing {
    // Unsigned
    X,
    // SIMD/FP
    V,
    D,
}

/// TODO new name under this construction
#[derive(Debug, PartialOrd, Ord, Eq, Hash, PartialEq, Clone, Copy)]
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

impl Reg<f64> {
    pub fn as_simd(&self) -> &Reg<Simd<f64, 2>> {
        unsafe { std::mem::transmute(self) }
    }
}

impl<S> Reg<Simd<S, 2>> {
    pub fn into_<D>(self) -> Reg<Simd<D, 2>> {
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
#[derive(PartialEq, Debug, Hash, Ord, PartialOrd, Eq, Clone, Copy)]
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

impl<T> RegisterSource for Simd<T, 2> {
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

pub fn interleave(lhs: Vec<AtomicInstruction>, rhs: Vec<AtomicInstruction>) -> Vec<Instruction> {
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
                .take(200)
                .collect::<Vec<_>>(),
        )
    }

    pub fn allocated(&self) -> usize {
        self.0
            .iter()
            .filter(|&reg_state| matches!(reg_state, RegisterState::Assigned(_)))
            .count()
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
    pub fn output_register<T: RegisterSource>(
        &self,
        reg: &Reg<T>,
    ) -> Option<TypedSizedRegister<HardwareRegister>> {
        // Todo this could go from Reg to index instead of to_type_registers
        match self.index(reg.reg) {
            RegisterState::Unassigned => None,
            RegisterState::Assigned(hw_reg) => Some(*hw_reg),
            RegisterState::Dropped => None,
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
            .map(|tr| *tr.as_fresh())
            .collect();
        // The difference could be mutable
        let release: HashSet<_> = registers.difference(&seen_registers.0).copied().collect();

        if let Some(dest) = instruction.dest {
            if release.contains(dest.as_fresh()) {
                // Better way to give feedback? Now the user doesn't know where it comes from
                // We view an unused instruction as a problem
                print_instructions(&instructions);
                panic!("{instruction:?} does not use the destination")
            }; // The union could be mutable
        }

        release.iter().for_each(|reg| {
            seen_registers.0.insert(*reg);
        });
        commands.push_front(release);
    }
    commands
}

pub fn hardware_register_allocation(
    mapping: &mut RegisterMapping,
    register_bank: &mut RegisterBank,
    instructions: Vec<Instruction>,
    // Change this into a Seen, and then rename Seen?
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
        let dest = instruction
            .dest
            .map(|d| mapping.get_or_allocate_register(register_bank, d));
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

pub fn backend_global(label: &str, instructions: &Vec<InstructionF<HardwareRegister>>) -> String {
    let mut asm_code = String::new();
    let label = format!("_{label}");
    asm_code.push_str(&format!(".global {label}\n.align 4\n.text\n"));
    asm_code.push_str(&format!("{label}:\n"));
    asm_code.extend(
        instructions
            .into_iter()
            .map(|instruction| format!("  {}\n", instruction.format_instruction())),
    );
    asm_code.push_str("ret\n");
    asm_code
}

pub fn backend_inline(instructions: &Vec<InstructionF<HardwareRegister>>) -> String {
    let mut asm_code = String::new();
    asm_code.extend(
        instructions
            .into_iter()
            .map(|instruction| format!("\"{}\",\n", instruction.format_instruction())),
    );
    asm_code
}

/// Outputs all the arguments to the asm! macros
/// TODO provide labels for inputs

pub fn backend_rust(
    mapping: RegisterMapping,
    input_registers: &[TypedSizedRegister<HardwareRegister>],
    output_registers: &[TypedSizedRegister<HardwareRegister>],
    instructions: &Vec<InstructionF<HardwareRegister>>,
) -> String {
    assert_eq!(mapping.allocated(), output_registers.len());

    let inputs = input_registers
        .iter()
        .map(|r| format!("in(\"{}\") _", r))
        .intersperse(", ".to_string());

    let outputs = output_registers
        .iter()
        .enumerate()
        .map(|(i, r)| format!("lateout(\"{}\") out[{}]", r, i))
        .intersperse(", ".to_string());

    let mut clobber_registers: BTreeSet<&TypedSizedRegister<HardwareRegister>> = BTreeSet::new();
    instructions.iter().for_each(|instruction| {
        clobber_registers.extend(instruction.extract_registers());
    });

    let output_registers = BTreeSet::from_iter(output_registers.iter());

    let clobbers = clobber_registers
        .difference(&output_registers)
        .map(|r| format!("lateout(\"{}\") _", r))
        .intersperse(", ".to_string());

    let newline = std::iter::once(",\n".to_string());
    // We jump to the assembly code with br so we need to safe the lr register
    // This can change in the future
    let lr = std::iter::once("lateout(\"lr\") _".to_string());

    inputs
        .chain(newline.clone())
        .chain(outputs)
        .chain(newline.clone())
        .chain(clobbers)
        .chain(newline.clone())
        .chain(lr)
        .collect()
}
