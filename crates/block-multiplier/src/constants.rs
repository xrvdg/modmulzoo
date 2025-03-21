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

pub const MASK52: u64 = 2_u64.pow(52) - 1;
pub const MASK48: u64 = 2_u64.pow(48) - 1;
