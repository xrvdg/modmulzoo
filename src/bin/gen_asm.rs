use std::{
    array,
    collections::{BTreeSet, HashSet, VecDeque},
    mem,
};

use montgomery_reduction::emmart;

// FreshReg can be copied around, but should not be accessible from
// the user. You do that by not exposing BaseInstr
// but how can the user then add their own instructions?

type FreshReg = u64;

// Vec<BlockInstr> - mixing -> Vec<Instr> -> Vec<InstrDrop> -> Vec<PhysInstr>
// Naming convention here is very confusing
// Instr models a single instruction
// FreshInstr models atomic blocks of instructions
type AtomicInstr = Vec<Instr>;

// Instruction needs to know the difference between physical and vector
// because drop won't be able to tell which is which otherwise
/// BaseInstruction allows for s
#[derive(Debug)]
enum Instr {
    XInst1(String, FreshReg, u64),
    XInst2Cond(String, FreshReg, FreshReg, String /* condition */),
    VInst2(String, FreshReg, FreshReg),
    DXInst2(String, FreshReg, FreshReg),
    VXInst2(String, FreshReg, FreshReg),
    XInst3(String, FreshReg, FreshReg, FreshReg),
    VInst3I(String, FreshReg, FreshReg, FreshReg, u8),
}

impl Instr {
    // Format instruction using already resolved physical register numbers
    fn format_instruction(&self, phys_regs: &[u64]) -> String {
        match self {
            Instr::XInst2Cond(inst, _, _, cond) => {
                format!("{inst} x{}, x{},{cond}", phys_regs[0], phys_regs[1])
            }
            Instr::XInst1(inst, _, val) => {
                format!("{inst} x{}, #{val}", phys_regs[0])
            }
            Instr::VXInst2(inst, _, _) => {
                format!("{inst} v{}, x{}", phys_regs[0], phys_regs[1])
            }
            Instr::DXInst2(inst, _, _) => {
                format!("{inst} d{}, x{}", phys_regs[0], phys_regs[1])
            }
            Instr::VInst2(inst, _, _) => {
                format!("{inst} v{}, v{}", phys_regs[0], phys_regs[1])
            }
            Instr::XInst3(inst, _, _, _) => {
                format!(
                    "{inst} x{}, x{}, x{}",
                    phys_regs[0], phys_regs[1], phys_regs[2]
                )
            }
            Instr::VInst3I(inst, _, _, _, idx) => {
                format!(
                    "{inst} v{}, v{}, v{}[{idx}]",
                    phys_regs[0], phys_regs[1], phys_regs[2]
                )
            }
        }
    }
}

#[derive(Debug)]
enum InstrDrop {
    Instr(Instr),
    Drop(FreshReg),
}

impl From<Instr> for InstrDrop {
    fn from(instr: Instr) -> Self {
        InstrDrop::Instr(instr)
    }
}

// Define a macro for generating assembler instruction methods
// Don't write directly to the assembler as we would like to use these to construct grouped instructions
macro_rules! embed_asm {
    // For instructions with 3 register parameters
    ($name:ident, 3) => {
        fn $name(dst: &XReg, a: &XReg, b: &XReg) -> crate::AtomicInstr {
            vec![crate::Instr::XInst3(
                stringify!($name).to_string(),
                (dst.reg),
                (a.reg),
                (b.reg),
            )]
        }
    };

    ($name:ident, $inst:literal, 3) => {
        fn $name(dst: &VReg, src_a: &VReg, src_b: &VReg, i: u8) -> crate::AtomicInstr {
            vec![crate::Instr::VInst3I(
                $inst.to_string(),
                dst.reg,
                src_a.reg,
                src_b.reg,
                i,
            )]
        }
    };

    ($name:ident, $inst:literal, 2) => {
        fn $name(dst: &VReg, src: &VReg) -> crate::AtomicInstr {
            vec![crate::Instr::VInst2($inst.to_string(), dst.reg, src.reg)]
        }
    };

    ($name:ident, $inst:literal, 2, m) => {
        fn $name(dst: &VReg, src: &XReg) -> crate::AtomicInstr {
            vec![crate::Instr::VXInst2($inst.to_string(), dst.reg, src.reg)]
        }
    };

    ($name:ident, 2, m) => {
        fn $name(dst: &VReg, src: &XReg) -> crate::AtomicInstr {
            vec![crate::Instr::DXInst2(
                stringify!($name).to_string(),
                dst.reg,
                src.reg,
            )]
        }
    };

    ($name:ident, 1) => {
        fn $name(dst: &XReg, val: u64) -> crate::AtomicInstr {
            vec![crate::Instr::XInst1(
                stringify!($name).to_string(),
                dst.reg,
                val,
            )]
        }
    };

    // For instructions with 1 register and 1 string parameter (cinc)
    ($name:ident, cond) => {
        fn $name(dst: &XReg, src: &XReg, condition: &str) -> crate::AtomicInstr {
            vec![crate::Instr::XInst2Cond(
                stringify!($name).to_string(),
                dst.reg,
                src.reg,
                condition.to_string(),
            )]
        }
    };
}

