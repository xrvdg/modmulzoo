use std::mem;

use dynasmrt::{dynasm, DynasmApi, DynasmLabelApi};

#[cfg(target_os = "linux")]
fn main() {
    let mut ops = dynasmrt::aarch64::Assembler::new().unwrap();

    // dynasm!(ops
    // ; .arch aarch64
    // ; ->add: );

    let add = ops.offset();

    dynasm!(ops
        ; add x0, x0, x1
        ; ret
    );

    let buf = ops.finalize().unwrap();
    println!("{add:?}");
    let add_fn: extern "C" fn(u64, u64) -> u64 = unsafe { mem::transmute(buf.ptr(add)) };

    let res = add_fn(2, 3);
    println!("{res}")
}

#[cfg(target_os = "macos")]
fn main() {
    unimplemented!()
}
