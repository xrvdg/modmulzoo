#![feature(bigint_helper_methods)]

pub mod acar;
pub mod emmart;

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

pub fn convert_limb_sizes(
    input: &[u64],
    total_bits: usize,
    source_size: u8,
    destination_size: u8,
) -> Vec<u64> {
    assert!(source_size <= 64, "Source limb size must be <= 64 bits");
    assert!(
        destination_size <= 64,
        "Destination limb size must be <= 64 bits"
    );

    // If input is empty, return empty vector
    if input.is_empty() {
        return Vec::new();
    }
    // Calculate total bits and required output capacity
    let out_len = total_bits.div_ceil(destination_size as usize);
    let mut output = Vec::with_capacity(out_len);

    // Create mask for destination size
    let dest_mask = (1u128 << destination_size) - 1;

    // Track bits we're currently processing
    let mut bit_buffer: u128 = 0; // Use u128 to handle overflow during shifting
    let mut bits_in_buffer = 0u32;

    // Process each input limb
    for &limb in input {
        // Add new bits to buffer
        bit_buffer |= (limb as u128) << bits_in_buffer;
        bits_in_buffer += source_size as u32;

        // Extract complete destination-sized chunks
        while bits_in_buffer >= destination_size as u32 {
            let new_limb = (bit_buffer & dest_mask) as u64;
            output.push(new_limb);
            bit_buffer >>= destination_size;
            bits_in_buffer -= destination_size as u32;
        }
    }

    // Handle remaining bits if any
    if bits_in_buffer > 0 {
        let new_limb = (bit_buffer & dest_mask) as u64;
        output.push(new_limb);
    }

    output
}

pub fn subtraction_step<const N: usize>(a: [u64; N], b: [u64; N]) -> [u64; N] {
    let mut borrow: i64 = 0;
    let mut c = [0; N];
    for i in 0..N {
        let tmp = a[i] as i128 - b[i] as i128 + borrow as i128;
        c[i] = tmp as u64;
        borrow = (tmp >> 64) as i64
    }

    if borrow != 0 {
        a
    } else {
        c
    }
}

const MASK52: u64 = 2_u64.pow(52) - 1;
const MASK48: u64 = 2_u64.pow(48) - 1;

pub fn subtraction_step_u52<const N: usize>(a: [u64; N], b: [u64; N]) -> [u64; N] {
    let mut borrow: i64 = 0;
    let mut c = [0; N];
    for i in 0..N {
        let tmp = a[i] as i128 - b[i] as i128 + borrow as i128;
        c[i] = (tmp as u64) & MASK52;
        borrow = (tmp >> 52) as i64
    }

    if borrow != 0 {
        a
    } else {
        c
    }
}
