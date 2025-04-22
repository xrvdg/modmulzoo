#![feature(iter_intersperse)]
use std::{
    collections::{BTreeSet, HashSet, VecDeque},
    mem::{self},
};

pub mod frontend;
pub mod instructions;

pub use frontend::*;

pub type AtomicInstruction = Vec<InstructionF<FreshRegister>>;
pub type Instruction = InstructionF<FreshRegister>;

/// This instruction models both aliases and regular instructions
#[derive(Debug, PartialEq)]
pub struct InstructionF<R> {
    opcode: String,
    // Reasons for destination being a vector
    // - Some operations have do not write results to a register
    //   - CMN only affects flags
    //   - STR writes to a destination stored in an operand
    // - LDP has 2 destinations
    results: Vec<TypedSizedRegister<R>>,
    operands: Vec<TypedSizedRegister<R>>,
    modifiers: Mod,
}

#[derive(Debug, PartialEq)]
enum Mod {
    None,
    Imm(u64),
    ImmLSL(u16, u8),
    // Logical shift left
    LSL(u8),
    Cond(String),
}

impl<R: std::fmt::Display + Copy> std::fmt::Display for InstructionF<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let regs: String = self
            .extract_registers()
            .map(|x| x.to_string())
            .intersperse(", ".to_string())
            .collect();

        let extra = match &self.modifiers {
            Mod::None => String::new(),
            Mod::Imm(imm) => format!(", #{imm}"),
            Mod::Cond(cond) => format!(", {cond}"),
            Mod::ImmLSL(imm, shift) => format!(", #{imm}, lsl {shift}"),
            Mod::LSL(imm) => format!(", #{imm}"),
        };

        let inst = &self.opcode;
        write!(f, "{inst} {regs}{extra}")
    }
}

impl<R> InstructionF<R> {
    fn extract_registers(&self) -> impl Iterator<Item = &TypedSizedRegister<R>> {
        self.results.iter().chain(&self.operands)
    }
}

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

impl Addressing {
    fn to_pool(&self, reg: HardwareRegister) -> Pool {
        match self {
            Addressing::X => Pool::General(reg),
            Addressing::V | Addressing::D => Pool::Vector(reg),
        }
    }
}
#[derive(Debug, PartialOrd, Ord, Eq, Hash, PartialEq, Clone, Copy)]
pub struct TypedSizedRegister<R> {
    reg: R,
    addressing: Addressing,
    idx: Index,
}

impl<R: std::fmt::Display> std::fmt::Display for TypedSizedRegister<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reg = &self.reg;
        let addr = self.addressing;
        match self.idx {
            Index::None => write!(f, "{addr}{reg}"),
            Index::Lane(idx) => write!(f, "{addr}{reg}[{idx}]"),
            Index::LaneSized(lane_sizes, idx) => write!(f, "{addr}{reg}.{lane_sizes}[{idx}]"),
            Index::Pointer(offset) => write!(f, "[{addr}{reg}, #{offset}]"),
        }
    }
}

impl std::fmt::Display for LaneSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LaneSize::S => write!(f, "s"),
            LaneSize::D => write!(f, "d"),
        }
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

#[derive(Debug, PartialOrd, Ord, Eq, Hash, PartialEq, Clone, Copy)]
#[repr(u8)]
enum LaneSize {
    S = 4,
    D = 2,
}

#[derive(Debug, PartialOrd, Ord, Eq, Hash, PartialEq, Clone, Copy)]
enum Index {
    None,
    // Some instructions require the size (.d, .s, etc) in combination with
    // a lane and others do not
    Lane(u8),
    LaneSized(LaneSize, u8),
    // offset in bytes
    Pointer(usize),
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

#[derive(Debug)]
pub struct Allocator {
    // It's about unique counters so we use the counter for both
    // q and v registers
    // this makes it easier to read the assembly
    pub fresh: u64,
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
    Assigned(Pool),
    Dropped,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Pool {
    General(HardwareRegister),
    Vector(HardwareRegister),
}

impl Pool {
    fn reg(&self) -> HardwareRegister {
        match self {
            Pool::General(reg) => *reg,
            Pool::Vector(reg) => *reg,
        }
    }
}

#[derive(Debug)]
struct RegisterPool {
    pool: BTreeSet<HardwareRegister>,
    availability: Vec<Option<(FreshRegister, usize)>>,
}

impl RegisterPool {
    fn new<T>(registers: T) -> Self
    where
        T: Iterator<Item = u64> + Clone,
    {
        let len = registers
            .clone()
            .max()
            .expect("Can't have a zero sized register pool");

        RegisterPool {
            pool: BTreeSet::from_iter(registers.map(HardwareRegister)),
            // Registers start from 0 so highest count + 1
            availability: vec![None; (len + 1) as usize],
        }
    }

