use std::{
    array,
    cell::RefCell,
    collections::{HashSet, VecDeque},
};

#[derive(Debug)]
// The string can be replaced by an Opcode
// It could then possible be fully replaced by the Opcodes
// -> not directly as the distinction between FreshReg and Reg is still important.
// But it looks like there is a way for more simplification
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

type FreshReg = u64;

struct Reg<'a> {
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

    // This
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
    let s = [
        asm.fresh(),
        asm.fresh(),
        asm.fresh(),
        asm.fresh(),
        asm.fresh(),
    ];
    // tmp is now being reused instead of getting a fresh register each time
    let tmp = asm.fresh();
    asm_op!(asm,
        mul(&s[0], &a[0], &b);
        umulh(&s[1], &a[0], &b);

        //Replace formatted string instructions with method calls
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

type PhysicalReg = u64;

// No Clone as the state of one free reg
// does not make sense as the state of another free reg
#[derive(PartialEq, Debug)]
enum RegState {
    Unassigned,
    Map(PhysicalReg),
    Dropped,
}

fn main() {
    // If the allocator reaches then it needs to start saving
    // that can be done in a separate pass in front and in the back
    // doesn't fully do the indirect result register
    let asm = RefAssembler::new();
    let s = smult(&asm, array::from_fn(|_| asm.fresh()), asm.fresh());
    println!("{:?}", asm);

    // Take out the instructions such that the free registers counter stays.
    // Another option is to instantiate a new RefAssembler with a higher number. Both are valid
    let old = std::mem::replace(&mut asm.0.borrow_mut().inst, Vec::new());

    let p = smult(&asm, array::from_fn(|_| asm.fresh()), asm.fresh());
    let new = std::mem::replace(&mut asm.0.borrow_mut().inst, Vec::new());

    let mix = old
        .into_iter()
        .zip(new.into_iter())
        .flat_map(|(a, b)| [a, b])
        .collect::<Vec<_>>();

    let size = asm.0.borrow().fresh;
    // This can be an array doesn't need to be resizable, but also no benefits to not doing it.
    let mut mapping = std::iter::repeat_with(|| RegState::Unassigned)
        .take(size as usize)
        .collect::<Vec<_>>();

    let mut phys_registers = VecDeque::from_iter(0..=30);
    let mut out = Vec::new();
    for inst in mix {
        match inst {
            // The instructions will not be different for the most part so is this distinction useful?
            Instr::Inst1(inst, a, cond) => {
                let phys_reg = lookup_phys_reg(&mut mapping, &mut phys_registers, a);
                out.push(format!("{inst} x{phys_reg},{cond}"));
            }
            Instr::Inst3(inst, a, b, c) => {
                let phys_reg_a = lookup_phys_reg(&mut mapping, &mut phys_registers, a);
                let phys_reg_b = lookup_phys_reg(&mut mapping, &mut phys_registers, b);
                let phys_reg_c = lookup_phys_reg(&mut mapping, &mut phys_registers, c);
                out.push(format!(
                    "{inst} x{phys_reg_a}, x{phys_reg_b}, x{phys_reg_c}"
                ));
            }

            Instr::Drop(fresh) => {
                match mapping[fresh as usize] {
                    RegState::Unassigned => unreachable!(
                        "There should never be a drop before the register has been assigned"
                    ),
                    // Decide whether we want to keep the reg numbers low or just rotate through them
                    // Due to register naming on the processor the ordering doesn't matter
                    RegState::Map(phys_reg) => phys_registers.push_front(phys_reg),
                    RegState::Dropped => {
                        unreachable!("A register that has been dropped can't be dropped again")
                    }
                }
                mapping[fresh as usize] = RegState::Dropped
            }
        }
    }
    println!("{out:?}")
}

fn lookup_phys_reg(
    mapping: &mut Vec<RegState>,
    phys_registers: &mut VecDeque<u64>,
    fresh: u64,
) -> u64 {
    // Should be an Entry way of doing this
    let phys_reg = match mapping[fresh as usize] {
        RegState::Unassigned => {
            let reg = phys_registers.pop_front().expect("ran out of registers");
            mapping[fresh as usize] = RegState::Map(reg);
            reg
        }
        RegState::Map(reg) => reg,
        RegState::Dropped => unreachable!("Something went wrong"),
    };
    phys_reg
}
