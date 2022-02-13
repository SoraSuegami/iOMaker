use super::{LWEError, LWENoise, LWEParameter, LWESecret};
use crate::math::Scalar;
use nalgebra_sparse::na::*;
use nalgebra_sparse::*;
use rayon::prelude::*;
use std::marker::PhantomData;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TrapdoorError<S: Scalar> {
    #[error("Fail to inverse the LWE sample with a trapdoor. trapdoor_type:{0}, sample:{1}")]
    InverseLWEError(&'static str, DVector<S>),
    #[error("The value of a sparse matrix at ({0}, {1}) is zero.")]
    ZeroValue(usize, usize),
    #[error("The trapdoor is not provided.")]
    TrapdoorNotProvided,
}

pub trait Trapdoor<S: Scalar> {
    const trapdoor_type: &'static str;
    fn gen_mask_matrix(&self) -> Result<CooMatrix<S>, TrapdoorError<S>>;
    fn gen_trapdoor_matrix(&self) -> Result<Option<CooMatrix<S>>, TrapdoorError<S>>;
    fn inverse_lwe(
        &self,
        sample: &DVector<S>,
        trapdoor: Option<&CooMatrix<S>>,
    ) -> Result<(DVector<S>, DVector<S>), TrapdoorError<S>>;
}

#[derive(Debug, Clone)]
pub struct Smooth2GadgetTrapdoor<S: Scalar> {
    param: LWEParameter<S>,
}

impl<S: 'static + Scalar> Trapdoor<S> for Smooth2GadgetTrapdoor<S> {
    const trapdoor_type: &'static str = "Smooth2GadgetTrapdoor";

    fn gen_mask_matrix(&self) -> Result<CooMatrix<S>, TrapdoorError<S>> {
        let n_row = self.param.secret_size;
        let n_col = self.param.secret_size * self.param.log_q_size;
        let two = S::one() + S::one();
        let mut coo = CooMatrix::new(n_row, n_col);
        for i in 0..n_row {
            let mut pow_two = S::one();
            for j in 0..self.param.log_q_size {
                coo.push(i, i * self.param.log_q_size + j, pow_two);
                pow_two *= two;
            }
        }
        Ok(coo)
    }

    fn gen_trapdoor_matrix(&self) -> Result<Option<CooMatrix<S>>, TrapdoorError<S>> {
        Ok(None)
    }

    fn inverse_lwe(
        &self,
        sample: &DVector<S>,
        trapdoor: Option<&CooMatrix<S>>,
    ) -> Result<(DVector<S>, DVector<S>), TrapdoorError<S>> {
        let log_q_size = self.param.log_q_size;
        if sample.nrows() != log_q_size * self.param.secret_size {
            return Err(TrapdoorError::InverseLWEError(
                Self::trapdoor_type,
                sample.clone(),
            ));
        }
        let mut vec_samples = Vec::new();
        for i in 0..self.param.secret_size {
            let chunk = sample.rows(i * log_q_size, log_q_size);
            vec_samples.extend_from_slice(chunk.as_slice());
        }
        let two = S::one() + S::one();
        let inversed = (0..self.param.secret_size)
            .into_par_iter()
            .map(|i| {
                let mut secret = S::zero();
                let mut errors = Vec::new();
                for j in 0..log_q_size {
                    let sample_val = vec_samples[self.param.secret_size * i + log_q_size - 1 - j];
                    if (sample_val - secret) >= self.param.threshold {
                        secret += two.pow(log_q_size - 1 - i);
                    }
                    errors.push(sample_val - secret);
                }
                errors.reverse();
                (secret, errors)
            })
            .collect::<Vec<(S, Vec<S>)>>();
        let mut secret_vec = DVector::repeat(self.param.secret_size, S::zero());
        let error_vec = DVector::repeat(self.param.secret_size * log_q_size, S::zero());
        for i in 0..self.param.secret_size {
            secret_vec.push(inversed[i].0);
            for j in 0..log_q_size {
                error_vec.push(inversed[i].1[j]);
            }
        }
        Ok((secret_vec, error_vec))
    }
}

impl<S: 'static + Scalar> Smooth2GadgetTrapdoor<S> {
    pub fn new(param: LWEParameter<S>) -> Self {
        Self { param }
    }
}

#[derive(Debug, Clone)]
pub struct Smooth2PrivateTrapdoor<S: Scalar> {
    param: LWEParameter<S>,
    matrix_a_bar: DMatrix<S>, //n * m~
    matrix_r: DMatrix<S>,     //m~ * nl
}

impl<S: 'static + Scalar> Trapdoor<S> for Smooth2PrivateTrapdoor<S> {
    const trapdoor_type: &'static str = "Smooth2PrivateTrapdoor";

    fn gen_mask_matrix(&self) -> Result<CooMatrix<S>, TrapdoorError<S>> {
        let n = self.param.secret_size;
        let m_bar = self.matrix_a_bar.ncols();
        let log_q_size = self.param.log_q_size;
        let m = m_bar + n * log_q_size;
        let matrix_ar = &self.matrix_a_bar * &self.matrix_r;
        let mut mask = CooMatrix::new(n, m);
        let pub_td = Smooth2GadgetTrapdoor::new(self.param.clone());
        let gadget = CsrMatrix::from(&pub_td.gen_mask_matrix()?);
        for i in 0..n {
            for j in 0..m_bar {
                let val: S = self.matrix_a_bar[(i, j)];
                mask.push(i, j, val);
            }
            for k in m_bar..m {
                match gadget.index_entry(i, k - m_bar) {
                    SparseEntry::Zero => {
                        return Err(TrapdoorError::ZeroValue(i, k - m_bar));
                    }
                    SparseEntry::NonZero(g_val) => {
                        let val = *g_val - matrix_ar[(i, k - m_bar)];
                        mask.push(i, k, val);
                    }
                }
            }
        }
        Ok(mask)
    }

    fn gen_trapdoor_matrix(&self) -> Result<Option<CooMatrix<S>>, TrapdoorError<S>> {
        let n = self.param.secret_size;
        let m_bar = self.matrix_a_bar.ncols();
        let log_q_size = self.param.log_q_size;
        let m = m_bar + n * log_q_size;

        let mut trapdoor = CooMatrix::new(m, n * log_q_size);
        for i in 0..m_bar {
            for j in 0..(n * log_q_size) {
                trapdoor.push(i, j, self.matrix_r[(i, j)]);
            }
        }
        for i in 0..(n * log_q_size) {
            trapdoor.push(i, i, S::one());
        }
        Ok(Some(trapdoor))
    }

    fn inverse_lwe(
        &self,
        sample: &DVector<S>,
        trapdoor: Option<&CooMatrix<S>>,
    ) -> Result<(DVector<S>, DVector<S>), TrapdoorError<S>> {
        let sample = CsrMatrix::from(&sample.transpose());
        let trapdoor = trapdoor.ok_or(TrapdoorError::TrapdoorNotProvided)?;
        let trapdoor = CsrMatrix::from(trapdoor);
        let new_sample = sample * trapdoor;
        let new_sample = DMatrix::from(&new_sample);
        let new_sample = new_sample.row(1);
        let pub_td = Smooth2GadgetTrapdoor::new(self.param.clone());
        pub_td.inverse_lwe(&new_sample.transpose(), None)
    }
}
