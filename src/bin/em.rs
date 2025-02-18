use montgomery_reduction::{
    emmart::fios_opt_f64, set_round_to_zero, U256b52, U52_NP0, U52_P, U52_R2,
};

fn main() {
    println!("cios_opt");
    set_round_to_zero();
    let a = U256b52([0, 0, 0, 0, 1]);

    let _ = fios_opt_f64(a, U256b52(U52_R2), U256b52(U52_P), U52_NP0);
}
