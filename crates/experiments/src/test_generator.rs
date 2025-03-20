// Test generators
#![cfg(test)]
use quickcheck::Arbitrary;

use crate::emmart::{MASK48, MASK52};

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct U256b64(pub [u64; 4]);
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct U256b52(pub [u64; 5]);

impl From<U256b64> for U256b52 {
    fn from(u: U256b64) -> Self {
        let U256b64(limbs) = u;
        let [l0, l1, l2, l3] = limbs;
        U256b52([
            l0 & MASK52,
            ((l0 >> 52) | (l1 << 12)) & MASK52,
            ((l1 >> 40) | (l2 << 24)) & MASK52,
            ((l2 >> 28) | (l3 << 36)) & MASK52,
            l3 >> 16,
        ])
    }
}

impl From<U256b52> for U256b64 {
    fn from(u: U256b52) -> Self {
        let U256b52(limbs) = u;
        let [l0, l1, l2, l3, l4] = limbs;
        U256b64([
            l0 | (l1 << 52),
            ((l1 >> 12) | (l2 << 40)),
            ((l2 >> 24) | (l3 << 28)),
            ((l3 >> 36) | (l4 << 16)),
        ])
    }
}

impl Arbitrary for U256b52 {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        U256b52([
            u64::arbitrary(g) & MASK52,
            u64::arbitrary(g) & MASK52,
            u64::arbitrary(g) & MASK52,
            u64::arbitrary(g) & MASK52,
            u64::arbitrary(g) & MASK48,
        ])
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let a = self.0;

        if a.iter().all(|&ai| ai == 0) {
            return Box::new(std::iter::empty());
        }

        // Create vector of shrunk values
        let mut shrunk = Vec::new();

        // Add zero as the smallest possible value
        shrunk.push(Self(Default::default()));

        for (i, &elem) in a.iter().enumerate() {
            if elem != 0 {
                let mut zero_limb = a.clone();
                zero_limb[i] = 0;
                let mut half_limb = a.clone();
                half_limb[i] = elem >> 1;
                shrunk.push(Self(zero_limb));
                shrunk.push(Self(half_limb));
            }
        }

        Box::new(shrunk.into_iter())
    }
}

impl Arbitrary for U256b64 {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        U256b64([
            u64::arbitrary(g),
            u64::arbitrary(g),
            u64::arbitrary(g),
            u64::arbitrary(g),
        ])
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let a = self.0;

        if a.iter().all(|&ai| ai == 0) {
            return Box::new(std::iter::empty());
        }

        // Create vector of shrunk values
        let mut shrunk = Vec::new();

        // Add zero as the smallest possible value
        shrunk.push(Self(Default::default()));

        for (i, &elem) in a.iter().enumerate() {
            if elem != 0 {
                let mut zero_limb = a.clone();
                zero_limb[i] = 0;
                let mut half_limb = a.clone();
                half_limb[i] = elem >> 1;
                shrunk.push(Self(zero_limb));
                shrunk.push(Self(half_limb));
            }
        }

        Box::new(shrunk.into_iter())
    }
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use super::{U256b52, U256b64};

    #[quickcheck]
    fn conv64_52(a: U256b64) -> bool {
        let converted = U256b52::from(a);
        let reconverted = U256b64::from(converted);

        a == reconverted
    }

    #[quickcheck]
    fn conv52_64(a: U256b52) -> bool {
        let converted = U256b64::from(a);
        let reconverted = U256b52::from(converted);

        a == reconverted
    }
}
