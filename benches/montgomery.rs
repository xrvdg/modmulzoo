use criterion::{black_box, criterion_group, criterion_main, Criterion};
use montgomery_reduction::{
    cios, cios_opt, fios, sampled_product, sampled_product_masked, school_method,
    set_round_to_zero, sos, U256b52, U256b64,
};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

const NP0: u64 = 0xc2e1f593efffffff;
const P: [u64; 4] = [
    0x43e1f593f0000001,
    0x2833e84879b97091,
    0xb85045b68181585d,
    0x30644e72e131a029,
];

fn bench_montgomery(c: &mut Criterion) {
    let mut group = c.benchmark_group("Montgomery Multiplication");

    // Generate and print a random seed
    let seed: u64 = rand::random();
    println!("Using random seed: {}", seed);
    let mut rng = StdRng::seed_from_u64(seed);

    // Generate random test case
    let a = [
        rng.gen::<u64>(),
        rng.gen::<u64>(),
        rng.gen::<u64>(),
        rng.gen::<u64>(),
    ];
    let b = [
        rng.gen::<u64>(),
        rng.gen::<u64>(),
        rng.gen::<u64>(),
        rng.gen::<u64>(),
    ];

    println!("Random test values:");
    println!("a = {:?}", a);
    println!("b = {:?}", b);

    group.bench_function("sos_random", |bencher| {
        bencher.iter(|| sos(black_box(a), black_box(b), P, NP0))
    });

    group.bench_function("fios_random", |bencher| {
        bencher.iter(|| fios(black_box(a), black_box(b), P, NP0))
    });

    group.bench_function("cios_random", |bencher| {
        bencher.iter(|| cios(black_box(a), black_box(b), P, NP0))
    });

    group.bench_function("cios_opt_random", |bencher| {
        bencher.iter(|| cios_opt(black_box(a), black_box(b), P, NP0))
    });

    group.bench_function("mul_school_method", |bencher| {
        bencher.iter(|| school_method(black_box(U256b64(a)), black_box(U256b64(b))))
    });

    // Set up for sampled_product_masked benchmark
    set_round_to_zero();
    let a64 = U256b64(a);
    let b64 = U256b64(b);
    let a52: U256b52 = a64.into();
    let b52: U256b52 = b64.into();
    let a_float = a52.0.map(|x| x as f64);
    let b_float = b52.0.map(|x| x as f64);

    group.bench_function("mul_sampled_product_masked_random", |bencher| {
        bencher.iter(|| sampled_product_masked(black_box(a_float), black_box(b_float)))
    });
    group.bench_function("mul_sampled_product_random", |bencher| {
        bencher.iter(|| sampled_product(black_box(a_float), black_box(b_float)))
    });

    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default()
        .sample_size(5000)
        .warm_up_time(std::time::Duration::new(3,0))
        .measurement_time(std::time::Duration::new(10,0));
    targets = bench_montgomery
);
criterion_main!(benches);
