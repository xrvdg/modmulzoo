use criterion::{Criterion, black_box, criterion_group, criterion_main};
use hla::montgomery::{call_schoolmethod, call_single_step};
use rand::Rng;

fn generate_random_array() -> [u64; 4] {
    let mut rng = rand::thread_rng();
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
            black_box(call_single_step(a, b))
        })
    });

    group.finish();
}

fn bench_schoolmethod(c: &mut Criterion) {
    let mut group = c.benchmark_group("Schoolbook Multiplication");
    let a = generate_random_array();
    let b = generate_random_array();

    group.bench_function("schoolmethod", |bencher| {
        bencher.iter(|| {
            let a = black_box(a);
            let b = black_box(b);
            black_box(call_schoolmethod(a, b))
        })
    });

    group.finish();
}

criterion_group!(benches, bench_single_step, bench_schoolmethod);
criterion_main!(benches);
