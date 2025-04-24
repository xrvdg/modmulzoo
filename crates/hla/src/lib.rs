#![feature(iter_intersperse)]
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

pub mod codegen;
pub mod frontend;
pub mod instructions;
pub mod reification;

pub use frontend::*;
pub use reification::*;

pub type AtomicInstruction = Vec<InstructionF<FreshRegister>>;
pub type Instruction = InstructionF<FreshRegister>;

/// This instruction models both aliases and regular instructions
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
    fn extract_registers(&self) -> impl Iterator<Item = &ReifiedRegister<R>> {
        self.results.iter().chain(&self.operands)
    }
}

/// FreshRegister represent the label for a fresh variable which is hidden inside
/// a Reg.
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
    // A counter for the fresh variable labels
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

#[derive(PartialEq, Debug, Hash, Ord, PartialOrd, Eq, Clone, Copy)]
pub struct HardwareRegister(u64);

impl std::fmt::Display for HardwareRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Basic register represents as it is contained within the
/// register banks. It does not have any kind information nor indexing.
#[derive(Clone, Copy, PartialEq, Debug, Eq, Ord, PartialOrd)]
pub enum BasicRegister {
    General(HardwareRegister),
    Vector(HardwareRegister),
}

impl BasicRegister {
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

pub fn pin_register<T: ReifyRegister>(
    register_bank: &mut RegisterBank,
    lifetimes: &Vec<(usize, usize)>,
    fresh: &T,
    hardware_register: u64,
) where
    T: ReifyRegister,
{
    let hardware_register = HardwareRegister(hardware_register);
    let tp = fresh.reify();

    register_bank.set_availability(hardware_register, tp, lifetimes[tp.reg.0 as usize].0);
}

pub struct Seen(HashSet<FreshRegister>);

impl Seen {
    pub fn new() -> Self {
        Self(HashSet::new())
    }

    pub fn output_interface<T: ReifyRegister>(&mut self, fresh: &T) -> bool {
        self.seen(fresh.reify().reg)
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

    fn get_register_pool(&mut self, r#type: RegisterType) -> &mut RegisterPool {
        match r#type {
            RegisterType::X => &mut self.x,
            RegisterType::V | RegisterType::D => &mut self.v,
        }
    }

    fn pop_first(
        &mut self,
        tp: ReifiedRegister<FreshRegister>,
        end_lifetime: usize,
    ) -> Option<HardwareRegister> {
        self.get_register_pool(tp.r#type)
            .pop_first(tp.reg, end_lifetime)
    }

    fn remove(&mut self, register: HardwareRegister, addr: RegisterType) -> bool {
        self.get_register_pool(addr).remove(&register)
    }

    fn set_availability(
        &mut self,
        register: HardwareRegister,
        tp: ReifiedRegister<FreshRegister>,
        lifetime: usize,
    ) {
        self.get_register_pool(tp.r#type)
            .set_availability(tp.reg, register, lifetime);
    }

    /// Return the hardware register back into the register pool
    fn insert(&mut self, register: BasicRegister) -> bool {
        match register {
            BasicRegister::General(hardware_register) => self.x.insert(hardware_register),
            BasicRegister::Vector(hardware_register) => self.v.insert(hardware_register),
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

/// A mapping from FreshRegister to its hardware assignment state
#[derive(Debug, Default)]
pub struct RegisterMapping {
    mapping: HashMap<FreshRegister, BasicRegister>,
    // dropped is not strictly necessary.
    dropped: HashSet<FreshRegister>,
}

impl RegisterMapping {
    pub fn new() -> Self {
        Self {
            mapping: HashMap::with_capacity(100),
            dropped: HashSet::with_capacity(100),
        }
    }

    pub fn allocated(&self) -> usize {
        self.mapping.len()
    }

    /// Directly assign a register (used by input function for compatibility)
    pub fn assign_register(&mut self, fresh: FreshRegister, hardware: BasicRegister) {
        self.mapping.insert(fresh, hardware);
    }

    /// Get the physical register for a source register
    fn get_register(
        &self,
        fresh: ReifiedRegister<FreshRegister>,
    ) -> ReifiedRegister<HardwareRegister> {
        assert!(
            !self.dropped.contains(&fresh.reg),
            "{:?} already has been dropped",
            fresh
        );

        match self.mapping.get(&fresh.reg) {
            Some(reg) => fresh.into_hardware(reg.reg()),
            None => panic!("{:?} has not been assigned yet", fresh),
        }
    }

    /// Get or allocate a register
    fn get_or_allocate_register(
        &mut self,
        register_bank: &mut RegisterBank,
        typed_register: ReifiedRegister<FreshRegister>,
        end_lifetime: usize,
    ) -> ReifiedRegister<HardwareRegister> {
        assert!(
            !self.dropped.contains(&typed_register.reg),
            "{:?} already has been dropped",
            typed_register
        );

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

    /// Free a register, returning it to the register bank
    fn free_register(&mut self, register_bank: &mut RegisterBank, fresh: FreshRegister) -> bool {
        assert!(
            !self.dropped.contains(&fresh),
            "Register {:?} has already been dropped",
            fresh
        );

        if let Some(reg) = self.mapping.remove(&fresh) {
            self.dropped.insert(fresh);
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

    pub fn output_register<R: ReifyRegister>(
        &self,
        reg: &R,
    ) -> Option<ReifiedRegister<HardwareRegister>> {
        let reified_register = reg.reify();
        if self.dropped.contains(&reified_register.reg) {
            return None;
        }

        self.mapping
            .get(&reified_register.reg)
            .map(|hw_reg| ReifiedRegister {
                reg: hw_reg.reg(),
                r#type: reified_register.r#type,
                idx: Index::None,
            })
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

pub fn print_instructions<R: std::fmt::Display + Copy>(instrs: &[InstructionF<R>]) {
    instrs
        .iter()
        .enumerate()
        .for_each(|(line, inst)| println!("{line}: {}", inst));
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
