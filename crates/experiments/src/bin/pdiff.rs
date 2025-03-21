use block_multiplier::constants::P;
/// Explore the differences between the single step unsigned and floating point algorithm
/// and how many excessive Ps they contain.
use montgomery_reduction::{arith, domb, yuval};
use num_bigint::BigUint;
use rand::Rng;
use std::collections::HashMap;

fn generate_random_input() -> [u64; 4] {
    let mut rng = rand::rng();
    let to_biguint = |v: &[u64]| {
        BigUint::from_bytes_le(&v.iter().flat_map(|b| b.to_le_bytes()).collect::<Vec<u8>>())
    };
    let max = BigUint::new(vec![u32::max_value(); 8]) - 2_u32 * to_biguint(&P);

    let mut val = rng.random::<[u64; 4]>();

    // The random generator in num_bigint doesn't work past rand 0.7. Therefore we sample a random value and if it's
    // above our maximum we'll sample again.
    while to_biguint(&val) > max {
        val = rng.random::<[u64; 4]>();
    }

    val
}

fn main() {
    println!("Testing differences between parallel_sub_r256 and yuval::parallel");
    println!("============================================================");

    // Create a histogram to track frequency of p count diffs
    let mut p_diff_histogram: HashMap<u64, u32> = HashMap::new();
    let mut p_float_histogram: HashMap<u64, u32> = HashMap::new();
    let mut p_unsigned_histogram: HashMap<u64, u32> = HashMap::new();
    let num_test_cases = 10_000_000;

    // Statistics tracking
    let mut equal_modp_count = 0;
    let mut equal_result_count = 0;

    println!("Generating and testing {} random cases...", num_test_cases);

    // Generate and test random inputs
    for _ in 0..num_test_cases {
        let input = generate_random_input();
        let input2 = generate_random_input();

        // Calculate results using both methods
        let result_domb = domb::parallel_sub_r256(input, input2);
        let result_yuval = yuval::parallel(input, input2);

        // Calculate modular values
        let (mod_domb, count_domb) = modulus_count(result_domb, P);
        let (mod_yuval, count_yuval) = modulus_count(result_yuval, P);

        *p_unsigned_histogram.entry(count_yuval).or_insert(0) += 1;
        *p_float_histogram.entry(count_domb).or_insert(0) += 1;
        // Are they equal modulo P?
        assert_eq!(mod_domb, mod_yuval);
        if mod_domb == mod_yuval {
            equal_modp_count += 1;
        }

        // Are the raw results equal?
        let equal_result = result_domb == result_yuval;
        if equal_result {
            equal_result_count += 1;
        }

        // Count how many P's difference there is
        let p_count = count_domb.abs_diff(count_yuval);

        // Update histogram
        *p_diff_histogram.entry(p_count).or_insert(0) += 1;

        // Print progress every 100 cases
    }

    // Print summary statistics
    println!("\nSummary:");
    println!("Total test cases: {}", num_test_cases);
    println!(
        "Equal modulo P: {} ({:.2}%)",
        equal_modp_count,
        (equal_modp_count as f64 / num_test_cases as f64) * 100.0
    );
    println!(
        "Equal raw result: {} ({:.2}%)",
        equal_result_count,
        (equal_result_count as f64 / num_test_cases as f64) * 100.0
    );

    // Print histograms for all three types
    for (histogram_name, histogram) in [
        ("P Count Difference Histogram", p_diff_histogram),
        ("P Count Unsigned Histogram", p_unsigned_histogram),
        ("P Count Float Histogram", p_float_histogram),
    ] {
        println!("\n{}:", histogram_name);
        println!(
            "| {:^10} | {:^10} | {:^10} |",
            "P Count", "Frequency", "Percentage"
        );
        println!("|{:-<12}|{:-<12}|{:-<12}|", "", "", "");

        // Convert histogram to a vector for sorting
        let mut histogram_vec: Vec<(u64, u32)> = histogram.into_iter().collect();
        histogram_vec.sort_by_key(|&(count, _)| count);

        for (p_count, frequency) in histogram_vec {
            let percentage = (frequency as f64 / num_test_cases as f64) * 100.0;
            println!(
                "| {:<10} | {:<10} | {:>8.2}% |",
                p_count, frequency, percentage
            );
        }
    }
}

fn modulus_count<const N: usize>(a: [u64; N], b: [u64; N]) -> ([u64; N], u64) {
    let mut d = a;
    let mut prev = d;
    let mut count = 0;
    loop {
        d = arith::subtraction_step(d, b);
        if d == prev {
            break;
        }
        prev = d;
        count += 1;
    }
    (d, count)
}
