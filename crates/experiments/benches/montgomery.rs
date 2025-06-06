#![feature(portable_simd)]
use std::array;
use std::simd::Simd;

use block_multiplier::constants::{NP0, P, U52_NP0, U52_P};
use block_multiplier::rtz::RTZ;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use montgomery_reduction::arith::school_method;
use montgomery_reduction::emmart;
use montgomery_reduction::{acar, domb, interleaved, yuval};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use ark_ff::{Field, Fp256, MontBackend, MontConfig};
#[derive(MontConfig)]
#[modulus = "21888242871839275222246405745257275088548364400416034343698204186575808495617"]
#[generator = "5"]
pub struct BN254Config;
pub type Field256 = Fp256<MontBackend<BN254Config, 4>>;

fn bench_acar(c: &mut Criterion) {
    let mut group = c.benchmark_group("Acar");

    // Generate and print a random seed
    let seed: u64 = rand::random();
    println!("Using random seed: {}", seed);
    let mut rng = StdRng::seed_from_u64(seed);

    let a = array::from_fn(|_| rng.random::<u64>());
    let b = array::from_fn(|_| rng.random::<u64>());
    let c = array::from_fn(|_| rng.random::<u64>());
    let d = array::from_fn(|_| rng.random::<u64>());

    println!("Random test values:");
    println!("a = {:?}", a);
    println!("b = {:?}", b);

    let ark_a = Field256::new(ark_ff::BigInt(a));
    let ark_b = Field256::new(ark_ff::BigInt(b));

    group.bench_function("ark_ff", |bencher| {
        bencher.iter(|| black_box(ark_a) * black_box(ark_b))
    });

    group.bench_function("sos_random", |bencher| {
        bencher.iter(|| acar::sos(black_box(a), black_box(b), P, NP0))
    });

    group.bench_function("fios_random", |bencher| {
        bencher.iter(|| acar::fios(black_box(a), black_box(b), P, NP0))
    });

    group.bench_function("cios_random", |bencher| {
        bencher.iter(|| acar::cios(black_box(a), black_box(b), P, NP0))
    });

    group.bench_function("cios_opt_random", |bencher| {
        bencher.iter(|| acar::cios_opt(black_box(a), black_box(b), P, NP0))
    });

    group.bench_function("cios_opt_seq_random", |bencher| {
        bencher.iter(|| acar::cios_opt_seq(black_box(a), black_box(b), P, NP0))
    });

    group.bench_function("cios_opt_sat_random", |bencher| {
        bencher.iter(|| {
            acar::cios_opt_sat(
                black_box(a),
                black_box(b),
                black_box(c),
                black_box(d),
                P,
                NP0,
            )
        })
    });

    group.bench_function("mul_school_method", |bencher| {
        bencher.iter(|| school_method(black_box(a), black_box(b)))
    });
}