    fn pop_first(&mut self, reg: FreshRegister, end_lifetime: usize) -> Option<HardwareRegister> {
        // Find the first register that satisfies the condition
        let reg = self
            .pool
            .iter()
            .find(
                // Check if the hardware register has been preassigned assigned to this fresh registers
                // Check if the hardware register can be used before it's preassigned moment
                //
                |&hardware_register| match self.availability[hardware_register.0 as usize] {
                    Some((tp, _lifetime)) if reg == tp => true,
                    Some((_tp, lifetime)) if end_lifetime <= lifetime => true,
                    // Hardware register has not been preassigned
                    None => true,
                    // Hardware register was preassigned to a different fresh register and it's ownership overlaps
                    // with the lifetime of reg
                    _ => false,
                },
            )
            .copied();

        // Remove the register from the pool if found
        if let Some(hardware_register) = reg {
            self.pool.remove(&hardware_register);
        }

        reg
    }

    fn set_availability(&mut self, tp: FreshRegister, register: HardwareRegister, lifetime: usize) {
        let av = &mut self.availability[register.0 as usize];

        match av {
            Some(_) => panic!("Availability of hardware register {register} already set"),
            None => *av = Some((tp, lifetime)),
        }
    }

    fn insert(&mut self, register: HardwareRegister) -> bool {
        self.pool.insert(register)
    }

    fn remove(&mut self, register: &HardwareRegister) -> bool {
        self.pool.remove(register)
    }
}

// TODO different name than RegisterSource
pub trait RegisterSource {
    fn to_typed_register(&self) -> TypedSizedRegister<FreshRegister>;
}

impl<T> RegisterSource for Reg<*mut T> {
    fn to_typed_register(&self) -> TypedSizedRegister<FreshRegister> {
        TypedSizedRegister {
            reg: self.reg,
            addressing: Addressing::X,
            idx: Index::Pointer(0),
        }
    }
}

impl<T> RegisterSource for PointerReg<'_, T> {
    fn to_typed_register(&self) -> TypedSizedRegister<FreshRegister> {
        TypedSizedRegister {
            reg: self.reg.reg,
            addressing: Addressing::X,
            idx: Index::Pointer(self.offset as usize),
        }
    }
}

impl RegisterSource for Reg<u64> {
    fn to_typed_register(&self) -> TypedSizedRegister<FreshRegister> {
        TypedSizedRegister {
            reg: self.reg,
            addressing: Addressing::X,
            idx: Index::None,
        }
    }
}

impl RegisterSource for Reg<f64> {
    fn to_typed_register(&self) -> TypedSizedRegister<FreshRegister> {
        TypedSizedRegister {
            reg: self.reg,
            addressing: Addressing::D,
            idx: Index::None,
        }
    }
}

impl<T> RegisterSource for Reg<Simd<T, 2>> {
    fn to_typed_register(&self) -> TypedSizedRegister<FreshRegister> {
        TypedSizedRegister {
            reg: self.reg,
            addressing: Addressing::V,
            idx: Index::None,
        }
    }
}

impl<T, const I: u8> RegisterSource for Idx<Reg<Simd<T, 2>>, I> {
    fn to_typed_register(&self) -> TypedSizedRegister<FreshRegister> {
        let mut tp = self.0.to_typed_register();
        tp.idx = Index::Lane(I);
        tp
    }
}

impl<T, const L: u8, const I: u8> RegisterSource for Sized<Idx<Reg<Simd<T, 2>>, I>, L> {
    fn to_typed_register(&self) -> TypedSizedRegister<FreshRegister> {
        let mut tp = self.0.to_typed_register();

        let sizes = match L {
            2 => LaneSize::D,
            4 => LaneSize::S,
            _ => panic!("invalid lane size"),
        };

        tp.idx = Index::LaneSized(sizes, I);
        tp
    }
}

pub fn input<T>(
    asm: &mut Allocator,
    mapping: &mut RegisterMapping,
    register_bank: &mut RegisterBank,
    phys: u64,
) -> Reg<T>
where
    Reg<T>: RegisterSource,
{
    let fresh = asm.fresh();

    let hw_reg = HardwareRegister(phys);
    let tp = fresh.to_typed_register();

    if !register_bank.remove(hw_reg, tp.addressing) {
        panic!("{:?} is already in use", phys)
    }

    *mapping.index_mut(fresh.reg) = RegisterState::Assigned(tp.addressing.to_pool(hw_reg));

    fresh
}

pub fn pin_register<T: RegisterSource>(
    register_bank: &mut RegisterBank,
    lifetimes: &Vec<(usize, usize)>,
    fresh: &T,
    hardware_register: u64,
) where
    T: RegisterSource,
{
    let hardware_register = HardwareRegister(hardware_register);
    let tp = fresh.to_typed_register();

    register_bank.set_availability(hardware_register, tp, lifetimes[tp.reg.0 as usize].0);
}

pub struct Seen(HashSet<FreshRegister>);

impl Seen {
    pub fn new() -> Self {
        Self(HashSet::new())
    }

