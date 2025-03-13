use std::{cell::RefCell, collections::VecDeque};

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
    fn add_insts(&mut self, mut s: Vec<String>) {
        self.inst.append(&mut s);
    }
    fn add_inst(&mut self, s: String) {
        self.inst.push(s);
    }
    fn mul(&mut self, dst: &Reg, a: &Reg, b: &Reg) {
        self.add_inst(format!("mul {}, {}, {}", dst, a, b))
    }
    fn umulh(&mut self, dst: &Reg, a: &Reg, b: &Reg) {
        self.add_inst(format!("umulh {}, {}, {}", dst, a, b))
    }
}

// In this algorithm the inputs are not used after
fn smult<'a>(asm: &mut Assembler, alloc: &'a Alloc, a: [Reg; 4], b: Reg) -> [Reg<'a>; 5] {
    // If you want to drop them individually you need to unpack them
    let s = alloc.x_array();
    // In this description you force the temp register to be reused without giving it a new name
    let tmp = alloc.x();
    asm.mul(&s[0], &a[0], &b);
    asm.umulh(&s[1], &a[0], &b);
    let insts = vec![
        //
        format!("mul {}, {}, {}", tmp, a[1], b),
        format!("umulh {}, {}, {}", s[2], a[1], b),
        format!("adds {},{},{}", s[1], s[1], tmp),
        //
        format!("mul {}, {}, {}", tmp, a[2], b),
        format!("umulh {}, {}, {}", s[3], a[2], b),
        format!("adcs {},{},{}", s[2], s[2], tmp),
        //
        format!("mul {}, {}, {}", tmp, a[3], b),
        format!("umulh {}, {}, {}", s[4], a[3], b),
        format!("adcs {},{},{}", s[3], s[3], tmp),
        format!("cinc {}, hs", s[4]),
    ];
    asm.add_insts(insts);
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
