use serde_json;
use thiserror;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("The given string {0} is invalid for field element.")]
    InvalidFieldString(String),
    #[error("The given string {0} is invalid for monomial. Reason: {1}")]
    InvalidMonomialString(String, String),
    #[error("The given string {0} is invalid for polynomial. Reason: {1}")]
    InvalidPolynomialString(String, String),
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}