    pub fn output_interface<T: RegisterSource>(&mut self, fresh: &T) -> bool {
        self.seen(fresh.to_typed_register().reg)
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
            // Exclude registers:
            // - 18 Reserved by OS
            // - 19 Reserved by LLVM
            x: RegisterPool::new((0..=17).chain(20..29)),
            v: RegisterPool::new(0..=30),
        }
    }

    fn get_register_pool(&mut self, addr: Addressing) -> &mut RegisterPool {
        match addr {
            Addressing::X => &mut self.x,
            Addressing::V | Addressing::D => &mut self.v,
        }
    }

    fn pop_first(
        &mut self,
        tp: TypedSizedRegister<FreshRegister>,
        end_lifetime: usize,
    ) -> Option<HardwareRegister> {
        self.get_register_pool(tp.addressing)
            .pop_first(tp.reg, end_lifetime)
    }

    fn remove(&mut self, register: HardwareRegister, addr: Addressing) -> bool {
        self.get_register_pool(addr).remove(&register)
    }

    fn set_availability(
        &mut self,
        register: HardwareRegister,
        tp: TypedSizedRegister<FreshRegister>,
        lifetime: usize,
    ) {
        self.get_register_pool(tp.addressing)
            .set_availability(tp.reg, register, lifetime);
    }

    /// Return the hardware register back into the register pool
    fn insert(&mut self, register: Pool) -> bool {
        match register {
            Pool::General(hardware_register) => self.x.insert(hardware_register),
            Pool::Vector(hardware_register) => self.v.insert(hardware_register),
        }
    }
}

pub fn interleave<T>(lhs: Vec<T>, rhs: Vec<T>) -> Vec<T> {
    let (shorter, longer) = if lhs.len() <= rhs.len() {
        (lhs, rhs)
    } else {
        (rhs, lhs)
    };

    if shorter.is_empty() {
        return longer;
    }

    let mut result = Vec::with_capacity(shorter.len() + longer.len());

    let short_len = shorter.len();
    let mut short_iter = shorter.into_iter().enumerate();

    let long_len = longer.len();
    let mut long_iter = longer.into_iter();
    // For the first element (short_index = 0 ) -> The location will be ((short_index + 1) * long_len) / short_len
    let mut next = long_len / short_len;

    // With spacing i needs to reach and place the last element of short
    // ((short_len - 1 + 1) * long_len) / short_len = long_len. Therefore the range is 0..=long_len
    for i in 0..=long_len {
        if i == next {
            if let Some((short_index, item)) = short_iter.next() {
                result.push(item);
                // Order is important due to flooring
                // next = index next element (short_index + 1) + 1
                next = ((short_index + 2) * long_len) / short_len;
            }
        }

        if let Some(item) = long_iter.next() {
            result.push(item)
        }
    }

    assert!(short_iter.next().is_none());

    result
}
// Write test that checks if the combined length is always the same as from the individual lengths

#[derive(Debug)]
pub struct RegisterMapping(Vec<RegisterState>);

