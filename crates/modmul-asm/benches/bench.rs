#![feature(portable_simd)]
use std::simd::Simd;

use block_multiplier::rtz::RTZ;
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use modmul_asm::{
    call_schoolmethod, call_schoolmethod_inline, call_single_step, call_single_step_5,
    call_single_step_interleaved, call_single_step_interleaved_seq_scalar, call_single_step_simd,
    call_single_step_split,
};
use rand::Rng;

fn generate_random_array() -> [u64; 4] {
    let mut rng = rand::rng();
    [
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
        rng.random::<u64>(),
    ]
}

fn bench_single_step(c: &mut Criterion) {
    let mut group = c.benchmark_group("Montgomery Reduction");
    let a = generate_random_array();
    let b = generate_random_array();

    group.bench_function("single_step", |bencher| {
        bencher.iter(|| {
            let a = black_box(a);
            let b = black_box(b);
            call_single_step(a, b)
        })
    });

    group.bench_function("single_step_split", |bencher| {
        bencher.iter(|| {
            let a = black_box(a);
            let b = black_box(b);
            call_single_step_split(a, b)
        })
    });

    let rtz = RTZ::set().unwrap();

    let av = a.map(|i| Simd::splat(i));
    let bv = b.map(|i| Simd::splat(i));

    group.bench_function("single_step_simd", |bencher| {
        bencher.iter(|| {
            let a = black_box(av);
            let b = black_box(bv);
            call_single_step_simd(&rtz, a, b)
        })
    });

    group.bench_function("single_step_interleaved", |bencher| {
        bencher.iter(|| {
            let a = black_box(a);
            let b = black_box(b);
            let av = black_box(av);
            let bv = black_box(bv);
            call_single_step_interleaved(&rtz, a, b, av, bv)
        })
    });

    group.bench_function("single_step_interleaved_seq_scalar", |bencher| {
        bencher.iter(|| {
            let a = black_box(a);
            let b = black_box(b);
            let c = black_box(b);
            let d = black_box(a);
            let av = black_box(av);
            let bv = black_box(bv);
            call_single_step_interleaved_seq_scalar(&rtz, a, b, c, d, av, bv)
        })
    });

    group.bench_function("single_step_interleaved_5", |bencher| {
        bencher.iter(|| {
            let a = black_box(a);
            let b = black_box(b);
            let c = black_box(b);
            let d = black_box(a);
            let av = black_box(av);
            let bv = black_box(bv);
            call_single_step_5(&rtz, a, b, c, d, a, d, av, bv)
        })
    });

    group.finish();
}

fn bench_schoolmethod(c: &mut Criterion) {
    let mut group = c.benchmark_group("Schoolbook");
    let a = generate_random_array();
    let b = generate_random_array();

    group.bench_function("schoolmethod", |bencher| {
        bencher.iter(|| {
            let a = black_box(a);
            let b = black_box(b);
            call_schoolmethod(a, b)
        })
    });

    group.bench_function("schoolmethod inline", |bencher| {
        bencher.iter(|| {
            let a = black_box(a);
            let b = black_box(b);
            call_schoolmethod_inline(a, b)
        })
    });

    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default()
        .without_plots()
        .sample_size(5000)
        // Warm up is warm because it literally warms up the pi
        .warm_up_time(std::time::Duration::new(1,0))
        .measurement_time(std::time::Duration::new(10,0));
    targets = bench_single_step, bench_schoolmethod
);
criterion_main!(benches);
