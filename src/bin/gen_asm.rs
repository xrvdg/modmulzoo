use std::{
    cell::RefCell,
    collections::{HashSet, VecDeque},
};

#[derive(Debug)]
enum Instr {
    Inst1(String, FreshReg, String /* condition */),
    Inst3(String, FreshReg, FreshReg, FreshReg),
    Drop(u64),
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
        // println!("Dropping x{}", self.reg);
        self.asm.0.borrow_mut().inst.push(Instr::Drop(self.reg));
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

fn main() {
    // If the allocator reaches then it needs to start saving
    // that can be done in a separate pass in front and in the back
    // doesn't fully do the indirect result register
    let asm = RefAssembler::new();
    let s = smult(
        &asm,
        [asm.fresh(), asm.fresh(), asm.fresh(), asm.fresh()],
        asm.fresh(),
    );
    {
        let s = s;
        println!("{:?}", s);
    }
    println!("{:?}", asm);
}
