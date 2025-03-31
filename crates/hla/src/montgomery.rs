use std::arch::{asm, global_asm};

global_asm!(include_str!("../asm/global_asm_single_step.s"));
global_asm!(include_str!("../asm/global_asm_schoolmethod.s"));

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
pub fn call_schoolmethod(a: [u64; 4], b: [u64; 4]) -> [u64; 8] {
    let mut out = [0; 8];
    unsafe {
        asm!(
            "bl _schoolmethod",
            in("x0") a[0], in("x1") a[1], in("x2") a[2], in("x3") a[3],
            in("x4") b[0], in("x5") b[1], in("x6") b[2], in("x7") b[3],
            lateout("x8") out[0], lateout("x12") out[1], lateout("x4") out[2], lateout("x5") out[3],
            lateout("x10") out[4], lateout("x0") out[5], lateout("x1") out[6], lateout("x3") out[7],
            lateout("x2") _, lateout("x6") _, lateout("x7") _, lateout("x9") _, lateout("x11") _, lateout("x13") _, lateout("x14") _,
            lateout("lr") _
        )
    };
    out
}
