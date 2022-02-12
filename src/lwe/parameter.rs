use super::LWEError;
use crate::math::Scalar;
use ff::{Field, PrimeField};
use nalgebra::{DMatrix, DVector};
use num_bigint::BigUint;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LWEParameter {
    q_order: BigUint,
    log_q_size: usize,  //l=log_2 q
    secret_size: usize, //n
    mask_size: usize,   //m
}

impl LWEParameter {
    pub fn new_checked(
        q_order: BigUint,
        log_q_size: usize,
        secret_size: usize,
        mask_size: usize,
    ) -> Result<Self, LWEError> {
        let param = Self {
            q_order,
            log_q_size,
            secret_size,
            mask_size,
        };
        if param.verify_param() != true {
            return Err(LWEError::InvalidLWEParam(
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
    ) -> Self {
        Self {
            q_order,
            log_q_size,
            secret_size,
            mask_size,
        }
    }

    //[TODO]: verify the given parameter.
    pub fn verify_param(&self) -> bool {
        true
    }
}