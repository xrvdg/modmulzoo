#![feature(iter_intersperse)]
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

pub mod codegen;
pub mod frontend;
pub mod instructions;
pub mod reification;

pub use frontend::*;
pub use reification::*;

/// A vector of instructions representing an atomic unit of execution.
///
/// This type represents a sequence of instructions that should be executed together
/// as they rely on side effects such as flag setting that could potentially be disturbed when interleaved.
pub type AtomicInstruction = Vec<InstructionF<FreshRegister>>;

/// An alias for an instruction using fresh registers.
///
/// This type represents a single machine instruction that operates on virtual registers
/// (fresh registers) before hardware register allocation occurs.
pub type Instruction = InstructionF<FreshRegister>;

/// A generic instruction representation that can work with different register types.
///
/// This instruction models both regular machine instructions and register aliases.
/// It contains the opcode, result registers, operand registers, and any modifiers.
///
/// # Type Parameters
///
/// * `R` - The register type is either `FreshRegister` for virtual registers
///   or `HardwareRegister` for physical machine registers.
#[derive(Debug, PartialEq)]
pub struct InstructionF<R> {
    opcode: String,
    // Result is a vector because:
    // - Some operations have do not write results to a register
    //   - CMN only affects flags
    //   - STR writes to a destination stored in operands
    // - LDP has 2 destinations
    results: Vec<ReifiedRegister<R>>,
    operands: Vec<ReifiedRegister<R>>,
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
    /// Returns an iterator over all registers referenced by this instruction.
    ///
    /// The iterator includes both result registers and operand registers.
    fn extract_registers(&self) -> impl Iterator<Item = &ReifiedRegister<R>> {
        self.results.iter().chain(&self.operands)
    }
}

/// A virtual register identifier used before hardware register allocation.
///
/// FreshRegister represents a unique label for a variable in the intermediate
/// representation. It serves as a placeholder for a hardware register that will
/// be assigned during the register allocation phase.
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

/// A container for assembly instructions.
///
/// The Assembler maintains a collection of atomic instruction blocks that
/// make up a program. Instructions are appended to build up the program in
/// a way similar to a Write/State monad.
pub struct Assembler {
    pub instructions: Vec<AtomicInstruction>,
}

impl Assembler {
    /// Creates a new empty Assembler.
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
        }
    }

    /// Appends an atomic instruction block to the assembler.
    pub fn append_instruction(&mut self, inst: AtomicInstruction) {
        self.instructions.push(inst)
    }
}

/// Generates fresh register identifiers for intermediate code.
///
/// The Allocator maintains a counter to generate unique FreshRegister
/// identifiers that represent virtual registers in the intermediate code.
#[derive(Debug)]
pub struct Allocator {
    /// Counter for the fresh variable labels
    pub fresh: u64,
}

impl Allocator {
    /// Generates a new fresh register of the specified type.
    pub fn fresh<T>(&mut self) -> Reg<T> {
        let x = self.fresh;
        self.fresh += 1;
        Reg::new(x)
    }

    /// Creates a new Allocator
    pub fn new() -> Self {
        Self { fresh: 0 }
    }
}

/// Represents a physical hardware register.
///
/// HardwareRegister is a wrapper around a register number that identifies
/// a specific register in the target CPU architecture.
#[derive(PartialEq, Debug, Hash, Ord, PartialOrd, Eq, Clone, Copy)]
pub struct HardwareRegister(u64);

impl std::fmt::Display for HardwareRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Represents a basic hardware register with its type (general or vector).
///
/// BasicRegister describes a physical register as it is contained within the
/// register banks. It does not have any kind information nor indexing.
#[derive(Clone, Copy, PartialEq, Debug, Eq, Ord, PartialOrd)]
pub enum BasicRegister {
    /// A general purpose register (like x0-x31 on ARM64)
    General(HardwareRegister),
    /// A vector register (like v0-v31 on ARM64)
    Vector(HardwareRegister),
}

impl BasicRegister {
    /// Extracts the hardware register number from the basic register.
    fn reg(&self) -> HardwareRegister {
        match self {
            BasicRegister::General(reg) | BasicRegister::Vector(reg) => *reg,
        }
    }
}

impl std::fmt::Display for BasicRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BasicRegister::General(reg) => write!(f, "x{}", reg.0),
            BasicRegister::Vector(reg) => write!(f, "v{}", reg.0),
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

