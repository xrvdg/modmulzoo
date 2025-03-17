#![feature(never_type)]
#![feature(concat_idents)]
use std::{
    array,
    collections::{BTreeSet, HashSet, VecDeque},
    mem,
};

// FreshReg can be copied around, but should not be accessible from
// the user. You do that by not exposing BaseInstr
// but how can the user then add their own instructions?

type FreshReg = u64;

// Vec<BlockInstr> - mixing -> Vec<Instr> -> Vec<InstrDrop> -> Vec<PhysInstr>
// Naming convention here is very confusing
// Instr models a single instruction
// FreshInstr models atomic blocks of instructions
type Instr = BaseInstr<!>;
type AtomicInstr = Vec<Instr>;
type InstrDrop = BaseInstr<FreshReg>;

// Instruction needs to know the difference between physical and vector
// because drop won't be able to tell which is which otherwise
/// BaseInstruction allows for s
#[derive(Debug)]
enum BaseInstr<D> {
    XInst1(String, FreshReg, String /* condition */),
    VInst2(String, FreshReg, FreshReg),
    XInst3(String, FreshReg, FreshReg, FreshReg),
    Drop(D),
}
// Define a macro for generating assembler instruction methods
// Don't write directly to the assembler as we would like to use these to construct grouped instructions
macro_rules! embed_asm {
    // For instructions with 3 register parameters
    ($name:ident, 3) => {
        fn $name(dst: &XReg, a: &XReg, b: &XReg) -> crate::AtomicInstr {
            vec![crate::BaseInstr::XInst3(
                stringify!($name).to_string(),
                (dst.reg),
                (a.reg),
                (b.reg),
            )]
        }
    };

    ($name:ident, $inst:literal, 2) => {
        fn $name(dst: &VReg, src: &VReg) -> crate::AtomicInstr {
            vec![crate::BaseInstr::VInst2(
                stringify!($inst).to_string(),
                dst.reg,
                src.reg,
            )]
        }
    };

    // For instructions with 1 register and 1 string parameter (cinc)
    ($name:ident, cond) => {
        fn $name(dst: &XReg, condition: &str) -> crate::AtomicInstr {
            vec![crate::BaseInstr::XInst1(
                stringify!($name).to_string(),
                dst.reg,
                condition.to_string(),
            )]
        }
    };
}

embed_asm!(mul, 3);
embed_asm!(umulh, 3);
embed_asm!(adds, 3);
embed_asm!(adcs, 3);
embed_asm!(cinc, cond);
embed_asm!(ucvtf2d, "ucvtf.2d", 2);

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
    fn reg(&self) -> FreshReg;
}

impl Reg for XReg {
    fn reg(&self) -> FreshReg {
        self.reg
    }
}

impl Reg for VReg {
    fn reg(&self) -> FreshReg {
        self.reg
    }
}

// Put both inside Assembler as I couldn't give a reference to Reg
// if fresh was &mut self. But give it another try
#[derive(Debug)]
struct Assembler {
    // It's about unique counters so we use the counter for both
    // q and v registers
    // this makes it easier to read the assembly
    fresh: u64,
    inst: Vec<AtomicInstr>,
}

impl Assembler {
    fn fresh(&mut self) -> XReg {
        let x = self.fresh;
        self.fresh += 1;
        XReg { reg: x }
    }

    fn freshv(&mut self) -> VReg {
        let x = self.fresh;
        self.fresh += 1;
        VReg { reg: x }
    }

    fn new() -> Self {
        Self::start_from(0)
    }

    fn start_from(n: u64) -> Self {
        Self {
            fresh: n,
            inst: Vec::new(),
        }
    }
}

// Macro for not having to do method chaining
macro_rules! asm_op {
    ($asm:ident, $($method:ident($($arg:expr),*));+) => {
        $(
            $asm.inst.push(vec![$method($($arg),*)]);
        )+
    };
}

// TODO Downside of the change is that builtin and own written now have a different feel to it
// Maybe that should be brought together later

// How do other allocating algorithms pass things along like Vec?
// In this algorithm the inputs are not used after
fn smult(asm: &mut Assembler, s: &[XReg; 5], a: [XReg; 4], b: XReg) -> Vec<AtomicInstr> {
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
fn carry_add(s: [&XReg; 2], add: &XReg) -> AtomicInstr {
    vec![adds(&s[0], &s[0], &add), cinc(&s[1], "hs")]
        .into_iter()
        .flatten()
        .collect()
}

fn smult_noinit_simd(t: &[VReg; 6], s: VReg, v: [XReg; 5]) -> Vec<AtomicInstr> {
    vec![ucvtf2d(&s, &s)]
}

impl std::fmt::Display for XReg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x{}", self.reg)
    }
}

