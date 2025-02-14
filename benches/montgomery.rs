use criterion::{black_box, criterion_group, criterion_main, Criterion};
use montgomery_reduction::{cios, cios_opt, fios, sos};
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

    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default()
        .sample_size(10000)
        .warm_up_time(std::time::Duration::new(5,0))
        .measurement_time(std::time::Duration::new(20,0));
    targets = bench_montgomery
);
criterion_main!(benches);
