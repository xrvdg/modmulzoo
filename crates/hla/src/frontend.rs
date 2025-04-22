use std::{marker::PhantomData, mem};

use crate::FreshRegister;

pub struct Reg<T> {
    pub(crate) reg: FreshRegister,
    _marker: PhantomData<T>,
}

/// Register that contains a pointer of type T
pub struct PointerReg<T> {
    pub(crate) reg: FreshRegister,
    // offset in bytes as that allows for conversions between
    // x and w without having to recalculate the offset
    pub(crate) offset: usize,
    _marker: PhantomData<T>,
}

impl<T> PointerReg<T> {
    pub fn new(reg: u64) -> Self {
        Self {
            reg: FreshRegister(reg),
            offset: 0,
            _marker: PhantomData,
        }
    }
}

impl<T, const N: usize> PointerReg<[T; N]> {
    pub fn get(&self, index: usize) -> PointerReg<T> {
        assert!(index < N, "out-of-bounds access");

        PointerReg {
            reg: self.reg,
            offset: mem::size_of::<T>() * index,
            _marker: PhantomData,
        }
    }
}

pub trait SIMD {}

impl<T, const N: usize> SIMD for Reg<Simd<T, N>> {}
impl<T: SIMD, const I: u8> SIMD for Idx<T, I> {}

/// Define the struct ourself as to not have to import it
pub struct Simd<T, const N: usize>(PhantomData<T>);
pub struct Idx<T, const I: u8>(pub(crate) T);
pub struct Sized<T, const L: u8>(pub(crate) T);
pub type SizedIdx<T, const L: u8, const I: u8> = Sized<Idx<T, I>, L>;

pub const D: u8 = 2;

pub trait Reg64Bit {}
impl Reg64Bit for u64 {}
impl Reg64Bit for f64 {}

impl<T> Reg<T> {
    pub(crate) fn new(reg: u64) -> Self {
        Self {
            reg: reg.into(),
            _marker: Default::default(),
        }
    }
}

impl Reg<f64> {
    pub fn as_simd(&self) -> &Reg<Simd<f64, 2>> {
        unsafe { std::mem::transmute(self) }
    }
}

impl<T> Reg<Simd<T, 2>> {
    pub fn into_<D>(self) -> Reg<Simd<D, 2>> {
        unsafe { std::mem::transmute(self) }
    }

    pub fn as_<D>(&self) -> &Reg<Simd<D, 2>> {
        unsafe { std::mem::transmute(self) }
    }

    pub fn _0(&self) -> &Idx<Reg<Simd<T, 2>>, 0> {
        unsafe { std::mem::transmute(self) }
    }

    pub fn _1(&self) -> &Idx<Reg<Simd<T, 2>>, 1> {
        unsafe { std::mem::transmute(self) }
    }

    pub fn _d0(&self) -> &SizedIdx<Reg<Simd<T, 2>>, D, 0> {
        unsafe { std::mem::transmute(self) }
    }

    pub fn _d1(&self) -> &SizedIdx<Reg<Simd<T, 2>>, D, 1> {
        unsafe { std::mem::transmute(self) }
    }
}

impl std::fmt::Display for Reg<u64> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x{}", self.reg)
    }
}

impl std::fmt::Debug for Reg<u64> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x{}", self.reg)
    }
}
