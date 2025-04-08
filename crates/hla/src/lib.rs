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
    // LS could be combined with Imm and let the compiler backend deal with it.
    LS(u8),
    Cond(String),
}

// TODO This could benefit from having really different types for FreshRegister and
// Hardware Register. The output could be made different for this
impl<R: std::fmt::Display> std::fmt::Display for TypedSizedRegister<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reg = &self.reg;
        let addr = self.addressing;
        match self.idx {
            Index::Lane(idx) => write!(f, "{addr}{reg}[{idx}]"),
            Index::None => write!(f, "{addr}{reg}"),
            Index::LaneSized(lane_sizes, idx) => write!(f, "{addr}{reg}.{lane_sizes}[{idx}]"),
        }
    }
}

impl std::fmt::Display for LaneSizes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LaneSizes::S => write!(f, "s"),
            LaneSizes::D => write!(f, "d"),
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
            Mod::ImmLSL(imm, shift) => format!(", #{imm}, lsl {shift}"),
            Mod::LS(imm) => format!(", #{imm}"),
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

pub trait SIMD {}

impl<T, const N: usize> SIMD for Simd<T, N> {}
impl<T: SIMD, const I: u8> SIMD for Idx<T, I> {}

// Create a new type for b that takes into account the index
pub fn fmla2d<S: SIMD + RegisterSource>(
    _alloc: &mut Allocator, // Done to have the same pattern as the rest
    asm: &mut Assembler,
    add: Reg<Simd<f64, 2>>,
    a: &Reg<Simd<f64, 2>>,
    b: &Reg<S>, // Trait bound a bit too loose, but for now don't want to add the complexity necessary
) -> Reg<Simd<f64, 2>> {
    asm.append_instruction(vec![fmla2d_inst(&add, a, b)]);
    add
}

pub fn fmla2d_inst<S: SIMD + RegisterSource>(
    dest_add: &Reg<Simd<f64, 2>>,
    a: &Reg<Simd<f64, 2>>,
    b: &Reg<S>,
) -> Instruction {
    InstructionF {
        opcode: "fmla.2d".to_string(),
        dest: Some(dest_add.to_typed_register()),
        src: vec![a.to_typed_register(), b.to_typed_register()],
        modifiers: Mod::None,
    }
}

// Could add ins that returns consumes and returns the register
pub fn ins_inst<const L: u8, const I: u8>(
    dest: &Reg<IdxSized<Simd<u64, 2>, L, I>>,
    a: &Reg<u64>,
) -> Instruction {
    InstructionF {
        opcode: "ins".to_string(),
        dest: Some(dest.to_typed_register()),
        src: vec![a.to_typed_register()],
        modifiers: Mod::None,
    }
}

pub fn shl2d(
    alloc: &mut Allocator,
    asm: &mut Assembler,
    a: &Reg<Simd<u64, 2>>,
    imm: u8,
) -> Reg<Simd<u64, 2>> {
    let ret = alloc.fresh();
    asm.append_instruction(vec![shl2d_inst(&ret, a, imm)]);
    ret
}

pub fn shl2d_inst(dest: &Reg<Simd<u64, 2>>, a: &Reg<Simd<u64, 2>>, imm: u8) -> Instruction {
    InstructionF {
        opcode: "shl.2d".to_string(),
        dest: Some(dest.to_typed_register()),
        src: vec![a.to_typed_register()],
        modifiers: Mod::LS(imm),
    }
}

pub fn ushr2d(
    alloc: &mut Allocator,
    asm: &mut Assembler,
    a: &Reg<Simd<u64, 2>>,
    imm: u8,
) -> Reg<Simd<u64, 2>> {
    let ret = alloc.fresh();
    asm.append_instruction(vec![ushr2d_inst(&ret, a, imm)]);
    ret
}

pub fn ushr2d_inst(dest: &Reg<Simd<u64, 2>>, a: &Reg<Simd<u64, 2>>, imm: u8) -> Instruction {
    InstructionF {
        opcode: "ushr.2d".to_string(),
        dest: Some(dest.to_typed_register()),
        src: vec![a.to_typed_register()],
        modifiers: Mod::LS(imm),
    }
}

pub fn usra2d(
    _alloc: &mut Allocator,
    asm: &mut Assembler,
    add: Reg<Simd<u64, 2>>,
    a: &Reg<Simd<u64, 2>>,
    imm: u8,
) -> Reg<Simd<u64, 2>> {
    asm.append_instruction(vec![usra2d_inst(&add, a, imm)]);
    add
}

pub fn usra2d_inst(dest: &Reg<Simd<u64, 2>>, a: &Reg<Simd<u64, 2>>, imm: u8) -> Instruction {
    InstructionF {
        opcode: "usra.2d".to_string(),
        dest: Some(dest.to_typed_register()),
        src: vec![a.to_typed_register()],
        modifiers: Mod::LS(imm),
    }
}

