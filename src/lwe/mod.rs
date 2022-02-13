use nalgebra::{DMatrix, DVector};
use num_bigint::BigUint;
use thiserror::Error;

mod parameter;
pub use parameter::*;
mod trapdoor;
pub use trapdoor::*;

use crate::math::{Sampler, Scalar, UniformSampler};
use rand::{
    distributions::{Distribution, Uniform},
    Rng,
};

#[derive(Error, Debug)]
pub enum LWEError<S: Scalar> {
    #[error(transparent)]
    LWEParameterError(#[from] LWEParameterError),
    #[error(transparent)]
    TrapdoorError(#[from] TrapdoorError<S>),
}

#[derive(Error, Debug, Clone)]
pub struct LWESecret<S: Scalar>(pub(crate) DVector<S>);

impl<S: Scalar> LWESecret<S> {
    pub fn gen_uniform<D: Distribution<S>, R: Rng + ?Sized>(
        param: &LWEParameter<S>,
        sampler: &UniformSampler<S, D>,
        rng: &mut R,
    ) -> Self {
        let vector = sampler.sample_vector(rng, param.secret_size);
        Self(vector)
    }
}

#[derive(Error, Debug, Clone)]
pub struct LWENoise<S: Scalar>(pub(crate) DVector<S>);
