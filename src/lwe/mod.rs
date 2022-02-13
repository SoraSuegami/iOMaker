use nalgebra::{DMatrix, DVector};
use num_bigint::BigUint;
use thiserror::Error;

mod parameter;
pub use parameter::*;
mod trapdoor;
pub use trapdoor::*;

use crate::math::Scalar;

#[derive(Error, Debug)]
pub enum LWEError<S: Scalar> {
    #[error(transparent)]
    LWEParameterError(#[from] LWEParameterError),
    #[error(transparent)]
    TrapdoorError(#[from] TrapdoorError<S>),
}

#[derive(Error, Debug, Clone)]
pub struct LWESecret<S: Scalar>(pub(crate) DVector<S>);

#[derive(Error, Debug, Clone)]
pub struct LWENoise<S: Scalar>(pub(crate) DVector<S>);