/// Creates an input register binding to a specific hardware register.
///
/// This function creates a fresh register and binds it to a specific hardware register.
/// It is typically used for handling input values that need to come from specific
/// hardware registers.
///
/// # Arguments
///
/// * `asm` - Allocator to generate a fresh register
/// * `mapping` - RegisterMapping to store the binding
/// * `register_bank` - RegisterBank to allocate from
/// * `phys` - Physical register number to bind to
///
/// # Returns
///
/// A new `Reg<T>` that is bound to the specified hardware register.
///
/// # Panics
///
/// Panics if the specified hardware register is already in use.
pub fn input<T>(
    asm: &mut Allocator,
    mapping: &mut RegisterMapping,
    register_bank: &mut RegisterBank,
    phys: u64,
) -> Reg<T>
where
    Reg<T>: ReifyRegister,
{
    let fresh = asm.fresh();

    let hw_reg = HardwareRegister(phys);
    let reified_register = fresh.reify().into_hardware(hw_reg);

    if !register_bank.remove(hw_reg, reified_register.r#type) {
        panic!("{:?} is already in use", phys)
    }

    mapping.assign_register(fresh.reg, reified_register.to_basic_register());

    fresh
}

/// Pins a fresh register to a specific hardware register.
///
/// This function assigns a specific hardware register to a fresh register,
/// ensuring that register allocation will use the specified hardware register.
///
/// # Arguments
///
/// * `register_bank` - RegisterBank to pin the register in
/// * `lifetimes` - Register lifetimes for allocation planning. It will use the begin value.
/// * `fresh` - The fresh register to pin
/// * `hardware_register` - The hardware register number to pin to
pub fn pin_register<T: ReifyRegister>(
    register_bank: &mut RegisterBank,
    lifetimes: &[(usize, usize)],
    fresh: &T,
    hardware_register: u64,
) where
    T: ReifyRegister,
{
    let hardware_register = HardwareRegister(hardware_register);
    let tp = fresh.reify();

    register_bank.set_availability(hardware_register, tp, lifetimes[tp.reg.0 as usize].0);
}

/// Tracks which registers have been seen during analysis.
///
/// This structure is used during liveness analysis to track which registers
/// have been processed.
pub struct Seen(HashSet<FreshRegister>);

impl Seen {
    /// Creates a new empty Seen instance.
    pub fn new() -> Self {
        Self(HashSet::new())
    }

    /// Marks a register as seen and returns whether it was previously unseen.
    ///
    /// # Arguments
    ///
    /// * `fresh` - The register to mark
    ///
    /// # Returns
    ///
    /// `true` if the register was not previously seen, `false` otherwise.
    pub fn mark_register<T: ReifyRegister>(&mut self, fresh: &T) -> bool {
        let fresh = fresh.reify().reg;
        self.0.insert(fresh)
    }
}

/// Manages pools of hardware registers for allocation.
///
/// RegisterBank maintains separate pools for general-purpose registers and
/// vector registers. It handles allocation and deallocation of hardware registers.
#[derive(Debug)]
pub struct RegisterBank {
    x: RegisterPool,
    v: RegisterPool,
}

impl RegisterBank {
    /// Creates a new RegisterBank with default register pools.
    ///
    /// # Returns
    ///
    /// A new RegisterBank with general-purpose and vector register pools.
    /// Certain registers are excluded:
    /// - Register 18 (reserved by OS)
    /// - Register 19 (reserved by LLVM)
    pub fn new() -> Self {
        Self {
            x: RegisterPool::new((0..=17).chain(20..29)),
            v: RegisterPool::new(0..=30),
        }
    }

