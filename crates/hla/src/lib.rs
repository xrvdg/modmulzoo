#![feature(iter_intersperse)]
use core::panic;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};

pub mod codegen;
pub mod frontend;
pub mod instructions;
pub mod ir;
pub mod reification;

pub use frontend::*;
pub use ir::*;
pub use reification::*;

#[derive(Debug)]
struct RegisterPool {
    // Keeps track of the free hardware during allocations
    pool: BTreeSet<HardwareRegister>,
    pinned: PinnedOutputRegisters,
}

#[derive(Debug)]
struct PinnedOutputRegisters {
    // Acts as a concrete iterator
    iter: BTreeSet<HardwareRegister>,
    // Keep track till when the pinned output register is free to use
    // for other variables.
    reserved: BTreeMap<HardwareRegister, (FreshRegister, usize)>,
}

impl PinnedOutputRegisters {
    fn new(iter: impl Iterator<Item = u64>) -> Self {
        Self {
            iter: BTreeSet::from_iter(iter.map(HardwareRegister)),
            reserved: BTreeMap::new(),
        }
    }

    fn reserve_output_register(
        &mut self,
        lifetimes: &Lifetimes,
        reified_register: &ReifiedRegister<FreshRegister>,
    ) {
        match self.iter.pop_first() {
            Some(hardware_register) => {
                let lifetime = lifetimes[reified_register.reg].begin;

                self.reserved
                    .insert(hardware_register, (reified_register.reg, lifetime));
            }
            None => panic!("Ran out of registers to reserve"),
        }
    }
}

impl RegisterPool {
    fn new<T>(registers: T) -> Self
    where
        T: Iterator<Item = u64> + Clone,
    {
        let pool = BTreeSet::from_iter(registers.clone().map(HardwareRegister));
        RegisterPool {
            pool,
            pinned: PinnedOutputRegisters::new(registers),
        }
    }

