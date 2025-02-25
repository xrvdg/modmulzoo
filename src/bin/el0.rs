fn main() {
    let tmp: u64;
    unsafe {
        core::arch::asm!(
            "mrs {tmp}, ctr_el0",
            tmp = out(reg) tmp
        );
    }
    println!("tmp: {tmp}");
}
