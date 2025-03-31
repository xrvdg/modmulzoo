use std::arch::{asm, global_asm};

global_asm!(include_str!("../../asm/mulu128.s"));

global_asm!(include_str!("../../asm/global_asm_smul.s"));
global_asm!(include_str!("../../asm/global_asm_schoolmethod.s"));
global_asm!(include_str!("../../asm/global_asm_smul_add.s"));
global_asm!(include_str!("../../asm/global_asm_single_step.s"));

fn main() {
    // let r = call_mulu128(5, 6);
    // println!("r: {r:?}");
    // let r = inline_call_mulu128(5, 6);
    // println!("r: {r:?}");
    // let r = call_smul([1, 2, 3, 4], 5);
    // println!("r: {r:?}");
    // let r = call_schoolmethod([1, 2, 3, 4], [5, 6, 7, 8]);
    let r = call_single_step([1, 2, 3, 4], [5, 6, 7, 8]);
    println!("r: {r:?}");
}

#[inline(never)]
fn call_mulu128(a: u64, b: u64) -> u128 {
    let mut lo: u64;
    let mut hi: u64;
    // For now hard code since it only generated every now and then
    unsafe { asm!("bl _mulu128", in("x0") a, in("x1") b, out("x2") lo, out("x3") hi) };
    (hi as u128) << 64 | lo as u128
}

// This might be the best approach to include it into Rust, but depends on if it destroys the order
#[inline(never)]
fn inline_call_mulu128(a: u64, b: u64) -> u128 {
    let mut lo: u64;
    let mut hi: u64;
    // For now hard code since it only generated every now and then
    unsafe {
        asm!(r#"
    mul x2, x0,x1
    umulh x3, x0, x1
    "#, in("x0") a, in("x1") b, out("x2") lo, out("x3") hi)
    };
    (hi as u128) << 64 | lo as u128
}

#[inline(never)]
fn call_smul(a: [u64; 4], b: u64) -> [u64; 5] {
    let mut out = [0; 5];
    unsafe {
        asm!(
            "bl _smul",
            in("x0") b, in("x1") a[0], in("x2") a[1], in("x3") a[2], in("x4") a[3],
            lateout("x5") out[0], lateout("x6") out[1], lateout("x1") out[2], lateout("x2") out[3], lateout("x0") out[4],
            lateout("lr") _
        )
    };
    out
}

#[inline(never)]
pub fn call_schoolmethod(a: [u64; 4], b: [u64; 4]) -> [u64; 8] {
    let mut out = [0; 8];
    unsafe {
        asm!(
            "bl _schoolmethod",
            in("x0") a[0], in("x1") a[1], in("x2") a[2], in("x3") a[3],
            in("x4") b[0], in("x5") b[1], in("x6") b[2], in("x7") b[3],
            lateout("x8") out[0], lateout("x12") out[1], lateout("x4") out[2], lateout("x5") out[3], lateout("x10") out[4], lateout("x0") out[5], lateout("x1") out[6], lateout("x3") out[7],
            lateout("x2") _, lateout("x6") _, lateout("x7") _, lateout("x9") _, lateout("x11") _, lateout("x13") _, lateout("x14") _,
            lateout("lr") _
        )
    };
    out
}

#[inline(never)]
pub fn call_single_step(a: [u64; 4], b: [u64; 4]) -> [u64; 4] {
    let mut out = [0; 4];
    unsafe {
        asm!(
            "bl _single_step",
            // input
            in("x0") a[0], in("x1") a[1], in("x2") a[2], in("x3") a[3],
            in("x4") b[0], in("x5") b[1], in("x6") b[2], in("x7") b[3],
            // output
            lateout("x2") out[0], lateout("x3") out[1], lateout("x1") out[2], lateout("x0") out[3],
            // single step clobbers the following registers
            lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _,
            lateout("x8") _, lateout("x9") _, lateout("x10") _, lateout("x11") _, lateout("x12") _, lateout("x13") _, lateout("x14") _,
            lateout("lr") _
        )
    };
    out
}

#[inline(never)]
fn call_smul_add(t: [u64; 5], a: [u64; 4], b: u64) -> [u64; 5] {
    let mut out = [0; 5];
    unsafe {
        asm!(
            "bl _smul_add",
            in("x0") t[0], in("x1") t[1], in("x2") t[2], in("x3") t[3], in("x4") t[4],
            in("x5") a[0], in("x6") a[1], in("x7") a[2], in("x8") a[3],
            in("x9") b,
            lateout("x10") out[0], lateout("x0") out[1], lateout("x1") out[2], lateout("x2") out[3], lateout("x3") out[4],
            lateout("lr") _
        )
    };
    out
}

#[cfg(test)]
mod tests {
    use mod256_generator::U256b64;
    use montgomery_reduction::{arith, yuval};
    use quickcheck_macros::quickcheck;

    use crate::{call_schoolmethod, call_single_step, call_smul, call_smul_add};

    #[quickcheck]
    fn smul(a: U256b64, b: u64) -> bool {
        let a = a.0;
        arith::smul(b, a) == call_smul(a, b)
    }

    #[quickcheck]
    fn school_method(a: U256b64, b: U256b64) -> bool {
        let a = a.0;
        let b = b.0;
        arith::school_method(b, a) == call_schoolmethod(a, b)
    }

    #[quickcheck]
    fn smul_add(a: U256b64, mut t: U256b64, tp: u64, b: u64) -> bool {
        let a = a.0;
        let t = [tp, t.0[0], t.0[1], t.0[2], t.0[3]];
        arith::addv(arith::smul(b, a), t) == call_smul_add(t, a, b)
    }

    #[quickcheck]
    fn single_step(a: U256b64, b: U256b64) -> bool {
        yuval::parallel(b.0, a.0) == call_single_step(a.0, b.0)
    }
}
