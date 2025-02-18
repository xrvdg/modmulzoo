use std::hint::black_box;

use montgomery_reduction::{
    emmart::fios_opt_sub_f64, set_round_to_zero, U256b52, U52_NP0, U52_P, U52_R2,
};

fn main() {
    set_round_to_zero();
    let a = U256b52([1, 2, 3, 4, 5]);
    let b = U256b52([10, 20, 30, 40, 50]);

    let _c = black_box(fios_opt_sub_f64(
        black_box(a),
        black_box(b),
        U256b52(U52_P),
        U52_NP0,
    ));
}