fn bench_emmart(c: &mut Criterion) {
    let mut group = c.benchmark_group("Emmart");

    // Generate and print a random seed
    let seed: u64 = rand::random();
    println!("Using random seed: {}", seed);
    let mut rng = StdRng::seed_from_u64(seed);

    // SET ROUND TO ZERO BENCHES
    let rtz = block_multiplier::rtz::RTZ::set().unwrap();
    let a = array::from_fn(|_| rng.random::<u64>());
    let b = array::from_fn(|_| rng.random::<u64>());
    let c = array::from_fn(|_| rng.random::<u64>());
    let d = array::from_fn(|_| rng.random::<u64>());
    let e = array::from_fn(|_| rng.random::<u64>());
    let f = array::from_fn(|_| rng.random::<u64>());
    let g = array::from_fn(|_| rng.random::<u64>());
    let h = array::from_fn(|_| rng.random::<u64>());
    let i = array::from_fn(|_| rng.random::<u64>());
    let j = array::from_fn(|_| rng.random::<u64>());
    let k = array::from_fn(|_| rng.random::<u64>());
    let l = array::from_fn(|_| rng.random::<u64>());

    let a_float = a.map(|x| x as f64);
    let b_float = b.map(|x| x as f64);

    group.bench_function("mul_sampled_product_masked_random", |bencher| {
        bencher.iter(|| {
            emmart::paper::sampled_product_masked(&rtz, black_box(a_float), black_box(b_float))
        })
    });
    group.bench_function("mul_sampled_product_random", |bencher| {
        bencher.iter(|| emmart::paper::sampled_product(black_box(a_float), black_box(b_float)))
    });

    group.bench_function("cios_opt_random", |bencher| {
        bencher.iter(|| emmart::cios_opt(&rtz, black_box(a), black_box(b), U52_P, U52_NP0))
    });

    group.bench_function("fios_opt_random", |bencher| {
        bencher.iter(|| emmart::fios_opt(&rtz, black_box(a), black_box(b), U52_P, U52_NP0))
    });
    group.bench_function("fios_opt_sub_random", |bencher| {
        bencher.iter(|| emmart::fios_opt_sub(&rtz, black_box(a), black_box(b), U52_P, U52_NP0))
    });
    group.bench_function("fios_opt_sub_sat_random", |bencher| {
        bencher.iter(|| {
            emmart::fios_opt_sub_sat(
                &rtz,
                black_box(a),
                black_box(b),
                black_box(c),
                black_box(d),
                U52_P,
                U52_NP0,
            )
        })
    });
    group.bench_function("fios_opt_sub_simd_random", |bencher| {
        bencher.iter(|| {
            emmart::fios_opt_sub_simd(&rtz, black_box(a), black_box(b), black_box(c), black_box(d))
        })
    });
    group.bench_function("cios_opt_sub_simd_random", |bencher| {
        bencher.iter(|| {
            emmart::cios_opt_sub_simd(&rtz, black_box(a), black_box(b), black_box(c), black_box(d))
        })
    });
    group.bench_function("fios_opt_sub_simd_sat_random", |bencher| {
        bencher.iter(|| {
            emmart::fios_opt_sub_simd_sat(
                &rtz,
                black_box(a),
                black_box(b),
                black_box(c),
                black_box(d),
                black_box(e),
                black_box(f),
                black_box(g),
                black_box(h),
                U52_P,
                U52_NP0,
            )
        })
    });
    group.bench_function("fios_opt_sub_simd_sat_seq_random", |bencher| {
        bencher.iter(|| {
            emmart::fios_opt_sub_simd_sat_seq(
                &rtz,
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
                black_box(k),
                black_box(l),
                U52_P,
                U52_NP0,
            )
        })
    });
    group.bench_function("fios_opt_sub_simd_seq_random", |bencher| {
        bencher.iter(|| {
            emmart::fios_opt_sub_simd_seq(
                &rtz,
                black_box(a),
                black_box(b),
                black_box(c),
                black_box(d),
                black_box(i),
                black_box(j),
                black_box(k),
                black_box(l),
                U52_P,
                U52_NP0,
            )
        })
    });
    group.bench_function("cios_opt_sub_random", |bencher| {
        bencher.iter(|| emmart::cios_opt_sub(&rtz, black_box(a), black_box(b)))
    });

    let resolve = [Simd::splat(rng.random()); 6];
    group.bench_function("resolve_simd", |bencher| {
        bencher.iter(|| emmart::resolve_simd(resolve))
    });

    group.finish();
}

