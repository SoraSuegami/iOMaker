use super::*;
use crate::{poly::Polynomial, Error};
use ark_ec::{
    pairing::{Pairing, PairingOutput},
    Group,
};
use ark_ff::PrimeField;
use core::panic;
use nalgebra::{base::Scalar, ClosedAddAssign};
use nalgebra::{DMatrix, DVector};
use num_traits::{One, Zero};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::{
    ops::{Add, AddAssign, Mul, MulAssign, Neg},
    path::Path,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PhfeFunc<F: PrimeField> {
    pub num_public_vars: usize,
    pub num_private_vars1: usize,
    pub num_private_vars2: usize,
    pub polys: Vec<Polynomial<F>>,
    pub dfx_coeffs: Vec<Polynomial<F>>,
    pub l0: DMatrix<F>,
    pub l1: DMatrix<F>,
}

impl<F: PrimeField> PhfeFunc<F> {
    pub fn new(
        num_public_vars: usize,
        num_private_vars1: usize,
        num_private_vars2: usize,
        polys: Vec<Polynomial<F>>,
        dfx_coeffs: Vec<Polynomial<F>>,
        l0: DMatrix<F>,
        l1: DMatrix<F>,
    ) -> Self {
        Self {
            num_public_vars,
            num_private_vars1,
            num_private_vars2,
            polys,
            dfx_coeffs,
            l0,
            l1,
        }
    }

    pub fn from_str(s: &str) -> Result<Self, Error> {
        let json: PhfeFuncJson = serde_json::from_str(s)?;
        Self::from_json(json)
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let json: PhfeFuncJson = serde_json::from_reader(std::fs::File::open(path)?)?;
        Self::from_json(json)
    }

    pub fn from_json(json: PhfeFuncJson) -> Result<Self, Error> {
        let polys = json
            .polys
            .iter()
            .map(|poly| Polynomial::from_str(poly))
            .collect::<Result<_, _>>()?;
        let dfx_coeffs = json
            .dfx_coeffs
            .iter()
            .map(|poly| Polynomial::from_str(poly))
            .collect::<Result<_, _>>()?;
        let l0_rows = json.l0.len();
        let l0_cols = json.l0[0].len();
        let mut l0_vec = Vec::with_capacity(l0_rows * l0_cols);
        for i in 0..l0_rows {
            for j in 0..l0_cols {
                l0_vec.push(parse_field_str(&json.l0[i][j])?);
            }
        }
        let l0 = DMatrix::from_row_slice(l0_rows, l0_cols, &l0_vec);
        let l1_rows = json.l1.len();
        let l1_cols = json.l1[0].len();
        let mut l1_vec = Vec::with_capacity(l1_rows * l1_cols);
        for i in 0..l1_rows {
            for j in 0..l1_cols {
                l1_vec.push(parse_field_str(&json.l1[i][j])?);
            }
        }
        let l1 = DMatrix::from_row_slice(l1_rows, l1_cols, &l1_vec);

        Ok(Self {
            num_public_vars: json.num_public_vars,
            num_private_vars1: json.num_private_vars1,
            num_private_vars2: json.num_private_vars2,
            polys,
            dfx_coeffs,
            l0,
            l1,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PhfeFuncJson {
    pub num_public_vars: usize,
    pub num_private_vars1: usize,
    pub num_private_vars2: usize,
    pub polys: Vec<String>,
    pub dfx_coeffs: Vec<String>,
    pub l0: Vec<Vec<String>>,
    pub l1: Vec<Vec<String>>,
}
