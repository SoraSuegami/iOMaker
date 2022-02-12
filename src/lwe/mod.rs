use num_bigint::BigUint;
use thiserror::Error;

mod parameter;
pub use parameter::*;

#[derive(Error, Debug)]
pub enum LWEError {
    #[error("Invalid LWE Parameter. log_q_size:{0}, secret_size:{1}, mask_size:{2}")]
    InvalidLWEParam(usize, usize, usize),
}
