use std::arch::{asm, global_asm};

global_asm!(include_str!("../asm/global_asm_single_step.s"));
global_asm!(include_str!("../asm/global_asm_schoolmethod.s"));

#[inline(always)]
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

#[inline(always)]
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

#[inline(always)]
pub fn call_schoolmethod_inline(a: [u64; 4], b: [u64; 4]) -> [u64; 8] {
    let mut out = [0; 8];
    unsafe {
        asm!(
        "mul x8, x0, x4",
        "umulh x9, x0, x4",
        "mul x10, x1, x4",
        "umulh x11, x1, x4",
        "adds x10, x10, x9",
        "cinc x11, x11, hs",
        "mul x9, x2, x4",
        "umulh x12, x2, x4",
        "adds x9, x9, x11",
        "cinc x12, x12, hs",
        "mul x11, x3, x4",
        "umulh x4, x3, x4",
        "adds x11, x11, x12",
        "cinc x4, x4, hs",
        "mul x12, x0, x5",
        "umulh x13, x0, x5",
        "adds x12, x12, x10",
        "cinc x13, x13, hs",
        "mul x10, x1, x5",
        "umulh x14, x1, x5",
        "adds x10, x10, x13",
        "cinc x14, x14, hs",
        "adds x10, x10, x9",
        "cinc x14, x14, hs",
        "mul x9, x2, x5",
        "umulh x13, x2, x5",
        "adds x9, x9, x14",
        "cinc x13, x13, hs",
        "adds x9, x9, x11",
        "cinc x13, x13, hs",
        "mul x11, x3, x5",
        "umulh x5, x3, x5",
        "adds x11, x11, x13",
        "cinc x5, x5, hs",
        "adds x11, x11, x4",
        "cinc x5, x5, hs",
        "mul x4, x0, x6",
        "umulh x13, x0, x6",
        "adds x4, x4, x10",
        "cinc x13, x13, hs",
        "mul x10, x1, x6",
        "umulh x14, x1, x6",
        "adds x10, x10, x13",
        "cinc x14, x14, hs",
        "adds x10, x10, x9",
        "cinc x14, x14, hs",
        "mul x9, x2, x6",
        "umulh x13, x2, x6",
        "adds x9, x9, x14",
        "cinc x13, x13, hs",
        "adds x9, x9, x11",
        "cinc x13, x13, hs",
        "mul x11, x3, x6",
        "umulh x6, x3, x6",
        "adds x11, x11, x13",
        "cinc x6, x6, hs",
        "adds x11, x11, x5",
        "cinc x6, x6, hs",
        "mul x5, x0, x7",
        "umulh x0, x0, x7",
        "adds x5, x5, x10",
        "cinc x0, x0, hs",
        "mul x10, x1, x7",
        "umulh x1, x1, x7",
        "adds x10, x10, x0",
        "cinc x1, x1, hs",
        "adds x10, x10, x9",
        "cinc x1, x1, hs",
        "mul x0, x2, x7",
        "umulh x2, x2, x7",
        "adds x0, x0, x1",
        "cinc x2, x2, hs",
        "adds x0, x0, x11",
        "cinc x2, x2, hs",
        "mul x1, x3, x7",
        "umulh x3, x3, x7",
        "adds x1, x1, x2",
        "cinc x3, x3, hs",
        "adds x1, x1, x6",
        "cinc x3, x3, hs",
        in("x0") a[0], in("x1") a[1], in("x2") a[2], in("x3") a[3],
        in("x4") b[0], in("x5") b[1], in("x6") b[2], in("x7") b[3],
        lateout("x8") out[0], lateout("x12") out[1], lateout("x4") out[2], lateout("x5") out[3],
        lateout("x10") out[4], lateout("x0") out[5], lateout("x1") out[6], lateout("x3") out[7],
        lateout("x2") _, lateout("x6") _, lateout("x7") _, lateout("x9") _, lateout("x11") _, lateout("x13") _, lateout("x14") _,
        )
    };
    out
}

#[inline(never)]
pub fn call_schoolmethod_stub(a: [u64; 4], b: [u64; 4]) -> [u64; 8] {
    call_schoolmethod(a, b)
}

#[inline(never)]
pub fn call_schoolmethod_inline_stub(a: [u64; 4], b: [u64; 4]) -> [u64; 8] {
    call_schoolmethod_inline(a, b)
}
