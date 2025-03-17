#![feature(never_type)]
use std::{
    array,
    collections::{BTreeSet, HashSet, VecDeque},
    mem,
};

// The string can be replaced by an Opcode
// It could then possible be fully replaced by the Opcodes
// -> not directly as the distinction between FreshReg and Reg is still important.
// But it looks like there is a way for more simplification
// By using OpCode it can be used for both Physical Reg and FreshRegs.
// What is the unhabitable type in Rust?
// Changing the way Inst is done impacts two important things
// can't write an instruction as fmla.2d (but can't any way)
// need to write a formatter for every instruction
// test out with vector register first to see how it should work
type InstrDrop = BaseInstr<FreshReg, FreshReg>;
type FreshInstr = BaseInstr<FreshReg, !>;

#[derive(Debug)]
enum BaseInstr<R, D> {
    Inst1(String, R, String /* condition */),
    Inst3(String, R, R, R),
    Drop(D),
}
// Define a macro for generating assembler instruction methods
macro_rules! embed_asm {
    // For instructions with 3 register parameters
    ($name:ident, 3) => {
        fn $name(&mut self, dst: &Reg, a: &Reg, b: &Reg) {
            self.inst.push(crate::BaseInstr::Inst3(
                stringify!($name).to_string(),
                dst.reg,
                a.reg,
                b.reg,
            ))
        }
    };

    // For instructions with 1 register and 1 string parameter (cinc)
    ($name:ident, cond) => {
        fn $name(&mut self, dst: &Reg, condition: &str) {
            self.inst.push(crate::BaseInstr::Inst1(
                stringify!($name).to_string(),
                dst.reg,
                condition.to_string(),
            ))
        }
    };
}

impl Assembler {
    embed_asm!(mul, 3);
    embed_asm!(umulh, 3);
    embed_asm!(adds, 3);
    embed_asm!(adcs, 3);
    embed_asm!(cinc, cond);
}

type FreshReg = usize;

struct Reg {
    // Maybe make reg a usize instead
    reg: FreshReg,
}

// Put both inside Assembler as I couldn't give a reference to Reg
// if fresh was &mut self. But give it another try
#[derive(Debug)]
struct Assembler {
    fresh: FreshReg,
    inst: Vec<FreshInstr>,
}

impl Assembler {
    fn fresh(&mut self) -> Reg {
        let x = self.fresh;
        self.fresh += 1;
        Reg { reg: x }
    }

    fn new() -> Self {
        Self::start_from(0)
    }

    fn start_from(n: FreshReg) -> Self {
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
            $asm.$method($($arg),*);
        )+
    };
}

// How do other allocating algorithms pass things along like Vec?
// In this algorithm the inputs are not used after
fn smult<'a>(asm: &mut Assembler, a: [Reg; 4], b: Reg) -> [Reg; 5] {
    let s = array::from_fn(|_| asm.fresh());
    // tmp being reused instead of a fresh variable each time.
    // should not make much of a difference
    let tmp = asm.fresh();
    asm_op!(asm,
        mul(&s[0], &a[0], &b);
        umulh(&s[1], &a[0], &b);

        mul(&tmp, &a[1], &b);
        umulh(&s[2], &a[1], &b);
        adds(&s[1], &s[1], &tmp);

        mul(&tmp, &a[2], &b);
        umulh(&s[3], &a[2], &b);
        adcs(&s[2], &s[2], &tmp);

        mul(&tmp, &a[3], &b);
        umulh(&s[4], &a[3], &b);
        adcs(&s[3], &s[3], &tmp);
        cinc(&s[4], "hs")
    );

    // or let them drop here automatically
    // make use of the ownership system
    s
}

impl std::fmt::Display for Reg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x{}", self.reg)
    }
}

impl<'a> std::fmt::Debug for Reg {
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
    mapping: &mut Vec<RegState>,
    phys_registers: &mut BTreeSet<PhysicalReg>,
    phys: PhysicalReg,
) -> Reg {
    let fresh = asm.fresh();
    if !phys_registers.remove(&phys) {
        panic!("Register q{} is already in use", phys.0)
    }
    mapping[fresh.reg as usize] = RegState::Map(phys);
    fresh
}

fn output_interface(seen: &mut HashSet<FreshReg>, fresh: Reg) {
    seen.insert(fresh.reg);
}

