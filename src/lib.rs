#![feature(bigint_helper_methods)]

pub mod acar;
mod emmart;

// TODO This should already flow out the above mod?
pub use acar::{cios, cios_opt, fios, sos};
pub use emmart::{
    sampled_product, sampled_product_masked, school_method, set_round_to_zero, U256b52, U256b64,
};

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
    let out_len = (total_bits + destination_size as usize - 1) / destination_size as usize;
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
        let new_limb = (bit_buffer & dest_mask as u128) as u64;
        output.push(new_limb);
    }

    output
}