impl<'a> std::fmt::Debug for XReg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x{}", self.reg)
    }
}

// Add another struct to prevent things from being created
// Make a struct around here such that it can't be copied
// THe phys_register file is the one that creates them
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct PhysicalReg(u64);

// No Clone as the state of one free reg
// does not make sense as the state of another free reg
#[derive(PartialEq, Debug)]
enum RegState {
    Unassigned,
    Map(PhysicalReg),
    Dropped,
}

// Both Reg and PhysicalReg are not supposed to be copied.
// BUt for the interface we do need to map them some way
// This can also be done as part of the initialisation
// A way out of the ordering for now is to just make it a big enough size
fn input(
    asm: &mut Assembler,
    mapping: &mut RegisterMapping,
    phys_registers: &mut RegisterBank,
    phys: PhysicalReg,
) -> XReg {
    let fresh = asm.fresh();
    if !phys_registers.q.remove(&phys) {
        panic!("Register q{} is already in use", phys.0)
    }
    mapping[fresh.reg()] = RegState::Map(phys);
    fresh
}

type Seen = HashSet<FreshReg>;

fn output_interface(seen: &mut Seen, fresh: impl Reg) {
    seen.insert(fresh.reg());
}

struct RegisterBank {
    q: BTreeSet<PhysicalReg>,
}

impl RegisterBank {
    fn new() -> Self {
        Self {
            q: BTreeSet::from_iter(Vec::from_iter(0..=30).iter().map(|&r| PhysicalReg(r))),
        }
    }
}

fn main() {
    // If the allocator reaches then it needs to start saving
    // that can be done in a separate pass in front and in the back
    // doesn't fully do the indirect result register
    let mut asm = Assembler::new();
    let mut mapping = RegisterMapping::new();
    let mut phys_registers = RegisterBank::new();

    // Map how the element are mapped to physical registers
    // This needs in to be in part of the code that can talk about physical registers
    // Could structure this differently such that it gives a fresh reg
    let b = input(&mut asm, &mut mapping, &mut phys_registers, PhysicalReg(0));
    let a_regs = array::from_fn(|ai| PhysicalReg(1 + ai as u64));
    let a = a_regs.map(|pr| input(&mut asm, &mut mapping, &mut phys_registers, pr));

    let s: [XReg; 5] = array::from_fn(|_| asm.fresh());

    let sinst = smult(&mut asm, &s, a, b);
    println!("{:?}", asm);

    let old = sinst;

    let mut asm = Assembler::start_from(asm.fresh);

    let b = input(&mut asm, &mut mapping, &mut phys_registers, PhysicalReg(5));
    let a_regs = array::from_fn(|ai| PhysicalReg(6 + ai as u64));
    let a = a_regs.map(|pr| input(&mut asm, &mut mapping, &mut phys_registers, pr));
    let p: [XReg; 5] = array::from_fn(|_| asm.fresh());
    let p_inst = smult(&mut asm, &p, a, b);
    let new = p_inst;

    let mix = old
        .into_iter()
        .zip(new.into_iter())
        .flat_map(|(a, b)| [a, b])
        .flatten()
        .collect::<Vec<_>>();

    // Is there something we can do to tie off the outputs.
    // and to make sure it happens before drop_pass
    let mut seen = HashSet::new();
    s.into_iter().for_each(|r| output_interface(&mut seen, r));
    p.into_iter().for_each(|r| output_interface(&mut seen, r));
    let mix = drop_pass(&mut seen, mix);
    println!("\nmix: {mix:?}");

    // Fix this up later
    // let size = asm.0.borrow().fresh;
    // This can be an array doesn't need to be resizable, but also no benefits to not doing it.

    // Mapping and phys_registers seem to go togetehr
    let out = generate(&mut mapping, &mut phys_registers, mix);
    println!("{out:?}")

    // Nicer debug output would be to take the predrop instruction list and zip it with the output
    // A next step would be to keep the label
}

struct RegisterMapping(Vec<RegState>);

impl RegisterMapping {
    fn new() -> Self {
        Self(
            std::iter::repeat_with(|| RegState::Unassigned)
                .take(30)
                .collect::<Vec<_>>(),
        )
    }
}

impl std::ops::Index<FreshReg> for RegisterMapping {
    type Output = RegState;

    fn index(&self, idx: FreshReg) -> &Self::Output {
        &self.0[idx as usize]
    }
}

impl std::ops::IndexMut<FreshReg> for RegisterMapping {
    fn index_mut(&mut self, idx: FreshReg) -> &mut Self::Output {
        &mut self.0[idx as usize]
    }
}

