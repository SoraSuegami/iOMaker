use ark_ff::{Field, PrimeField};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Variable(pub u32);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Monomial<F: PrimeField> {
    pub variables: Vec<Variable>,
    pub coefficient: F,
    pub index: usize,
}

impl<F: PrimeField> Monomial<F> {
    pub fn new(variables: Vec<Variable>, coefficient: F, index: usize) -> Self {
        Self {
            variables,
            coefficient,
            index,
        }
    }

    pub fn degree(&self) -> usize {
        self.variables.len()
    }

    pub fn eval(&self, assignment: &[F]) -> F {
        let mut result = F::one();
        for variable in &self.variables {
            result *= assignment[variable.0 as usize];
        }
        result *= self.coefficient;
        result
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

    pub fn eval(&self, assignment: &[F]) -> F {
        self.monomials
            .iter()
            .map(|monomial| monomial.eval(assignment))
            .sum()
    }
}