fn main() {
    // If the allocator reaches then it needs to start saving
    // that can be done in a separate pass in front and in the back
    // doesn't fully do the indirect result register
    let mut asm = Assembler::new();
    let mut mapping = std::iter::repeat_with(|| RegState::Unassigned)
        .take(30)
        .collect::<Vec<_>>();

    let mut phys_registers =
        BTreeSet::from_iter(Vec::from_iter(0..=30).iter().map(|&r| PhysicalReg(r)));

    // Map how the element are mapped to physical registers
    // This needs in to be in part of the code that can talk about physical registers
    // Could structure this differently such that it gives a fresh reg
    let b = input(&mut asm, &mut mapping, &mut phys_registers, PhysicalReg(0));
    let a_regs = array::from_fn(|ai| PhysicalReg(1 + ai as u64));
    let a = a_regs.map(|pr| input(&mut asm, &mut mapping, &mut phys_registers, pr));

    let s = smult(&mut asm, a, b);
    println!("{:?}", asm);

    let old = asm.inst;

    let mut asm = Assembler::start_from(asm.fresh);

    let b = input(&mut asm, &mut mapping, &mut phys_registers, PhysicalReg(5));
    let a_regs = array::from_fn(|ai| PhysicalReg(6 + ai as u64));
    let a = a_regs.map(|pr| input(&mut asm, &mut mapping, &mut phys_registers, pr));
    let p = smult(&mut asm, a, b);
    let new = asm.inst;

    let mix = old
        .into_iter()
        .zip(new.into_iter())
        .flat_map(|(a, b)| [a, b])
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
    let mut out = Vec::new();
    for inst in mix {
        match inst {
            // The instructions will not be different for the most part so is this distinction useful?
            InstrDrop::Inst1(inst, a, cond) => {
                // Here the dst is also the source
                let phys_reg = lookup_phys_reg_src(&mut mapping, a);
                out.push(format!("{inst} x{phys_reg},{cond}"));
            }
            InstrDrop::Inst3(inst, a, b, c) => {
                let phys_reg_src = lookup_phys_reg_dst(&mut mapping, &mut phys_registers, a);
                let phys_reg_b = lookup_phys_reg_src(&mut mapping, b);
                let phys_reg_c = lookup_phys_reg_src(&mut mapping, c);
                out.push(format!(
                    "{inst} x{phys_reg_src}, x{phys_reg_b}, x{phys_reg_c}"
                ));
            }

            InstrDrop::Drop(fresh) => {
                let old = mem::replace(&mut mapping[fresh], RegState::Dropped);
                match old {
                    RegState::Unassigned => unreachable!(
                        "There should never be a drop before the register has been assigned"
                    ),
                    RegState::Map(phys_reg) => phys_registers.insert(phys_reg),
                    RegState::Dropped => {
                        unreachable!("A register that has been dropped can't be dropped again")
                    }
                };
            }
        }
    }
    println!("{out:?}")
}

fn convert_inst(inst: FreshInstr) -> InstrDrop {
    match inst {
        BaseInstr::Inst1(a, b, c) => BaseInstr::Inst1(a, b, c),
        BaseInstr::Inst3(a, b, c, d) => BaseInstr::Inst3(a, b, c, d),
    }
}

fn drop_pass(seen: &mut HashSet<FreshReg>, insts: Vec<FreshInstr>) -> VecDeque<InstrDrop> {
    // Can already calculate the size it's the amount of registers + the amount of free variables.
    // So we can just do it on a vector
    // We can preallocate
    // We do have that knowledge
    let mut dinsts = VecDeque::new();
    for inst in insts.into_iter().rev() {
        match inst {
            FreshInstr::Inst1(_, r, _) => {
                if seen.insert(r) {
                    dinsts.push_front(InstrDrop::Drop(r));
                }
            }
            FreshInstr::Inst3(_, r0, r1, r2) => {
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

fn lookup_phys_reg_src(mapping: &mut Vec<RegState>, fresh: FreshReg) -> u64 {
    // Should be an Entry way of doing this
    let phys_reg = match &mapping[fresh] {
        RegState::Unassigned => unreachable!("{fresh} has not been assigned yet"),
        RegState::Map(reg) => reg.0,
        RegState::Dropped => unreachable!("{fresh} already has been dropped"),
    };
    phys_reg
}

fn lookup_phys_reg_dst(
    mapping: &mut Vec<RegState>,
    phys_registers: &mut BTreeSet<PhysicalReg>,
    fresh: FreshReg,
) -> u64 {
    // Should be an Entry way of doing this
    let phys_reg = match &mapping[fresh] {
        RegState::Unassigned => {
            // Todo switchover to second set
            let reg = phys_registers.pop_first().expect("ran out of registers");
            let regnr = reg.0;
            mapping[fresh as usize] = RegState::Map(reg);
            regnr
        }
        RegState::Map(reg) => reg.0,
        RegState::Dropped => unreachable!("{fresh} already has been dropped"),
    };
    phys_reg
}