    fn pop_first(&mut self, reg: FreshRegister, end_lifetime: usize) -> Option<HardwareRegister> {
        // Find the first register that is free and will be free for the entirety of the lifetime of the fresh register.
        let reg = self
            .pool
            .iter()
            .find(
                // Check if the hardware register has been preassigned assigned to this fresh registers
                // Check if the hardware register can be used before it's preassigned moment
                |&hardware_register| match self.pinned.reserved.get(hardware_register) {
                    Some((tp, _lifetime)) if reg == *tp => true,
                    Some((_tp, lifetime)) if end_lifetime <= *lifetime => true,
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

    fn insert(&mut self, register: HardwareRegister) -> bool {
        self.pool.insert(register)
    }
}

pub fn allocate_input_variable(
    mapping: &mut RegisterMapping,
    register_bank: &mut RegisterBank,
    input_hw_registers: Vec<FreshVariable>,
    lifetimes: &Lifetimes,
) -> Vec<AllocatedVariable> {
    input_hw_registers
        .into_iter()
        .map(|variable| {
            let registers = variable
                .registers
                .into_iter()
                .map(|register| {
                    mapping
                        .get_or_allocate_register(register_bank, register, lifetimes[register.reg])
                        .to_basic_register()
                })
                .collect();
            AllocatedVariable {
                label: variable.label,
                registers,
            }
        })
        .collect()
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
pub fn reserve_output_variable(
    register_bank: &mut RegisterBank,
    lifetimes: &Lifetimes,
    variable: &FreshVariable,
) {
    let pool = register_bank.get_register_pool(variable.registers[0].r#type);
    for reified_register in &variable.registers {
        pool.pinned
            .reserve_output_register(lifetimes, reified_register);
    }
}

/// Tracks which registers have been seen during analysis.
///
/// This structure is used during liveness analysis to track which registers
/// have been processed.
pub struct Seen(HashSet<FreshRegister>);

impl Default for Seen {
    fn default() -> Self {
        Self::new()
    }
}

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
    fn mark_register(&mut self, fresh: &ReifiedRegister<FreshRegister>) -> bool {
        self.0.insert(fresh.reg)
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

impl Default for RegisterBank {
    fn default() -> Self {
        Self::new()
    }
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
    /// - Register 30 (reserved for link register)
    /// - Register 31 (reserved for stack pointer)
    pub fn new() -> Self {
        Self {
            x: RegisterPool::new((0..=17).chain(20..29)),
            v: RegisterPool::new(0..=31),
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
    ) -> Option<ReifiedRegister<HardwareRegister>> {
        let hw_reg = self
            .get_register_pool(reified_register.r#type)
            .pop_first(reified_register.reg, end_lifetime);

        hw_reg.map(|reg| reified_register.into_hardware(reg))
    }

    /// Returns a hardware register back to the register pool.
    ///
    /// # Returns
    ///
    /// `true` if the register was added to the pool, `false` if it was already in the pool.
    fn insert(&mut self, register: TypedHardwareRegister) -> bool {
        match register {
            TypedHardwareRegister::General(hardware_register) => self.x.insert(hardware_register),
            TypedHardwareRegister::Vector(hardware_register) => self.v.insert(hardware_register),
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
/// and their corresponding physical hardware registers _during_ register allocation.
/// It represents the active set of allocations.
#[derive(Debug, Default)]
pub struct RegisterMapping {
    mapping: HashMap<FreshRegister, TypedHardwareRegister>,
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
    pub fn assign_register(&mut self, fresh: FreshRegister, hardware: TypedHardwareRegister) {
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
    pub fn get_or_allocate_register(
        &mut self,
        register_bank: &mut RegisterBank,
        typed_register: ReifiedRegister<FreshRegister>,
        lifetime: Lifetime,
    ) -> ReifiedRegister<HardwareRegister> {
        // Either return existing mapping or create new one
        match self.mapping.get(&typed_register.reg) {
            Some(reg) => typed_register.into_hardware(reg.reg()),
            None => {
                let hardware_reified_register = register_bank
                    .pop_first(typed_register, lifetime.end)
                    .expect("ran out of registers");

                self.mapping.insert(
                    typed_register.reg,
                    hardware_reified_register.to_basic_register(),
                );
                hardware_reified_register
            }
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
    pub fn output_register(
        &self,
        reified_register: &ReifiedRegister<FreshRegister>,
    ) -> Option<ReifiedRegister<HardwareRegister>> {
        self.mapping
            .get(&reified_register.reg)
            .map(|hw_reg| ReifiedRegister {
                reg: hw_reg.reg(),
                r#type: reified_register.r#type,
                idx: Index::None,
            })
    }
}

#[derive(Clone, Copy)]
pub struct Lifetime {
    begin: usize,
    end: usize,
}

pub struct Lifetimes(Vec<Lifetime>);

impl Lifetimes {
    pub fn new(nr_fresh_registers: usize) -> Self {
        Self(vec![
            Lifetime {
                begin: usize::MAX,
                end: usize::MAX,
            };
            nr_fresh_registers
        ])
    }
}

impl std::ops::Index<FreshRegister> for Lifetimes {
    type Output = Lifetime;

    fn index(&self, index: FreshRegister) -> &Self::Output {
        &self.0[index.0 as usize]
    }
}

impl std::ops::IndexMut<FreshRegister> for Lifetimes {
    fn index_mut(&mut self, index: FreshRegister) -> &mut Self::Output {
        &mut self.0[index.0 as usize]
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
pub fn liveness_analysis(
    output_variables: &[FreshVariable],
    instructions: &[Instruction],
    nr_fresh_registers: usize,
) -> (VecDeque<HashSet<FreshRegister>>, Lifetimes) {
    // Initialize the seen_registers with the output registers such that they won't get released.
    let mut seen_registers = Seen::new();
    output_variables.iter().for_each(|variable| {
        variable.registers.iter().for_each(|register| {
            seen_registers.mark_register(register);
        });
    });

    // Keep track of the last line the free register is used for
    let mut lifetimes = Lifetimes::new(nr_fresh_registers);
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
                print_instructions(instructions);
                panic!("{line}: {instruction:?} does not use the destination")
            }; // The union could be mutable

            let lifetime = &mut lifetimes[dest];
            lifetime.begin = line;
        });
        release.iter().for_each(|reg| {
            let lifetime = &mut lifetimes[*reg];
            lifetime.end = line;
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
    lifetimes: Lifetimes,
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
                    let idx = d.reg;
                    mapping.get_or_allocate_register(register_bank, d, lifetimes[idx])
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
