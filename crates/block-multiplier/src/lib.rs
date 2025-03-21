pub mod constants;
pub mod test_generator;

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
