use criterion::{black_box, criterion_group, criterion_main, Criterion};
use montgomery_reduction::{montgomery_reduction, montgomery_reduction_power_of_two};

fn benchmark_montgomery_reductions(c: &mut Criterion) {
    let mut group = c.benchmark_group("Montgomery Reduction");

    // Test with different input sizes
    let test_cases = vec![
        (42, 17), // Small numbers
        (80, 70), // Medium numbers
    ];

    for (a, b) in test_cases {
        let p = a * b;

        group.bench_function(format!("standard_reduction_{}x{}", a, b), |b| {
            b.iter(|| montgomery_reduction(black_box(p), black_box(97), black_box(100)))
        });

        group.bench_function(format!("power_of_two_reduction_{}x{}", a, b), |b| {
            b.iter(|| {
                montgomery_reduction_power_of_two(black_box(p), black_box(97), black_box(256))
            })
        });
    }

    group.finish();
}

criterion_group!(benches, benchmark_montgomery_reductions);
criterion_main!(benches);