impl std::fmt::Display for RegisterMapping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Register Mapping: [")?;
        for (i, state) in self.0.iter().enumerate() {
            match state {
                RegisterState::Unassigned => write!(f, "  {}: U", i)?,
                RegisterState::Assigned(reg) => write!(f, "  {}: M{:?}", i, reg)?,
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
                .take(2000)
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
        match *self.index(fresh.reg) {
            RegisterState::Unassigned => unreachable!("{fresh:?} has not been assigned yet"),

            RegisterState::Assigned(reg) => TypedSizedRegister {
                reg: reg.reg(),
                addressing: fresh.addressing,
                idx: fresh.idx,
            },
            RegisterState::Dropped => unreachable!("{fresh:?} already has been dropped"),
        }
    }

    // Get or allocate a register
    fn get_or_allocate_register(
        &mut self,
        register_bank: &mut RegisterBank,
        typed_register: TypedSizedRegister<FreshRegister>,
        end_lifetime: usize,
    ) -> TypedSizedRegister<HardwareRegister> {
        // Possible to do a mutable reference here
        let entry = self.index_mut(typed_register.reg);
        let hw_reg = match *entry {
            RegisterState::Unassigned => {
                let addr = typed_register.addressing;
                let hw_reg = register_bank
                    .pop_first(typed_register, end_lifetime)
                    .expect("ran out of registers");

                *entry = RegisterState::Assigned(addr.to_pool(hw_reg));
                hw_reg
            }

            RegisterState::Assigned(reg) => reg.reg(),
            RegisterState::Dropped => unreachable!("{typed_register:?} already has been dropped"),
        };
        TypedSizedRegister {
            reg: hw_reg,
            addressing: typed_register.addressing,
            idx: typed_register.idx,
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
                    "hardware:{reg:?} is assigned to more than one fresh register. "
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
    // Two reasons we convert it to to_typed_register
    // - have access to reg without having to introduce a reg on the trait
    // - to only have the addressing defined in a single place namely RegisterSource
    // Whether we should keep it as Typed Sized Register is another question
    // The index is not of interested here and needs to be set to None explicitly
    pub fn output_register<R: RegisterSource>(
        &self,
        reg: &R,
    ) -> Option<TypedSizedRegister<HardwareRegister>> {
        let tp = reg.to_typed_register();
        match self.index(tp.reg) {
            RegisterState::Unassigned => None,
            RegisterState::Assigned(hw_reg) => Some(TypedSizedRegister {
                reg: hw_reg.reg(),
                addressing: tp.addressing,
                idx: Index::None,
            }),
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
    nr_fresh_registers: usize,
) -> (VecDeque<HashSet<FreshRegister>>, Vec<(usize, usize)>) {
    // Keep track of the last line the free register is used for
    let mut lifetimes = vec![(0, usize::MAX); nr_fresh_registers];
    let mut commands = VecDeque::new();
    for (line, instruction) in instructions.iter().enumerate().rev() {
        // Add check whether the source is released here.
        // If we don't want to check for that later it is required that the instruction is filtered out here
        // otherwise we need a special structure that checks for both
        let registers: HashSet<_> = instruction.extract_registers().map(|tr| tr.reg).collect();

        // The difference could be mutable
        let release: HashSet<_> = registers.difference(&seen_registers.0).copied().collect();

        instruction.results.iter().for_each(|dest| {
            let dest = dest.reg;

            if release.contains(&dest) {
                // Better way to give feedback? Now the user doesn't know where it comes from
                // We view an unused instruction as a problem
                print_instructions(&instructions);
                panic!("{line}: {instruction:?} does not use the destination")
            }; // The union could be mutable

            let (_b, e) = lifetimes[dest.0 as usize];
            lifetimes[dest.0 as usize] = (line, e);
        });

        release.iter().for_each(|reg| {
            let (b, _e) = lifetimes[reg.0 as usize];
            lifetimes[reg.0 as usize] = (b, line);
            seen_registers.0.insert(*reg);
        });
        commands.push_front(release);
    }
    (commands, lifetimes)
}

pub fn hardware_register_allocation(
    mapping: &mut RegisterMapping,
    register_bank: &mut RegisterBank,
    instructions: Vec<Instruction>,
    // Change this into a Seen, and then rename Seen?
    releases: VecDeque<HashSet<FreshRegister>>,
    lifetimes: Vec<(usize, usize)>,
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
            .operands
            .into_iter()
            .map(|s| mapping.get_register(s))
            .collect();
        // assert on the return of free register?
        release.into_iter().for_each(|fresh| {
            mapping.free_register(register_bank, fresh);
        });

        let dest = instruction
            .results
            .into_iter()
            .map(|d| {
                let idx = d.reg.0;
                mapping.get_or_allocate_register(register_bank, d, lifetimes[idx as usize].1)
            })
            .collect();

        InstructionF {
            opcode: instruction.opcode,
            results: dest,
            operands: src,
            modifiers: instruction.modifiers,
        }
    };

    instructions.into_iter().zip(releases).map(f).collect()
}

pub fn print_instructions<R: std::fmt::Display + Copy>(instrs: &[InstructionF<R>]) {
    instrs
        .iter()
        .enumerate()
        .for_each(|(line, inst)| println!("{line}: {}", inst));
}

pub fn backend_global(label: &str, instructions: &Vec<InstructionF<HardwareRegister>>) -> String {
    let mut asm_code = String::new();
    let label = format!("_{label}");
    asm_code.push_str(&format!(".global {label}\n.align 4\n.text\n"));
    asm_code.push_str(&format!("{label}:\n"));
    asm_code.extend(
        instructions
            .into_iter()
            .map(|instruction| format!("  {}\n", instruction)),
    );
    asm_code.push_str("ret\n");
    asm_code
}

pub fn backend_inline(instructions: &Vec<InstructionF<HardwareRegister>>) -> String {
    let mut asm_code = String::new();
    asm_code.extend(
        instructions
            .into_iter()
            .map(|instruction| format!("\"{}\",\n", instruction)),
    );
    asm_code
}

/// Outputs all the arguments to the asm! macros
/// TODO provide labels for inputs

pub fn backend_rust(
    mapping: RegisterMapping,
    inputs_registers: Vec<Vec<TypedSizedRegister<HardwareRegister>>>,
    outputs_registers: Vec<Vec<TypedSizedRegister<HardwareRegister>>>,
    instructions: &Vec<InstructionF<HardwareRegister>>,
) -> String {
    assert_eq!(
        mapping.allocated(),
        outputs_registers
            .iter()
            .map(|output_register| output_register.len())
            .sum()
    );

    let inputs: Vec<_> = inputs_registers
        .iter()
        .enumerate()
        .flat_map(|(n, input_register)| {
            input_register
                .iter()
                .enumerate()
                .map(move |(i, r)| format!("in(\"{r}\") in{n}[{i}]"))
        })
        .intersperse(", ".to_string())
        .collect();

    // Make this work with
    let outputs: Vec<_> = outputs_registers
        .iter()
        .enumerate()
        .flat_map(|(n, output_registers)| {
            output_registers
                .iter()
                .enumerate()
                .map(move |(i, r)| format!("lateout(\"{r}\") out{n}[{i}]"))
        })
        .intersperse(", ".to_string())
        .collect();

    let mut clobber_registers: BTreeSet<TypedSizedRegister<HardwareRegister>> = BTreeSet::new();
    instructions.iter().for_each(|instruction| {
        clobber_registers.extend(instruction.extract_registers().map(|reg| clobber(reg)));
    });

    let output_registers =
        BTreeSet::from_iter(outputs_registers.into_iter().flat_map(|r| r.into_iter()));

    // For the clobbers
    let clobbers = clobber_registers
        .difference(&output_registers)
        .map(|r| format!("lateout(\"{}\") _", r))
        .intersperse(", ".to_string());

    let newline = std::iter::once(",\n".to_string());
    // We jump to the assembly code with br so we need to safe the lr register
    // This can change in the future
    let lr = std::iter::once("lateout(\"lr\") _".to_string());

    inputs
        .into_iter()
        .chain(newline.clone())
        .chain(outputs)
        .chain(newline.clone())
        .chain(clobbers)
        .chain(newline.clone())
        .chain(lr)
        .collect()
}

/// For the clobber register we only have to mention the register
fn clobber(c: &TypedSizedRegister<HardwareRegister>) -> TypedSizedRegister<HardwareRegister> {
    // Unpack the fields of TypedSizedRegister using destructuring
    let TypedSizedRegister {
        reg,
        addressing,
        idx: _idx,
    } = c;

    let addressing = match addressing {
        Addressing::D => Addressing::V,
        other => *other,
    };

    // Return a new TypedSizedRegister with the same values
    TypedSizedRegister {
        reg: *reg,
        addressing,
        idx: Index::None,
    }
}

#[cfg(test)]
mod test {
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn interleave(lhs: Vec<u64>, rhs: Vec<u64>) -> bool {
        let left = lhs.len();
        let right = rhs.len();
        let res = super::interleave(lhs, rhs);
        res.len() == left + right
    }
}