embed_asm!(mov, 1);
embed_asm!(mul, 3);
embed_asm!(umulh, 3);
embed_asm!(adds, 3);
embed_asm!(adcs, 3);
embed_asm!(cinc, cond);
// mov now doesn't support immediates. Not sure if mov16 actually ever can
embed_asm!(mov16b, "mov.16b", 2);
embed_asm!(ucvtf2d, "ucvtf.2d", 2);
embed_asm!(dup2d, "dup.2d", 2, m);
// Could use another but this works too
embed_asm!(ucvtf, 2, m);
embed_asm!(fmla2d, "fmla.2d", 3);

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
    fn new(reg: FreshReg) -> Self;
    fn reg(&self) -> FreshReg;
}

impl Reg for XReg {
    fn reg(&self) -> FreshReg {
        self.reg
    }

    fn new(reg: FreshReg) -> Self {
        Self { reg }
    }
}

impl Reg for VReg {
    fn new(reg: FreshReg) -> Self {
        Self { reg }
    }
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
    fn fresh<T: Reg>(&mut self) -> T {
        let x = self.fresh;
        self.fresh += 1;
        T::new(x)
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
    vec![adds(&s[0], &s[0], &add), cinc(&s[1], &s[1], "hs")]
        .into_iter()
        .flatten()
        .collect()
}

// Whole vector is in registers, but that might not be great. Better to have it on the stack and load it from there
fn smult_noinit_simd(
    asm: &mut Assembler,
    t: &[VReg; 6],
    s: VReg,
    v: [XReg; 5],
) -> Vec<AtomicInstr> {
    // first do it as is written
    let tmp = asm.fresh();
    let splat_c1 = asm.fresh();
    let cc1 = asm.fresh();
    let fv0 = asm.fresh();
    vec![
        ucvtf2d(&s, &s),
        mov(&tmp, emmart::C1.to_bits()),
        ucvtf(&fv0, &v[0]),
        dup2d(&splat_c1, &tmp),
        mov16b(&cc1, &splat_c1),
        fmla2d(&cc1, &s, &fv0, 0),
    ]
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
    XMap(PhysicalReg),
    VMap(PhysicalReg),
    Dropped,
}

// Both Reg and PhysicalReg are not supposed to be copied.
// BUt for the interface we do need to map them some way
// This can also be done as part of the initialisation
// A way out of the ordering for now is to just make it a big enough size
fn inputx(
    asm: &mut Assembler,
    mapping: &mut RegisterMapping,
    phys_registers: &mut RegisterBank,
    phys: u64,
) -> XReg {
    let fresh: XReg = asm.fresh();
    let phys = PhysicalReg(phys);
    if !phys_registers.x.remove(&phys) {
        panic!("Register q{} is already in use", phys.0)
    }
    mapping[fresh.reg()] = RegState::XMap(phys);
    fresh
}

fn inputv(
    asm: &mut Assembler,
    mapping: &mut RegisterMapping,
    phys_registers: &mut RegisterBank,
    phys: u64,
) -> VReg {
    let fresh: VReg = asm.fresh();
    let phys = PhysicalReg(phys);
    if !phys_registers.v.remove(&phys) {
        panic!("Register v{} is already in use", phys.0)
    }
    mapping[fresh.reg()] = RegState::VMap(phys);
    fresh
}

type Seen = HashSet<FreshReg>;

fn output_interface(seen: &mut Seen, fresh: impl Reg) {
    seen.insert(fresh.reg());
}

// TODO(xrvdg) Different types for the PhysicalRegs
struct RegisterBank {
    x: BTreeSet<PhysicalReg>,
    v: BTreeSet<PhysicalReg>,
}

impl RegisterBank {
    fn new() -> Self {
        Self {
            x: BTreeSet::from_iter(Vec::from_iter(0..=30).iter().map(|&r| PhysicalReg(r))),
            v: BTreeSet::from_iter(Vec::from_iter(0..=30).iter().map(|&r| PhysicalReg(r))),
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
    let b = inputx(&mut asm, &mut mapping, &mut phys_registers, PhysicalReg(0));
    let a_regs = array::from_fn(|ai| PhysicalReg(1 + ai as u64));
    let a = a_regs.map(|pr| inputx(&mut asm, &mut mapping, &mut phys_registers, pr));

    let s: [XReg; 5] = array::from_fn(|_| asm.fresh());

    let sinst = smult(&mut asm, &s, a, b);
    println!("{:?}", asm);

    let old = sinst;

    let mut asm = Assembler::start_from(asm.fresh);

    let b = inputx(&mut asm, &mut mapping, &mut phys_registers, PhysicalReg(5));
    let a_regs = array::from_fn(|ai| PhysicalReg(6 + ai as u64));
    let a = a_regs.map(|pr| inputx(&mut asm, &mut mapping, &mut phys_registers, pr));
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
    println!("{out:?}");

    let mut asm = Assembler::new();
    let mut mapping = RegisterMapping::new();
    let mut phys_registers = RegisterBank::new();

    let t_regs = array::from_fn(|ai| PhysicalReg(ai as u64));
    let t = t_regs.map(|pr| inputv(&mut asm, &mut mapping, &mut phys_registers, pr));
    let v_regs = array::from_fn(|ai| PhysicalReg(ai as u64));
    let v = v_regs.map(|pr| inputx(&mut asm, &mut mapping, &mut phys_registers, pr));
    let s = inputv(
        &mut asm,
        &mut mapping,
        &mut phys_registers,
        PhysicalReg(t.len() as u64),
    );
    let ssimd = smult_noinit_simd(&mut asm, &t, s, v);
    println!("ssimd");
    println!("{:?}", ssimd);

    let mut seen = HashSet::new();
    t.into_iter().for_each(|r| output_interface(&mut seen, r));
    let out = generate(
        &mut mapping,
        &mut phys_registers,
        drop_pass(&mut seen, ssimd.into_iter().flatten().collect()),
    );
    println!("\nmix: {out:?}");

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

    // Get the physical register for a source register
    fn get_phys_reg(&self, fresh: FreshReg) -> u64 {
        match &self[fresh] {
            RegState::Unassigned => unreachable!("{fresh:?} has not been assigned yet"),
            RegState::XMap(reg) => reg.0,
            RegState::Dropped => unreachable!("{fresh:?} already has been dropped"),
            RegState::VMap(reg) => reg.0,
        }
    }

    // Get or allocate an X register
    fn get_or_allocate_x_reg(&mut self, register_bank: &mut RegisterBank, fresh: FreshReg) -> u64 {
        match &self[fresh] {
            RegState::Unassigned => {
                let reg = register_bank.x.pop_first().expect("ran out of registers");
                let regnr = reg.0;
                self[fresh] = RegState::XMap(reg);
                regnr
            }
            RegState::XMap(reg) => reg.0,
            RegState::Dropped => unreachable!("{fresh:?} already has been dropped"),
            RegState::VMap(_) => unreachable!("Looking for X got V"),
        }
    }

    // Get or allocate a V register
    fn get_or_allocate_v_reg(&mut self, register_bank: &mut RegisterBank, fresh: FreshReg) -> u64 {
        match &self[fresh] {
            RegState::Unassigned => {
                let reg = register_bank.v.pop_first().expect("ran out of registers");
                let regnr = reg.0;
                self[fresh] = RegState::VMap(reg);
                regnr
            }
            RegState::VMap(reg) => reg.0,
            RegState::Dropped => unreachable!("{fresh:?} already has been dropped"),
            RegState::XMap(_) => unreachable!("Looking for X got V"),
        }
    }

    // Drop a register and return it to the register bank
    fn drop_register(&mut self, register_bank: &mut RegisterBank, fresh: FreshReg) -> bool {
        let old = mem::replace(&mut self[fresh], RegState::Dropped);
        match old {
            RegState::Unassigned => {
                unreachable!("There should never be a drop before the register has been assigned")
            }
            RegState::XMap(phys_reg) => register_bank.x.insert(phys_reg),
            RegState::Dropped => {
                unreachable!("A register that has been dropped can't be dropped again")
            }
            RegState::VMap(physical_reg) => register_bank.v.insert(physical_reg),
        }
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

fn drop_pass(seen: &mut Seen, insts: Vec<Instr>) -> VecDeque<InstrDrop> {
    let mut dinsts = VecDeque::new();
    for inst in insts.into_iter().rev() {
        let registers = extract_regs(&inst);
        for reg in registers {
            if seen.insert(reg) {
                dinsts.push_front(InstrDrop::Drop(reg));
            }
        }
        dinsts.push_front(inst.into());
    }
    dinsts
}

fn extract_regs(inst: &Instr) -> Vec<FreshReg> {
    match inst {
        Instr::XInst1(_, r, _) => vec![*r],
        Instr::XInst2Cond(_, r0, r1, _) => vec![*r0, *r1],
        Instr::VInst2(_, r0, r1) => vec![*r0, *r1],
        Instr::VXInst2(_, r0, r1) => vec![*r0, *r1],
        Instr::DXInst2(_, r0, r1) => vec![*r0, *r1],
        Instr::XInst3(_, r0, r1, r2) => vec![*r0, *r1, *r2],
        Instr::VInst3I(_, r0, r1, r2, _) => vec![*r0, *r1, *r2],
    }
}

fn generate(
    mapping: &mut RegisterMapping,
    register_bank: &mut RegisterBank,
    instructions: VecDeque<InstrDrop>,
) -> Vec<String> {
    let mut out = Vec::new();
    for instdrop in instructions {
        match instdrop {
            InstrDrop::Instr(inst) => {
                // Resolve registers first - just the physical register numbers
                let phys_regs = match &inst {
                    Instr::XInst2Cond(_, a, b, _) => {
                        let dst = mapping.get_or_allocate_x_reg(register_bank, *a);
                        let src = mapping.get_phys_reg(*b);
                        vec![dst, src]
                    }
                    Instr::XInst1(_, a, _) => {
                        let phys_reg = mapping.get_or_allocate_x_reg(register_bank, *a);
                        vec![phys_reg]
                    }
                    Instr::VXInst2(_, a, b) => {
                        let phys_reg_src = mapping.get_or_allocate_v_reg(register_bank, *a);
                        let phys_reg_b = mapping.get_phys_reg(*b);
                        vec![phys_reg_src, phys_reg_b]
                    }
                    Instr::DXInst2(_, a, b) => {
                        let phys_reg_src = mapping.get_or_allocate_v_reg(register_bank, *a);
                        let phys_reg_b = mapping.get_phys_reg(*b);
                        vec![phys_reg_src, phys_reg_b]
                    }
                    Instr::VInst2(_, a, b) => {
                        let phys_reg_src = mapping.get_or_allocate_v_reg(register_bank, *a);
                        let phys_reg_b = mapping.get_phys_reg(*b);
                        vec![phys_reg_src, phys_reg_b]
                    }
                    Instr::XInst3(_, a, b, c) => {
                        let phys_reg_src = mapping.get_or_allocate_x_reg(register_bank, *a);
                        let phys_reg_b = mapping.get_phys_reg(*b);
                        let phys_reg_c = mapping.get_phys_reg(*c);
                        vec![phys_reg_src, phys_reg_b, phys_reg_c]
                    }
                    Instr::VInst3I(_, a, b, c, _) => {
                        let phys_reg_src = mapping.get_or_allocate_v_reg(register_bank, *a);
                        let phys_reg_b = mapping.get_phys_reg(*b);
                        let phys_reg_c = mapping.get_phys_reg(*c);
                        vec![phys_reg_src, phys_reg_b, phys_reg_c]
                    }
                };

                // Format using resolved physical register numbers
                out.push(inst.format_instruction(&phys_regs));
            }
            InstrDrop::Drop(fresh) => {
                mapping.drop_register(register_bank, fresh);
            }
        }
    }
    out
}
