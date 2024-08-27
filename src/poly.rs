use crate::Error;
use ark_ff::{Field, PrimeField};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VariableType {
    Private,
    Public,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Variable {
    pub index: u32,
    pub variable_type: VariableType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Monomial<F: PrimeField> {
    pub variables: Vec<Variable>,
    pub coefficient: F,
}

impl<F: PrimeField> Monomial<F> {
    pub fn new(variables: Vec<Variable>, coefficient: F) -> Self {
        Self {
            variables,
            coefficient,
        }
    }

    pub fn degree(&self) -> usize {
        self.variables.len()
    }

    pub fn eval(&self, assignment: &HashMap<Variable, F>) -> F {
        let mut result = F::one();
        for variable in &self.variables {
            result *= assignment[variable];
        }
        result *= self.coefficient;
        result
    }
}

impl<F: PrimeField> FromStr for Monomial<F> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim().split('*').collect();
        let re_minus = Regex::new(r"-$").unwrap();
        let re_const = Regex::new(r"\d+$").unwrap();
        let re_public_var = Regex::new(r"x\d+$").unwrap();
        let re_private_var = Regex::new(r"z\d+$").unwrap();

        let mut coeff = None;
        let mut minus_coeff = F::one();
        let mut variables = Vec::new();
        for part in parts {
            if re_minus.is_match(part) {
                minus_coeff = F::zero() - F::one();
            }
            match part {
                _ if re_public_var.is_match(part) => {
                    let index = u32::from_str(&part[1..]).map_err(|_| {
                        Error::InvalidMonomialString(
                            s.to_string(),
                            format!("failed to parse {} as a public variable index", part),
                        )
                    })?;
                    variables.push(Variable {
                        index,
                        variable_type: VariableType::Public,
                    });
                }
                _ if re_private_var.is_match(part) => {
                    let index = u32::from_str(&part[1..]).map_err(|_| {
                        Error::InvalidMonomialString(
                            s.to_string(),
                            format!("failed to parse {} as a private variable index", part),
                        )
                    })?;
                    variables.push(Variable {
                        index,
                        variable_type: VariableType::Private,
                    });
                }
                _ if re_const.is_match(part) => {
                    if coeff.is_some() {
                        return Err(Error::InvalidMonomialString(
                            s.to_string(),
                            format!(
                                "coefficient {} already appeared, but {} appeared again",
                                coeff.unwrap(),
                                part
                            ),
                        ));
                    }
                    coeff = Some(F::from_str(part).map_err(|_| {
                        Error::InvalidMonomialString(
                            s.to_string(),
                            format!("failed to parse {} as a field element", part),
                        )
                    })?);
                }
                _ => {
                    return Err(Error::InvalidMonomialString(
                        s.to_string(),
                        format!(
                        "failed to parse {} as a constant, public variable, or private variable",
                        part
                    ),
                    ));
                }
            }
        }
        Ok(Self {
            variables,
            coefficient: minus_coeff * coeff.unwrap_or(F::one()),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Polynomial<F: PrimeField> {
    pub monomials: Vec<Monomial<F>>,
}

impl<F: PrimeField> Polynomial<F> {
    pub fn new(monomials: Vec<Monomial<F>>) -> Self {
        Self { monomials }
    }

    pub fn degree(&self) -> usize {
        self.monomials
            .iter()
            .map(|monomial| monomial.degree())
            .max()
            .unwrap_or(0)
    }

    pub fn eval(&self, assignment: &HashMap<Variable, F>) -> F {
        self.monomials
            .iter()
            .map(|monomial| monomial.eval(assignment))
            .sum()
    }
}

impl<F: PrimeField> FromStr for Polynomial<F> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('+').collect();
        if parts.is_empty() {
            return Err(Error::InvalidPolynomialString(
                s.to_string(),
                "no monomials found".to_string(),
            ));
        }
        let mut monomials = Vec::new();
        for part in parts {
            monomials.push(Monomial::from_str(part)?);
        }
        Ok(Self { monomials })
    }
}
