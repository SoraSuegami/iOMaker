use core::ops::{AddAssign, DivAssign, MulAssign, Neg, SubAssign};
use num_traits::{pow::Pow, Num};
//use rand::{distributions::Distribution, Rng};
use nalgebra_sparse::na::{ClosedAdd, ClosedMul, ClosedSub, Scalar as NaScalar};
use std::fmt::{Debug, Display};
use std::marker::Sync;
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
    + Neg<Output = Self>
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