fn bench_domb(c: &mut Criterion) {
    let mut group = c.benchmark_group("Domb");

    let seed: u64 = rand::random();
    println!("Using random seed for parallel benchmarks: {}", seed);
    let mut rng = StdRng::seed_from_u64(seed);

    let yuval_a = array::from_fn(|_| rng.random::<u64>());
    let yuval_b = array::from_fn(|_| rng.random::<u64>());

    let domb_a = array::from_fn(|_| rng.random::<u64>());
    let domb_b = array::from_fn(|_| rng.random::<u64>());

    // Benchmark yuval parallel implementation
    group.bench_function("parallel", |bencher| {
        bencher.iter(|| yuval::parallel(black_box(yuval_a), black_box(yuval_b)))
    });
    // Benchmark yuval parallel implementation
    group.bench_function("parallel_reduce", |bencher| {
        bencher.iter(|| yuval::parallel_reduce(black_box(yuval_a), black_box(yuval_b)))
    });

    group.bench_function("parallel_reduce_ct", |bencher| {
        bencher.iter(|| yuval::parallel_reduce_ct(black_box(yuval_a), black_box(yuval_b)))
    });

    let rtz = RTZ::set().unwrap();
    // Benchmark domb parallel implementation
    group.bench_function("parallel_f64", |bencher| {
        bencher.iter(|| domb::parallel_ref(&rtz, black_box(domb_a), black_box(domb_b)))
    });

    group.bench_function("parallel_f64_sub", |bencher| {
        bencher.iter(|| domb::parallel_sub(&rtz, black_box(domb_a), black_box(domb_b)))
    });

    group.bench_function("parallel_f64_sub_cond", |bencher| {
        bencher.iter(|| domb::parallel_sub(&rtz, black_box(domb_a), black_box(domb_b)))
    });

    group.bench_function("parallel_f64_r256", |bencher| {
        bencher.iter(|| domb::parallel_sub_r256(&rtz, black_box(yuval_a), black_box(yuval_b)))
    });

    group.bench_function("parallel_f64_simd_r256", |bencher| {
        bencher.iter(|| {
            domb::parallel_sub_simd_r256(
                &rtz,
                black_box([yuval_a, yuval_b]),
                black_box([yuval_a, yuval_b]),
            )
        })
    });

    group.bench_function("u256_to_u260_shl2", |bencher| {
        bencher.iter(|| domb::u256_to_u260_shl2(black_box(yuval_a)))
    });

    group.bench_function("parallel_f64_sub_simd", |bencher| {
        bencher.iter(|| {
            domb::parallel_simd_sub(
                &rtz,
                black_box([domb_a, domb_b]),
                black_box([domb_b, domb_a]),
            )
        })
    });

    // Add benchmark for reduce function
    let red = array::from_fn(|_| rng.random::<u64>());
    group.bench_function("reduce_b52", |bencher| {
        bencher.iter(|| domb::reduce_ct(black_box(red)))
    });

    group.bench_function("reduce_ct", |bencher| {
        bencher.iter(|| yuval::reduce_ct(black_box(yuval_a)))
    });

    group.bench_function("reduce", |bencher| {
        bencher.iter(|| yuval::reduce(black_box(yuval_a)))
    });

    group.finish();
}

fn bench_interleaved(c: &mut Criterion) {
    let mut group = c.benchmark_group("Interleaved");

    let seed: u64 = rand::random();
    println!("Using random seed for parallel benchmarks: {}", seed);
    let mut rng = StdRng::seed_from_u64(seed);

    let yuval_a = array::from_fn(|_| rng.random::<u64>());
    let yuval_b = array::from_fn(|_| rng.random::<u64>());

    let domb_a = array::from_fn(|_| rng.random::<u64>());
    let domb_b = array::from_fn(|_| rng.random::<u64>());

    // let rtz = RTZ::set().unwrap();

    group.bench_function("interleaved", |bencher| {
        bencher.iter(|| {
            interleaved::interleaved(
                black_box(yuval_a),
                black_box(yuval_b),
                black_box([domb_a, domb_b]),
                black_box([domb_b, domb_a]),
            )
        })
    });

    group.bench_function("interleaved_naive", |bencher| {
        bencher.iter(|| {
            interleaved::interleaved_naive(
                black_box(yuval_a),
                black_box(yuval_b),
                black_box([domb_a, domb_b]),
                black_box([domb_b, domb_a]),
            )
        })
    });

    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default()
        .sample_size(5000)
        // Warm up is warm because it literally warms up the pi
        .warm_up_time(std::time::Duration::new(1,0))
        .measurement_time(std::time::Duration::new(10,0));
    targets = bench_acar, bench_emmart, bench_domb, bench_interleaved
);
criterion_main!(benches);
