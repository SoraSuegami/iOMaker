use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign, Neg};
use ff::{Field, PrimeField};
use num_bigint::BigUint;
use num_traits::{pow::Pow, Num};
use rand::{distributions::Distribution, Rng};
use std::fmt::{Debug, Display};
use std::marker::Sync;
use nalgebra_sparse::na::{Scalar as NaScalar,ClosedAdd,ClosedSub,ClosedMul};
pub trait Scalar:
    Debug
    + Sized
    + Send
    + Sync
    + Clone
    + Copy
    + Display
    + PartialEq
    + PartialOrd
    + Eq
    + Default
    + Num
    + Neg<Output=Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + Pow<usize, Output = Self>
    + NaScalar
    + ClosedAdd
    + ClosedSub
    + ClosedMul
{
}

impl Scalar for i32 {}
impl Scalar for i64 {}
