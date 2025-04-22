use std::{marker::PhantomData, mem};

use crate::{FreshRegister, ReifyRegister};

/// Represents a single hardware registers by modelling it as
/// a fresh variable.
pub struct Reg<T> {
    pub(crate) reg: FreshRegister,
    _marker: PhantomData<T>,
}

/// Represents a single hardware register that contains a pointer to a type T
/// and an offset from that pointer.
pub struct PointerReg<'a, T> {
    pub(crate) reg: &'a Reg<T>,
    // offset in bytes as that allows for conversions between
    // x and w without having to recalculate the offset
    pub(crate) offset: usize,
    _marker: PhantomData<T>,
}

impl<'a, T, const N: usize> Reg<*mut [T; N]> {
    pub fn get(&self, index: usize) -> PointerReg<*mut T> {
        assert!(index < N, "out-of-bounds access");

        PointerReg {
            reg: self.as_pointer(),
            offset: mem::size_of::<T>() * index,
            _marker: PhantomData,
        }
    }
}

impl<'a, T, const N: usize> Reg<*const [T; N]> {
    pub fn get(&self, index: usize) -> PointerReg<*const T> {
        assert!(index < N, "out-of-bounds access");

        PointerReg {
            reg: self.as_(),
            offset: mem::size_of::<T>() * index,
            _marker: PhantomData,
        }
    }
}

pub trait Pointer: ReifyRegister {}
impl<T> Pointer for PointerReg<'_, T> {}
impl<T> Pointer for Reg<*mut T> {}
impl<T> Pointer for Reg<*const T> {}

pub trait MutablePointer: Pointer {}
impl<T> MutablePointer for PointerReg<'_, *mut T> {}
impl<T> MutablePointer for Reg<*mut T> {}

// fmla.2d supports both a vector or vector lane as multiplier
pub trait SIMD {}
impl<T, const N: usize> SIMD for Reg<Simd<T, N>> {}
impl<T: SIMD, const I: u8> SIMD for Idx<T, I> {}

pub struct Simd<T, const N: usize>(PhantomData<T>);
pub struct Idx<T, const I: u8>(pub(crate) T);
pub struct Sized<T, const L: u8>(pub(crate) T);
pub type SizedIdx<T, const L: u8, const I: u8> = Sized<Idx<T, I>, L>;

/// When inspecting a vector as a D it has 2 elements.
/// Defined as such due to restrictions on const generics.
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

    // Depending on the instruction a vector lane needs
    // to be addressed by either it's size and lane or
    // just it's lane.

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

impl<T, const N: usize> Reg<*mut [T; N]> {
    pub fn as_pointer(&self) -> &Reg<*mut T> {
        unsafe { std::mem::transmute(self) }
    }
}

impl<T> Reg<*mut T> {
    pub fn as_(&self) -> &Reg<*const T> {
        unsafe { std::mem::transmute(self) }
    }
}

impl<T, const N: usize> Reg<*const [T; N]> {
    pub fn as_(&self) -> &Reg<*const T> {
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