pub fn ssra2d(
    _alloc: &mut Allocator,
    asm: &mut Assembler,
    add: Reg<Simd<i64, 2>>,
    a: &Reg<Simd<i64, 2>>,
    imm: u8,
) -> Reg<Simd<i64, 2>> {
    asm.append_instruction(vec![ssra2d_inst(&add, a, imm)]);
    add
}

pub fn ssra2d_inst(dest: &Reg<Simd<i64, 2>>, a: &Reg<Simd<i64, 2>>, imm: u8) -> Instruction {
    InstructionF {
        opcode: "ssra.2d".to_string(),
        dest: Some(dest.to_typed_register()),
        src: vec![a.to_typed_register()],
        modifiers: Mod::LS(imm),
    }
}

pub fn umov<const Lanes: u8, const I: u8>(
    alloc: &mut Allocator,
    asm: &mut Assembler,
    a: &Reg<IdxSized<Simd<u64, 2>, Lanes, I>>,
) -> Reg<u64> {
    let ret = alloc.fresh();
    asm.append_instruction(vec![umov_inst(&ret, a)]);
    ret
}
pub fn umov_inst<const Lanes: u8, const I: u8>(
    dest: &Reg<u64>,
    a: &Reg<IdxSized<Simd<u64, 2>, Lanes, I>>,
) -> Instruction {
    InstructionF {
        opcode: "umov".to_string(),
        dest: Some(dest.to_typed_register()),
        src: vec![a.to_typed_register()],
        modifiers: Mod::None,
    }
}

pub fn cmeq2d(
    alloc: &mut Allocator,
    asm: &mut Assembler,
    a: &Reg<Simd<u64, 2>>,
    imm: u64,
) -> Reg<Simd<u64, 2>> {
    let ret = alloc.fresh();
    asm.append_instruction(vec![cmeq2d_inst(&ret, a, imm)]);
    ret
}
pub fn cmeq2d_inst(dest: &Reg<Simd<u64, 2>>, a: &Reg<Simd<u64, 2>>, imm: u64) -> Instruction {
    InstructionF {
        opcode: "cmeq.2d".to_string(),
        dest: Some(dest.to_typed_register()),
        src: vec![a.to_typed_register()],
        modifiers: Mod::Imm(imm),
    }
}

embed_asm!(mul, "mul", (a: u64, b: u64) -> u64);
embed_asm!(umulh, "umulh", (a: u64, b: u64) -> u64);

embed_asm!(add, "add", (a: u64, b: u64) -> u64);
embed_asm!(and, "and", (a: u64, b: u64) -> u64);
// TODO: These operations set flags and should only make their inst available
embed_asm!(adds, "adds", (a: u64, b: u64) -> u64);
embed_asm!(subs, "subs", (a: u64, b: u64) -> u64);
embed_asm!(sbcs, "sbcs", (a: u64, b: u64) -> u64);

// Doesn't support immediates
embed_asm!(mov16b, "mov.16b", (a: Simd<u64,2>) -> Simd<u64,2>);
embed_asm!(ucvtf2d, "ucvtf.2d", (a: Simd<u64,2>) -> Simd<f64,2>);
embed_asm!(dup2d, "dup.2d", (a: u64) -> Simd<u64,2>);
embed_asm!(ucvtf, "ucvtf", (a: u64) -> f64);
embed_asm!(and16, "and.16b", (a: Simd<u64,2>, b: Simd<u64,2>) -> Simd<u64,2>);
embed_asm!(bic16, "bic.16b", (a: Simd<u64,2>, b: Simd<u64,2>) -> Simd<u64,2>);
embed_asm!(add2d, "add.2d", (a: Simd<u64,2>, b: Simd<u64,2>) -> Simd<u64,2>);
embed_asm!(sub2d, "sub.2d", (a: Simd<i64,2>, b: Simd<i64,2>) -> Simd<i64,2>);
embed_asm!(fsub2d, "fsub.2d", (a: Simd<f64,2>, b: Simd<f64,2>) -> Simd<f64,2>);
embed_asm!(orr16, "orr.16b", (a: Simd<u64,2>, b: Simd<u64,2>) -> Simd<u64,2>);

pub struct Reg<T> {
    reg: FreshRegister,
    _marker: PhantomData<T>,
}

/// Define the struct ourself as to not have to import it
pub struct Simd<T, const N: usize>(PhantomData<T>);
pub struct Idx<T, const I: u8>(PhantomData<T>);
// TODO better separated into Sized and Idx
pub struct IdxSized<T, const Lanes: u8, const I: u8>(PhantomData<T>);

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