fn generate(
    mapping: &mut RegisterMapping,
    phys_registers: &mut RegisterBank,
    instructions: VecDeque<InstrDrop>,
) -> Vec<String> {
    let mut out = Vec::new();
    for inst in instructions {
        match inst {
            BaseInstr::XInst1(inst, a, cond) => {
                // Here the dst is also the source
                let phys_reg = lookup_phys_reg_src(mapping, a);
                out.push(format!("{inst} x{phys_reg},{cond}"));
            }
            BaseInstr::VInst2(inst, a, b) => {
                let phys_reg_src = lookup_phys_reg_dst(mapping, phys_registers, a);
                let phys_reg_b = lookup_phys_reg_src(mapping, b);
                out.push(format!("{inst} v{phys_reg_src}, v{phys_reg_b}"));
            }
            BaseInstr::XInst3(inst, a, b, c) => {
                let phys_reg_src = lookup_phys_reg_dst(mapping, phys_registers, a);
                let phys_reg_b = lookup_phys_reg_src(mapping, b);
                let phys_reg_c = lookup_phys_reg_src(mapping, c);
                out.push(format!(
                    "{inst} x{phys_reg_src}, x{phys_reg_b}, x{phys_reg_c}"
                ));
            }
            BaseInstr::Drop(fresh) => {
                let old = mem::replace(&mut mapping[fresh], RegState::Dropped);
                match old {
                    RegState::Unassigned => unreachable!(
                        "There should never be a drop before the register has been assigned"
                    ),
                    RegState::Map(phys_reg) => phys_registers.q.insert(phys_reg),
                    RegState::Dropped => {
                        unreachable!("A register that has been dropped can't be dropped again")
                    }
                };
            }
        }
    }
    out
}

fn convert_inst(inst: Instr) -> InstrDrop {
    match inst {
        BaseInstr::XInst1(a, b, c) => BaseInstr::XInst1(a, b, c),
        BaseInstr::VInst2(a, b, c) => BaseInstr::VInst2(a, b, c),
        BaseInstr::XInst3(a, b, c, d) => BaseInstr::XInst3(a, b, c, d),
    }
}

fn drop_pass(seen: &mut Seen, insts: Vec<Instr>) -> VecDeque<InstrDrop> {
    // Can already calculate the size it's the amount of registers + the amount of free variables.
    // So we can just do it on a vector
    // We can preallocate
    // We do have that knowledge
    let mut dinsts = VecDeque::new();
    for inst in insts.into_iter().rev() {
        match inst {
            BaseInstr::XInst1(_, r, _) => {
                if seen.insert(r) {
                    dinsts.push_front(InstrDrop::Drop(r));
                }
            }
            BaseInstr::VInst2(_, r0, r1) => {
                if seen.insert(r0) {
                    dinsts.push_front(InstrDrop::Drop(r0));
                }
                if seen.insert(r1) {
                    dinsts.push_front(InstrDrop::Drop(r1));
                }
            }
            BaseInstr::XInst3(_, r0, r1, r2) => {
                if seen.insert(r0) {
                    dinsts.push_front(InstrDrop::Drop(r0));
                }
                if seen.insert(r1) {
                    dinsts.push_front(InstrDrop::Drop(r1));
                }
                if seen.insert(r2) {
                    dinsts.push_front(InstrDrop::Drop(r2));
                }
            }
        }
        dinsts.push_front(convert_inst(inst));
    }
    dinsts
}

fn lookup_phys_reg_src(mapping: &mut RegisterMapping, fresh: FreshReg) -> u64 {
    // Should be an Entry way of doing this
    let phys_reg = match &mapping[fresh] {
        RegState::Unassigned => unreachable!("{fresh:?} has not been assigned yet"),
        RegState::Map(reg) => reg.0,
        RegState::Dropped => unreachable!("{fresh:?} already has been dropped"),
    };
    phys_reg
}

fn lookup_phys_reg_dst(
    // Single mapping or double mapping
    mapping: &mut RegisterMapping,
    phys_registers: &mut RegisterBank,
    fresh: FreshReg,
) -> u64 {
    // Should be an Entry way of doing this
    let phys_reg = match &mapping[fresh] {
        RegState::Unassigned => {
            // Todo switchover to second set
            let reg = phys_registers.q.pop_first().expect("ran out of registers");
            let regnr = reg.0;
            mapping[fresh] = RegState::Map(reg);
            regnr
        }
        RegState::Map(reg) => reg.0,
        RegState::Dropped => unreachable!("{fresh:?} already has been dropped"),
    };
    phys_reg
}