    /// Gets the appropriate register pool based on register type.
    ///
    /// # Arguments
    ///
    /// * `r#type` - The register type (X, V, or D)
    ///
    /// # Returns
    ///
    /// A mutable reference to the corresponding register pool.
    fn get_register_pool(&mut self, r#type: RegisterType) -> &mut RegisterPool {
        match r#type {
            RegisterType::X => &mut self.x,
            RegisterType::V | RegisterType::D => &mut self.v,
        }
    }

    /// Allocates a hardware register for a fresh register.
    ///
    /// # Arguments
    ///
    /// * `tp` - The fresh register to allocate for
    /// * `end_lifetime` - The instruction index after which the register is no longer needed
    ///
    /// # Returns
    ///
    /// An Option containing the allocated hardware register, or None if allocation failed.
    fn pop_first(
        &mut self,
        reified_register: ReifiedRegister<FreshRegister>,
        end_lifetime: usize,
    ) -> Option<HardwareRegister> {
        self.get_register_pool(reified_register.r#type)
            .pop_first(reified_register.reg, end_lifetime)
    }

    /// Removes a hardware register from the pool.
    ///
    /// # Arguments
    ///
    /// * `register` - The hardware register to remove
    /// * `register_type` - The type of register
    ///
    /// # Returns
    ///
    /// `true` if the register was removed, `false` if it wasn't in the pool.
    fn remove(&mut self, register: HardwareRegister, register_type: RegisterType) -> bool {
        self.get_register_pool(register_type).remove(&register)
    }

    /// Sets the availability of a hardware register for a specific fresh register.
    ///
    /// # Arguments
    ///
    /// * `hardware_register` - The hardware register to set availability for
    /// * `reified_register` - The fresh register to associate with the hardware register
    /// * `lifetime` - The lifetime (instruction index) at which the register becomes available
    fn set_availability(
        &mut self,
        hardware_register: HardwareRegister,
        reified_register: ReifiedRegister<FreshRegister>,
        lifetime: usize,
    ) {
        self.get_register_pool(reified_register.r#type)
            .set_availability(reified_register.reg, hardware_register, lifetime);
    }

    /// Returns a hardware register back to the register pool.
    ///
    /// # Returns
    ///
    /// `true` if the register was added to the pool, `false` if it was already in the pool.
    fn insert(&mut self, register: BasicRegister) -> bool {
        match register {
            BasicRegister::General(hardware_register) => self.x.insert(hardware_register),
            BasicRegister::Vector(hardware_register) => self.v.insert(hardware_register),
        }
    }
}

/// Interleaves elements from two vectors.
///
/// This function combines elements from two vectors, distributing the elements
/// from the shorter vector evenly throughout the longer vector.
///
/// # Arguments
///
/// * `lhs` - First vector of elements
/// * `rhs` - Second vector of elements
///
/// # Returns
///
/// A new vector containing all elements from both input vectors, interleaved.
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

/// Maps fresh registers to their assigned hardware registers.
///
/// RegisterMapping maintains the mapping between virtual registers (FreshRegisters)
/// and their corresponding physical hardware registers during register allocation.
#[derive(Debug, Default)]
pub struct RegisterMapping {
    mapping: HashMap<FreshRegister, BasicRegister>,
}

impl RegisterMapping {
    /// Creates a new empty RegisterMapping.
    pub fn new() -> Self {
        Self {
            mapping: HashMap::with_capacity(100),
        }
    }

    /// Returns the number of registers currently allocated.
    pub fn allocated(&self) -> usize {
        self.mapping.len()
    }

    /// Directly assigns a hardware register to a fresh register.
    ///
    /// # Arguments
    ///
    /// * `fresh` - The fresh register to assign
    /// * `hardware` - The hardware register to assign to
    pub fn assign_register(&mut self, fresh: FreshRegister, hardware: BasicRegister) {
        self.mapping.insert(fresh, hardware);
    }

    /// Gets the physical register for an operand.
    ///
    /// # Arguments
    ///
    /// * `fresh` - The reified fresh register to look up
    ///
    /// # Returns
    ///
    /// The corresponding hardware register.
    ///
    /// # Panics
    ///
    /// Panics if the register has not been assigned yet.
    fn get_register(
        &self,
        fresh: ReifiedRegister<FreshRegister>,
    ) -> ReifiedRegister<HardwareRegister> {
        match self.mapping.get(&fresh.reg) {
            Some(reg) => fresh.into_hardware(reg.reg()),
            None => panic!("{:?} has not been assigned yet", fresh),
        }
    }

    /// Gets or allocates a register.
    ///
    /// If the register is already mapped, returns the existing mapping.
    /// Otherwise, allocates a new hardware register.
    ///
    /// # Arguments
    ///
    /// * `register_bank` - The register bank to allocate from
    /// * `typed_register` - The fresh register to get or allocate
    /// * `end_lifetime` - The instruction index after which the register is no longer needed
    ///
    /// # Returns
    ///
    /// The corresponding hardware register.
    ///
    /// # Panics
    ///
    /// Panics if register allocation fails.
    fn get_or_allocate_register(
        &mut self,
        register_bank: &mut RegisterBank,
        typed_register: ReifiedRegister<FreshRegister>,
        end_lifetime: usize,
    ) -> ReifiedRegister<HardwareRegister> {
        // Either return existing mapping or create new one
        let hw_reg = match self.mapping.get(&typed_register.reg) {
            Some(reg) => reg.reg(),
            None => {
                let hw_reg = register_bank
                    .pop_first(typed_register, end_lifetime)
                    .expect("ran out of registers");

                let hardware_reified_register = typed_register.into_hardware(hw_reg);
                self.mapping.insert(
                    typed_register.reg,
                    hardware_reified_register.to_basic_register(),
                );
                hw_reg
            }
        };

        ReifiedRegister {
            reg: hw_reg,
            r#type: typed_register.r#type,
            idx: typed_register.idx,
        }
    }

    /// Frees a register, returning it to the register bank.
    ///
    /// # Returns
    ///
    /// `true` if the register was freed, `false` otherwise.
    fn free_register(&mut self, register_bank: &mut RegisterBank, fresh: FreshRegister) -> bool {
        if let Some(reg) = self.mapping.remove(&fresh) {
            // TODO this assert needs to be moved into insert and that should also solve the todo
            let result = register_bank.insert(reg);
            assert!(
                result,
                "hardware:{reg:?} is assigned to more than one fresh register."
            );
            result
        } else {
            todo!()
        }
    }

    /// Gets the hardware register assigned to a register if available.
    ///
    /// # Returns
    ///
    /// An Option containing the corresponding hardware register if mapped, None otherwise.
    pub fn output_register<R: ReifyRegister>(
        &self,
        reg: &R,
    ) -> Option<ReifiedRegister<HardwareRegister>> {
        let reified_register = reg.reify();

        self.mapping
            .get(&reified_register.reg)
            .map(|hw_reg| ReifiedRegister {
                reg: hw_reg.reg(),
                r#type: reified_register.r#type,
                idx: Index::None,
            })
    }
}

/// Performs liveness analysis on instructions to determine register lifetimes.
///
/// This function analyzes the instruction sequence to determine at which instructions
/// each register is last used, allowing for register deallocation at the earliest possible point.
///
/// # Arguments
///
/// * `output_registers` - The registers that contain the results at the end of the instructions.
/// * `instructions` - The instruction sequence to analyze
/// * `nr_fresh_registers` - The total number of fresh registers used
///
/// # Returns
///
/// A tuple containing:
/// * A queue of sets of registers to release after each instruction
/// * A vector of (begin, end) lifetime indices for each register
///
/// # Panics
///
/// Panics if an instruction has an unused destination register.
pub fn liveness_analysis<'a, T>(
    output_registers: impl Iterator<Item = &'a T>,
    instructions: &[Instruction],
    nr_fresh_registers: usize,
) -> (VecDeque<HashSet<FreshRegister>>, Vec<(usize, usize)>)
where
    T: ReifyRegister + 'a,
{
    // Initialize the seen_registers with the output registers such that they won't get released.
    let mut seen_registers = Seen::new();
    output_registers.for_each(|r| {
        seen_registers.mark_register(r);
    });

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

/// Prints a formatted list of instructions for debugging.
pub fn print_instructions<R: std::fmt::Display + Copy>(instructions: &[InstructionF<R>]) {
    instructions
        .iter()
        .enumerate()
        .for_each(|(line, inst)| println!("{line}: {}", inst));
}

/// Allocates hardware registers for a sequence of instructions.
///
/// This function transforms instructions using fresh registers into instructions
/// using hardware registers, performing register allocation based on the results
/// of liveness analysis.
///
/// # Arguments
///
/// * `mapping` - The register mapping to use and update
/// * `register_bank` - The register bank to allocate from
/// * `instructions` - The instruction sequence using fresh registers
/// * `releases` - The registers to release after each instruction
/// * `lifetimes` - The lifetime information for each register
///
/// # Returns
///
/// A new sequence of instructions using hardware registers.
///
/// # Panics
///
/// Panics if the instructions and releases collections have different lengths.
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
        "The instructions and release collections need to be the same length"
    );

    instructions
        .into_iter()
        .zip(releases)
        .map(|(instruction, release)| {
            // Map operands to hardware registers
            let src = instruction
                .operands
                .into_iter()
                .map(|s| mapping.get_register(s))
                .collect();

            // Free registers that are no longer needed
            release.into_iter().for_each(|fresh| {
                mapping.free_register(register_bank, fresh);
            });

            // Allocate result registers
            let dest = instruction
                .results
                .into_iter()
                .map(|d| {
                    let idx = d.reg.0;
                    mapping.get_or_allocate_register(register_bank, d, lifetimes[idx as usize].1)
                })
                .collect();

            // Construct the hardware instruction
            InstructionF {
                opcode: instruction.opcode,
                results: dest,
                operands: src,
                modifiers: instruction.modifiers,
            }
        })
        .collect()
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
