#[cfg(target_os = "macos")]
#[link(name = "Accelerate", kind = "framework")]
extern "C" {
    fn vDSP_vaddD(
        input1: *const f64,
        stride1: i32,
        input2: *const f64,
        stride2: i32,
        output: *mut f64,
        strideResult: i32,
        size: i32,
    );

    fn vDSP_vmulD(
        input1: *const f64,
        stride1: i32,
        input2: *const f64,
        stride2: i32,
        output: *mut f64,
        strideResult: i32,
        size: i32,
    );
}

#[cfg(target_os = "macos")]
fn main() {
    // Create test vectors
    let vector1: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let vector2: Vec<f64> = vec![2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    let mut result_add = vec![0.0; 8];
    let mut result_mul = vec![0.0; 8];

    unsafe {
        // Perform vector addition using Accelerate
        vDSP_vaddD(
            vector1.as_ptr(),
            1,
            vector2.as_ptr(),
            1,
            result_add.as_mut_ptr(),
            1,
            8,
        );

        // Perform vector multiplication using Accelerate
        vDSP_vmulD(
            vector1.as_ptr(),
            1,
            vector2.as_ptr(),
            1,
            result_mul.as_mut_ptr(),
            1,
            8,
        );
    }

    println!("Vector 1: {:?}", vector1);
    println!("Vector 2: {:?}", vector2);
    println!("Vector Addition Result: {:?}", result_add);
    println!("Vector Multiplication Result: {:?}", result_mul);
}

#[cfg(not(target_os = "macos"))]
fn main() {}
