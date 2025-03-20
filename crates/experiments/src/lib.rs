#![feature(bigint_helper_methods)]
#![feature(portable_simd)]

pub mod acar;
pub mod arith;
pub mod domb;
pub mod emmart;
mod gen;
pub mod interleaved;
pub mod yuval;

pub const NP0: u64 = 0xc2e1f593efffffff;

pub const P: [u64; 4] = [
    0x43e1f593f0000001,
    0x2833e84879b97091,
    0xb85045b68181585d,
    0x30644e72e131a029,
];

// R^2 mod P
pub const R2: [u64; 4] = [
    0x1BB8E645AE216DA7,
    0x53FE3AB1E35C59E3,
    0x8C49833D53BB8085,
    0x0216D0B17F4E44A5,
];

pub const U52_NP0: u64 = 0x1F593EFFFFFFF;
pub const U52_R2: [u64; 5] = [
    0x0B852D16DA6F5,
    0xC621620CDDCE3,
    0xAF1B95343FFB6,
    0xC3C15E103E7C2,
    0x00281528FA122,
];

pub const U52_P: [u64; 5] = [
    0x1F593F0000001,
    0x4879B9709143E,
    0x181585D2833E8,
    0xA029B85045B68,
    0x030644E72E131,
];

pub const F52_P: [f64; 5] = [
    0x1F593F0000001_u64 as f64,
    0x4879B9709143E_u64 as f64,
    0x181585D2833E8_u64 as f64,
    0xA029B85045B68_u64 as f64,
    0x030644E72E131_u64 as f64,
];

/// Macro to extract a subarray from an array.
///
/// # Arguments
///
/// * `$t` - The source array
/// * `$b` - The starting index (base) in the source array
/// * `$l` - The length of the subarray to extract
///
/// This should be used over t[N..].try_into().unwrap() in getting a subarray. Using try_into+unwrap
/// introduces the eh_personality (exception handling)
///
/// # Example
///
/// ```
/// use montgomery_reduction::subarray;
/// let array = [1, 2, 3, 4, 5];
/// let sub = subarray!(array, 1, 3); // Creates [2, 3, 4]
/// ```
#[macro_export]
macro_rules! subarray{

    ($t:expr, $b: literal, $l: literal) => {
        {
        use seq_macro::seq;
        let t = $t;
        let mut s = [0;$l];

        // The compiler does not detect out-of-bounds when using `for` therefore `seq!` is used here
        seq!(i in 0..$l {
            s[i] = t[$b+i];
        });
        s
    }
    };
}
