use std::hint::black_box;

use montgomery_reduction::{emmart, U52_NP0, U52_P};
use rand::{rngs::StdRng, Rng, SeedableRng};

fn main() {
    let seed: u64 = rand::random();
    let mut rng = StdRng::seed_from_u64(seed);

    emmart::set_round_to_zero();

    let a = [
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
    ];
    let b = [
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
    ];
    let c = [
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
    ];
    let d = [
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
    ];
    let e = [
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
    ];
    let f = [
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
    ];
    let g = [
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
    ];
    let h = [
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
    ];
    let i = [
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
    ];
    let j = [
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
    ];

    for _ in 0..1_000_000_000 {
        // let _ = black_box(acar::cios_opt(a, b, P, NP0));
        // let _ = black_box(acar::cios_opt(black_box(a64), black_box(b64), P, NP0));
        let _ = black_box(emmart::fios_opt_sub_simd_sat_seq(
            black_box(a),
            black_box(b),
            black_box(c),
            black_box(d),
            black_box(e),
            black_box(f),
            black_box(g),
            black_box(h),
            black_box(i),
            black_box(j),
            black_box(j),
            black_box(i),
            U52_P,
            U52_NP0,
        ));
    }
}