#[derive(Debug, PartialOrd, Ord, Eq, Hash, PartialEq, Clone, Copy)]
enum LaneSizes {
    S,
    D,
}

#[derive(Debug, PartialOrd, Ord, Eq, Hash, PartialEq, Clone, Copy)]
enum Index {
    None,
    Lane(u8),
    LaneSized(LaneSizes, u8),
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

impl<T> Reg<Simd<T, 2>> {
    pub fn into_<D>(self) -> Reg<Simd<D, 2>> {
        unsafe { std::mem::transmute(self) }
    }

    pub fn as_<D>(&self) -> &Reg<Simd<D, 2>> {
        unsafe { std::mem::transmute(self) }
    }

    pub fn _0(&self) -> &Reg<Idx<Simd<T, 2>, 0>> {
        unsafe { std::mem::transmute(self) }
    }

    pub fn _1(&self) -> &Reg<Idx<Simd<T, 2>, 1>> {
        unsafe { std::mem::transmute(self) }
    }

    pub fn _d0(&self) -> &Reg<IdxSized<Simd<T, 2>, 2, 0>> {
        unsafe { std::mem::transmute(self) }
    }

    pub fn _d1(&self) -> &Reg<IdxSized<Simd<T, 2>, 2, 1>> {
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
            idx: Index::None,
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
            idx: Index::None,
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
            idx: Index::None,
        }
    }
}

impl<T: RegisterSource, const I: u8> RegisterSource for Idx<T, I> {
    fn get_register_pool(pools: &mut RegisterBank) -> &mut RegisterPool {
        &mut pools.v
    }

    fn to_typed_register<R>(reg: R) -> TypedSizedRegister<R> {
        let mut tp = T::to_typed_register(reg);
        tp.idx = Index::Lane(I);
        tp
    }
}

impl<T: RegisterSource, const I: u8, const Lanes: u8> RegisterSource for IdxSized<T, Lanes, I> {
    fn get_register_pool(pools: &mut RegisterBank) -> &mut RegisterPool {
        &mut pools.v
    }

    fn to_typed_register<R>(reg: R) -> TypedSizedRegister<R> {
        let mut tp = T::to_typed_register(reg);

        let sizes = match Lanes {
            2 => LaneSizes::D,
            4 => LaneSizes::S,
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
    T: RegisterSource,
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
            x: BTreeSet::from_iter((0..=17).chain(19..29).map(HardwareRegister)),
            v: BTreeSet::from_iter((0..=30).map(HardwareRegister)),
        }
    }

    fn get_register_pool(&mut self, addr: Addressing) -> &mut RegisterPool {
        match addr {
            Addressing::X => &mut self.x,
            Addressing::V | Addressing::D => &mut self.v,
        }
    }

    fn pop_first(&mut self, addr: Addressing) -> Option<HardwareRegister> {
        self.get_register_pool(addr).pop_first()
    }

    fn remove(&mut self, register: HardwareRegister, addr: Addressing) -> bool {
        self.get_register_pool(addr).remove(&register)
    }

    /// Return the hardware register back into the register pool
    fn insert(&mut self, register: Pool) -> bool {
        match register {
            Pool::General(hardware_register) => self.x.insert(hardware_register),
            Pool::Vector(hardware_register) => self.v.insert(hardware_register),
        }
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
                .take(1000)
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
    ) -> TypedSizedRegister<HardwareRegister> {
        // Possible to do a mutable reference here
        let entry = self.index_mut(*typed_register.as_fresh());
        let hw_reg = match *entry {
            RegisterState::Unassigned => {
                let addr = typed_register.addressing;
                let hw_reg = register_bank.pop_first(addr).expect("ran out of registers");

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
    pub fn output_register<T: RegisterSource>(
        &self,
        reg: &Reg<T>,
    ) -> Option<TypedSizedRegister<HardwareRegister>> {
        // Todo this could go from Reg to index instead of to_type_registers
        match self.index(reg.reg) {
            RegisterState::Unassigned => None,
            RegisterState::Assigned(hw_reg) => {
                let tp = reg.to_typed_register();
                Some(TypedSizedRegister {
                    reg: hw_reg.reg(),
                    addressing: tp.addressing,
                    idx: tp.idx,
                })
            }
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
    for (line, instruction) in instructions.iter().enumerate().rev() {
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
                panic!("{line}: {instruction:?} does not use the destination")
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
        .enumerate()
        .for_each(|(line, inst)| println!("{line}: {}", inst.format_instruction()));
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

    let mut clobber_registers: BTreeSet<TypedSizedRegister<HardwareRegister>> = BTreeSet::new();
    instructions.iter().for_each(|instruction| {
        clobber_registers.extend(instruction.extract_registers().map(|reg| clobber(reg)));
    });

    let output_registers = BTreeSet::from_iter(output_registers.to_owned());

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
