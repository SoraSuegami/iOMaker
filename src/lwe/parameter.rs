use super::LWEError;
use crate::math::Scalar;
use num_bigint::BigUint;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LWEParameterError {
    #[error("Invalid LWE Parameter. log_q_size:{0}, secret_size:{1}, mask_size:{2}")]
    InvalidLWEParam(usize, usize, usize),
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LWEParameter<S: Scalar> {
    pub q_order: BigUint,
    pub log_q_size: usize,  //l=log_2 q
    pub secret_size: usize, //n
    pub mask_size: usize,   //m
    pub threshold: S,
}

impl<S: Scalar> LWEParameter<S> {
    pub fn new_checked(
        q_order: BigUint,
        log_q_size: usize,
        secret_size: usize,
        mask_size: usize,
        threshold: S,
    ) -> Result<Self, LWEParameterError> {
        let param = Self {
            q_order,
            log_q_size,
            secret_size,
            mask_size,
            threshold,
        };
        if param.verify_param() != true {
            return Err(LWEParameterError::InvalidLWEParam(
                log_q_size,
                secret_size,
                mask_size,
            ));
        }
        Ok(param)
    }

    pub fn new_unchecked(
        q_order: BigUint,
        log_q_size: usize,
        secret_size: usize,
        mask_size: usize,
        threshold: S,
    ) -> Self {
        Self {
            q_order,
            log_q_size,
            secret_size,
            mask_size,
            threshold,
        }
    }

    //[TODO]: verify the given parameter.
    pub fn verify_param(&self) -> bool {
        true
    }
}
