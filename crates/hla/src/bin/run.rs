use std::arch::{asm, global_asm};

global_asm!(include_str!("../../asm/mulu128.s"));

global_asm!(include_str!("../../asm/global_asm_smul.s"));
global_asm!(include_str!("../../asm/global_asm_schoolmethod.s"));

fn main() {
    let r = call_mulu128(5, 6);
    println!("r: {r:?}");
    let r = inline_call_mulu128(5, 6);
    println!("r: {r:?}");
    let r = call_smul([1, 2, 3, 4], 5);
    println!("r: {r:?}");
    let r = call_schoolmethod([1, 2, 3, 4], [5, 6, 7, 8]);
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
            lateout("x5") out[0], lateout("x6") out[1], lateout("x1") out[2], lateout("x2") out[3], lateout("x0") out[4]
        )
    };
    out
}

#[inline(never)]
fn call_schoolmethod(a: [u64; 4], b: [u64; 4]) -> [u64; 8] {
    let mut out = [0; 8];
    unsafe {
        asm!(
            "bl _schoolmethod",
            in("x0") a[0], in("x1") a[1], in("x2") a[2], in("x3") a[3],
            in("x4") b[0], in("x5") b[1], in("x6") b[2], in("x7") b[3],
            lateout("x8") out[0], lateout("x12") out[1], lateout("x4") out[2], lateout("x5") out[3], lateout("x10") out[4], lateout("x0") out[5], lateout("x1") out[6], lateout("x3") out[7]

        )
    };
    out
}

#[cfg(test)]
mod tests {
    use montgomery_reduction::arith;
    use quickcheck_macros::quickcheck;

    use crate::{call_schoolmethod, call_smul};

    #[quickcheck]
    fn smul(a0: u64, a1: u64, a2: u64, a3: u64, b: u64) -> bool {
        let a = [a0, a1, a2, a3];
        arith::smul(b, a) == call_smul(a, b)
    }
    #[quickcheck]
    fn school_method(a0: u64, a1: u64, a2: u64, a3: u64, b: u64) -> bool {
        let a = [a0, a1, a2, a3];
        let b = [a3, a1, a2, a0];
        arith::school_method(b, a) == call_schoolmethod(a, b)
    }
}
