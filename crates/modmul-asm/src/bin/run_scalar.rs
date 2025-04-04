use modmul_asm::call_single_step;

fn main() {
    let a = [1, 2, 3, 4];
    let b = [6, 7, 8, 9];

    println!("{:?}", call_single_step(a, b));
}
