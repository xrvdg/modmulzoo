use std::{cell::RefCell, collections::VecDeque};

// Define a macro for generating assembler instruction methods
macro_rules! embed_asm {
    // For instructions with 3 register parameters
    ($name:ident, 3regs) => {
        fn $name(&mut self, dst: &Reg, a: &Reg, b: &Reg) {
            self.add_inst(format!(
                concat!(stringify!($name), " {}, {}, {}"),
                dst, a, b
            ))
        }
    };

    // For instructions with 1 register and 1 string parameter (cinc)
    ($name:ident, 1reg_1str) => {
        fn $name(&mut self, dst: &Reg, condition: &str) {
            self.add_inst(format!(
                concat!(stringify!($name), " {}, {}"),
                dst, condition
            ))
        }
    };
}

struct Reg<'a> {
    reg: u8,
    fresh: &'a RefCell<VecDeque<u8>>,
}

struct Assembler {
    inst: Vec<String>,
}

struct Alloc(RefCell<VecDeque<u8>>);

impl Alloc {
    fn x<'a>(&'a self) -> Reg<'a> {
        let reg = self
            .0
            .borrow_mut()
            .pop_front()
            .expect("No X registers available");

        Reg {
            reg,
            fresh: &self.0,
        }
    }
    fn x_array<'a, const N: usize>(&'a self) -> [Reg<'a>; N] {
        std::array::from_fn(|_| self.x())
    }
}

impl Assembler {
    fn add_inst(&mut self, s: String) {
        self.inst.push(s);
    }

    // Use the macro to define instruction methods
    embed_asm!(mul, 3regs);
    embed_asm!(umulh, 3regs);
    embed_asm!(adds, 3regs);
    embed_asm!(adcs, 3regs);
    embed_asm!(cinc, 1reg_1str);
}

// In this algorithm the inputs are not used after
fn smult<'a>(asm: &mut Assembler, alloc: &'a Alloc, a: [Reg; 4], b: Reg) -> [Reg<'a>; 5] {
    // If you want to drop them individually you need to unpack them
    let s = alloc.x_array();
    // In this description you force the temp register to be reused without giving it a new name
    let tmp = alloc.x();
    asm.mul(&s[0], &a[0], &b);
    asm.umulh(&s[1], &a[0], &b);

    // Replace formatted string instructions with method calls
    asm.mul(&tmp, &a[1], &b);
    asm.umulh(&s[2], &a[1], &b);
    asm.adds(&s[1], &s[1], &tmp);

    asm.mul(&tmp, &a[2], &b);
    asm.umulh(&s[3], &a[2], &b);
    asm.adcs(&s[2], &s[2], &tmp);

    asm.mul(&tmp, &a[3], &b);
    asm.umulh(&s[4], &a[3], &b);
    asm.adcs(&s[3], &s[3], &tmp);
    asm.cinc(&s[4], "hs");

    s
}

impl<'a> Drop for Reg<'a> {
    fn drop(&mut self) {
        // println!("Dropping x{}", self.reg);
        self.fresh.borrow_mut().push_front(self.reg);
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
    let alloc = Alloc(RefCell::new(VecDeque::from_iter(0..32)));
    let inst = Vec::new();
    let mut asm = Assembler { inst };
    let s = smult(&mut asm, &alloc, alloc.x_array(), alloc.x());
    println!("{:?}", s);
    println!("{:?}", asm.inst);
}
