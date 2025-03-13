use std::arch::asm;

fn main() {
    let r = smult_inline([1; 4], 7);
    println!("{r:?}")
}

#[inline(never)]
pub fn smult_inline(a: [u64; 4], b: u64) -> [u64; 5] {
    let mut s = [0; 5];
    unsafe {
        asm!(
                    "mul {s0},{a0},{b}",
                    "umulh {s1}, {a0}, {b}",
                    //
                    "mul {tmp}, {a1}, {b}",
                    "umulh {s2}, {a0}, {b}",
                    "adds {s1}, {s1}, {tmp}",
        //
                    "mul {tmp}, {a2}, {b}",
                    "umulh {s3}, {a2}, {b}",
                    "adcs {s2}, {s2}, {tmp}",
                    //
                    "mul {tmp}, {a3}, {b}",
                    "umulh {s4}, {a3}, {b}",
                    "adcs {s3}, {s3}, {tmp}",
                    "cinc {s4}, {s4}, hs",

                    tmp = out(reg) _,
                    a0 = in(reg) a[0],
                    a1 = in(reg) a[1],
                    a2 = in(reg) a[2],
                    a3 = in(reg) a[3],
                    b = in(reg) b,
                    s0 = out(reg) s[0],
                    s1 = out(reg) s[1],
                    s2 = out(reg) s[2],
                    s3 = out(reg) s[3],
                    s4 = out(reg) s[4]
                )
    }

    s
}
