use crate::scalar::Scalar;
use nalgebra::{DMatrix, DVector};
use num_bigint::BigUint;
use thiserror::Error;
use std::marker::PhantomData;
use rand::{distributions::{Distribution,Uniform}, Rng};
use super::{LWEError,LWEParameter};

pub trait Sampler<S:Scalar> {
    fn sample_scalar<R: Rng + ?Sized>(&self, rng: &mut R) -> S;

    fn sample_vector<R: Rng + ?Sized>(&self, rng: &mut R, n:usize) -> DVector<S> {
        DVector::from_fn(n,|_,_| Self::sample_scalar(rng))
    }

    fn sample_matrix<R: Rng + ?Sized>(&self, rng: &mut R, n_row:usize, n_col:usize) -> DMatrix<S> {
        DMatrix::from_fn(n_row, n_col, |_,_| Self::sample_scalar(rng))
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UniformSampler<S:Scalar, D:Distribution<S>> {
    dist: D,
    _s:PhantomData<S>
}

impl<S:Scalar, D:Distribution<S>> Sampler for UniformSampler<S, D> {
    fn sample_scalar<R: Rng + ?Sized>(rng: &mut R) -> Scalar<F> {
        self.dist.sample(rng)
    }
}

impl<S:Scalar, D:Distribution<S>> UniformSampler<S, D> {
    pub fn new(dist:D) -> Self {
        Self {
            dist,
            _s:PhantomData
        }
    }
}

pub type UniformSamplerU32 = UniformSampler<u32, Uniform>;
pub type UniformSamplerU64 = UniformSampler<u64, Uniform>;
/*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GaussianSampler<S:Scalar, D:Distribution<S>> {
    dist: D,
    _f:PhantomData<F>
}

impl<F:Filed> Sampler for GaussianSampler<F> {
    fn sample_scalar<R: Rng + ?Sized>(rng: &mut R) -> Scalar<F> {
        Scalar(F::random(rng))
    }
}*/


/* 
pub struct LWESample<F:PrimeField> {
    pub matrix_a: DMatrix<Scalar<F>>,
    pub vector_b: DVector<Scalar<F>>,
    //pub(crate) secret: DVector<Scalar<F>>
}


impl<F:PrimeField> LWESample<F> {
    pub fn recover_error(&self, secret: &DVector<Scalar<F>>) -> DVector<Scalar<F>> {
        self.vector_b - secret*self.matrix_a
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LWEDistribution {
    param: LWEParameter
}

impl<F: PrimeField> Distribution<LWESample<F>> for LWEDistribution {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> LWESample<F> {
        let dist = ScalarDistribution;
        let matrix_a = DMatrix::from_distribution(self.param.n_row, self.param.n_col, &dist, &mut rng);
        let vector_b = DVector::from_distribution(self.param.n_row, &dist, &mut rng);

        Scalar(F::random(rng))
    }
}
*/