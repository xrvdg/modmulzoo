#![feature(bigint_helper_methods)]

mod acar;
mod emmart;

pub use acar::{cios, cios_opt, fios, sos};
pub use emmart::{sampled_product_masked, school_method, set_round_to_zero, U256b52, U256b64};

pub const NP0: u64 = 0xc2e1f593efffffff;
pub const P: [u64; 4] = [
    0x43e1f593f0000001,
    0x2833e84879b97091,
    0xb85045b68181585d,
    0x30644e72e131a029,
];
