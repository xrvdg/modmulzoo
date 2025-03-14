use std::{array, cell::RefCell, collections::BTreeSet, mem};

#[derive(Debug)]
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
enum Instr {
    Inst1(String, FreshReg, String /* condition */),
    Inst3(String, FreshReg, FreshReg, FreshReg),
    Drop(FreshReg),
}
// Define a macro for generating assembler instruction methods
macro_rules! embed_asm {
    // For instructions with 3 register parameters
    ($name:ident, 3) => {
        fn $name(&self, dst: &Reg, a: &Reg, b: &Reg) {
            self.0.borrow_mut().inst.push(crate::Instr::Inst3(
                stringify!($name).to_string(),
                dst.reg,
                a.reg,
                b.reg,
            ))
        }
    };

    // For instructions with 1 register and 1 string parameter (cinc)
    ($name:ident, cond) => {
        fn $name(&self, dst: &Reg, condition: &str) {
            self.0.borrow_mut().inst.push(crate::Instr::Inst1(
                stringify!($name).to_string(),
                dst.reg,
                condition.to_string(),
            ))
        }
    };
}

impl RefAssembler {
    embed_asm!(mul, 3);
    embed_asm!(umulh, 3);
    embed_asm!(adds, 3);
    embed_asm!(adcs, 3);
    embed_asm!(cinc, cond);
}

type FreshReg = usize;

struct Reg<'a> {
    // Maybe make reg a usize instead
    reg: FreshReg,
    asm: &'a RefAssembler,
}

// Put both inside Assembler as I couldn't give a reference to Reg
// if fresh was &mut self. But give it another try
#[derive(Debug)]
struct Assembler {
    fresh: FreshReg,
    inst: Vec<Instr>,
}

#[derive(Debug)]
struct RefAssembler(RefCell<Assembler>);

impl RefAssembler {
    fn new() -> Self {
        Self(RefCell::new(Assembler::new()))
    }

    fn fresh(&self) -> Reg {
        let mut asm = self.0.borrow_mut();
        let x = asm.fresh;
        asm.fresh += 1;
        Reg { reg: x, asm: &self }
    }

    // This should only be accessible to Reg and nothing else
    fn drop_register(&self, reg: FreshReg) {
        self.0.borrow_mut().inst.push(Instr::Drop(reg));
    }
}

impl Assembler {
    fn new() -> Self {
        Self {
            fresh: 0,
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
fn smult<'a>(asm: &'a RefAssembler, a: [Reg; 4], b: Reg) -> [Reg<'a>; 5] {
    // If you want to drop them individually you need to unpack them
    let [a0, a1, a2, a3] = a;
    let s = array::from_fn(|_| asm.fresh());
    // tmp being reused instead of a fresh variable each time.
    // should not make much of a difference
    let tmp = asm.fresh();
    asm_op!(asm,
        mul(&s[0], &a0, &b);
        umulh(&s[1], &a0, &b)
    );

    // Drop explicitly like this
    drop(a0);

    {
        // Or by scoping like this
        let a1 = a1;
        asm_op!(asm,
         //Replace formatted string instructions with method calls
                mul(&tmp, &a1, &b);
                umulh(&s[2], &a1, &b);
                adds(&s[1], &s[1], &tmp)
        );
    }

    asm_op!(asm,
        mul(&tmp, &a2, &b);
        umulh(&s[3], &a2, &b);
        adcs(&s[2], &s[2], &tmp);

        mul(&tmp, &a3, &b);
        umulh(&s[4], &a3, &b);
        adcs(&s[3], &s[3], &tmp);
        cinc(&s[4], "hs")
    );

    // or let them drop here automatically
    // make use of the ownership system
    s
}

impl<'a> Drop for Reg<'a> {
    fn drop(&mut self) {
        self.asm.drop_register(self.reg);
    }
}

impl std::fmt::Display for Reg<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x{}", self.reg)
    }
}

impl<'a> std::fmt::Debug for Reg<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x{}", self.reg)
    }
}

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
fn interface(
    mapping: &mut Vec<RegState>,
    phys_registers: &mut BTreeSet<PhysicalReg>,
    fresh: &Reg,
    phys: PhysicalReg,
) {
    if !phys_registers.remove(&phys) {
        panic!("Register q{} is already in use", phys.0)
    }
    mapping[fresh.reg as usize] = RegState::Map(phys)
}

fn main() {
    // If the allocator reaches then it needs to start saving
    // that can be done in a separate pass in front and in the back
    // doesn't fully do the indirect result register
    let asm = RefAssembler::new();
    let a = array::from_fn(|_| asm.fresh());
    let b = asm.fresh();
    let mut mapping = std::iter::repeat_with(|| RegState::Unassigned)
        .take(30)
        .collect::<Vec<_>>();

    let mut phys_registers =
        BTreeSet::from_iter(Vec::from_iter(0..=30).iter().map(|&r| PhysicalReg(r)));

    // Map how the element are mapped to physical registers
    // This needs in to be in part of the code that can talk about physical registers
    interface(&mut mapping, &mut phys_registers, &b, PhysicalReg(0));
    a.iter()
        .zip(1..)
        .for_each(|(ai, r)| interface(&mut mapping, &mut phys_registers, ai, PhysicalReg(r)));

    let s = smult(&asm, a, b);
    println!("{:?}", asm);

    // Take out the instructions such that the free registers counter stays.
    // Another option is to instantiate a new RefAssembler with a higher number. Both are valid
    let old = std::mem::replace(&mut asm.0.borrow_mut().inst, Vec::new());

    // let p = smult(&asm, array::from_fn(|_| asm.fresh()), asm.fresh());
    // let new = std::mem::replace(&mut asm.0.borrow_mut().inst, Vec::new());

    // let mix = old
    //     .into_iter()
    //     .zip(new.into_iter())
    //     .flat_map(|(a, b)| [a, b])
    //     .collect::<Vec<_>>();

    let mix = old;

    // Fix this up later
    // let size = asm.0.borrow().fresh;
    // This can be an array doesn't need to be resizable, but also no benefits to not doing it.

    let mut out = Vec::new();
    for inst in mix {
        match inst {
            // The instructions will not be different for the most part so is this distinction useful?
            Instr::Inst1(inst, a, cond) => {
                // Here the dst is also the source
                let phys_reg = lookup_phys_reg_src(&mut mapping, a);
                out.push(format!("{inst} x{phys_reg},{cond}"));
            }
            Instr::Inst3(inst, a, b, c) => {
                let phys_reg_src = lookup_phys_reg_dst(&mut mapping, &mut phys_registers, a);
                let phys_reg_b = lookup_phys_reg_src(&mut mapping, b);
                let phys_reg_c = lookup_phys_reg_src(&mut mapping, c);
                out.push(format!(
                    "{inst} x{phys_reg_src}, x{phys_reg_b}, x{phys_reg_c}"
                ));
            }

            Instr::Drop(fresh) => {
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
