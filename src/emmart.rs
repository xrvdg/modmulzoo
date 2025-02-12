// struct U256<N> where N is limbsize

use num_traits::MulAdd;

// TODO: how to deal with all the converions
// Int to float is an expensive operation, or not if you do a casting?
// TODO how to ensure that the right multiplication algorithm is used.
// - First compare what the difference is
fn full_product(a: f64, b: f64) -> (f64, f64) {
    let p_hi = a.mul_add(b, 0.);
    let p_lo: f64 = a.mul_add(b, -p_hi);
    (p_lo, p_hi)
}

fn dpf_full_product(a: f64, b: f64) -> (f64, f64) {
    let c1 = 2.0_f64.powi(104);
    let c2 = 2.0_f64.powi(104) + 2.0_f64.powi(52);
    let c3 = 2.0_f64.powi(52);

    let p_hi = a.mul_add(b, c1);
    let p_lo = a.mul_add(b, c2 - p_hi);

    (p_lo - c3, p_hi - c1)
}

// Looks like for this to work you'll always have to do a conversion
fn int_full_product(a: f64, b: f64) -> (u64, u64) {
    let c1 = 2.0_f64.powi(104);
    let c2 = 2.0_f64.powi(104) + 2.0_f64.powi(52);
    let mask = 2_u64.pow(52) - 1;

    let p_hi = a.mul_add(b, c1);
    let p_lo = a.mul_add(b, c2 - p_hi);

    // vectorizable
    (p_lo.to_bits() & mask, p_hi.to_bits() & mask)
}

// These probably have to stay within the 11 bits
// Not totally convinced about the correctness of this one
fn make_initial(low_count: usize, high_count: usize) -> u64 {
    let val = high_count * 0x467 + low_count * 0x433;

    // Double if this is correct
    -((val as i64 & 0xFFF) << 52) as u64
}

const N: usize = 4;
// Can add up to 2^12 terms.
fn sampled_product(a: [f64; N], b: [f64; N]) -> [u64; 2 * N] {
    // TODO make these const across the code base
    // Does require doing a compile time computation
    let c1 = 2.0_f64.powi(104);
    let c2 = 2.0_f64.powi(104) + 2.0_f64.powi(52);

    let mut col_sums = [0; 2 * N];

    // Since our N is fixed this can be made a compile time constant
    // For a known size this is probably mixed in
    for i in 0..N {
        col_sums[i] = make_initial(i + 1, i);
        col_sums[2 * N - 1 - i] = make_initial(i, i + 1);
    }

    for i in 0..a.len() {
        for j in 0..b.len() {
            // These two multiplications can be shared between implementations
            // Shows what is common and what is different
            // This cannot be a vector operation
            let p_hi = a[i].mul_add(b[j], c1);
            let p_lo = a[i].mul_add(b[j], c2 - p_hi);
            // Looks like this could be vectorized
            col_sums[i + j + 1] = col_sums[i + j + 1].wrapping_add(p_hi.to_bits());
            col_sums[i + j] = col_sums[i + j].wrapping_add(p_lo.to_bits());
        }
    }
    col_sums
}

// Cost of getting it into the right from
// Seeing that there is exponentiation could it be profitable to keep in a certain range?
// Proper types
// TODO write a proper test suite
// Does rust have something like quickcheck?

// Do multiplication on the float representation and to convert it to 52-bit integer
// you add 2^52 and mask it.

// Looks like the value first needs to be converted to float
fn main() {
    // for i in 0..=10 {
    //     let num: f64 = f64::from_bits(i);
    //     print_float(num);
    // }

    let a = 2.0_f64.powi(52) + 4.;
    print_float(a);
    let b = 2.0_f64.powi(52) + 5.;
    print_float(b);
    print_float(a * b);

    print_float(4.0.mul_add(5.0, 2.0_f64.powi(52)));

    println!("int product: {:?}", full_product(4., 5.));

    let a = f64::from_bits(5 + 2_u64.pow(62));
    let b = f64::from_bits(4 + 2_u64.pow(62));
    print_float(a);
    print_float(b);

    let a = 2.0_f64.powi(50) + 1.0;
    let c = dpf_full_product(a, a);
    println!("{:?}", c);
    let d = int_full_product(a, a);
    println!("{:?}", d);
    [c.0, c.1].iter().for_each(|&num| print_float(num));

    let e = sampled_product([1., 0., 0., 2.0_f64.powi(51)], [1., 0., 0., 5.]);
    println!("{:?}", e);

    let e = sampled_product([5., 0., 0., 0.], [5., 0., 0., 0.]);
    println!("{:?}", e);
}

fn print_float(num: f64) {
    let bits = num.to_bits();
    let sign = (bits >> 63) & 1;
    let exponent = (bits >> 52) & 0x7FF;
    let mantissa = bits & 0xFFFFFFFFFFFFF;
    println!(
        // "{num}: \t sign: {:b}, exponent: {:011b}/{exponent}\t mantissa: {:052b}/{mantissa}",
        "{num}: \t sign: {:b}, exponent: {:011b}\t mantissa: {:052b}/{mantissa}",
        sign, exponent, mantissa
    );
}
