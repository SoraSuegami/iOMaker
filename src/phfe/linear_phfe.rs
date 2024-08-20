use super::*;
use crate::poly::{Monomial, Polynomial};
use ark_ec::{pairing::Pairing, PairingFriendlyCycle};
use ark_ff::PrimeField;
use ark_ff::UniformRand;
use nalgebra::{DMatrix, DVector};
use rand::Rng;
use std::marker::PhantomData;
use utils::matrix_to_g1;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LinearPhfeMpk<C: Pairing> {
    pub a_matrix: DMatrix<C::G1>,
    pub aw_matrix: DMatrix<C::G1>,
    pub au_matrix: DMatrix<C::G1>,
    pub av_matrix: DMatrix<C::G1>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LinearPhfeMsk<C: Pairing> {
    pub w_matrix: DMatrix<C::ScalarField>,
    pub u_matrix: DMatrix<C::ScalarField>,
    pub v_matrix: DMatrix<C::ScalarField>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LinearPhfeCt<C: Pairing> {
    pub sa_vec: DVector<C::G1>,
    pub z_vec: DMatrix<C::G1>,
    pub x_vec: DMatrix<C::G1>,
    pub x: DVector<C::ScalarField>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LinearPhfeFunc<F: PrimeField> {
    pub l_0: DMatrix<F>,
    pub l_1: DMatrix<Monomial<F>>,
    pub dfx_coeffs: Vec<Polynomial<F>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LinearPhfeFsk<C: Pairing> {
    pub t_matrix: DMatrix<C::G2>,
    pub l1_matrix: DMatrix<C::G2>,
    pub l0_matrix: DMatrix<C::G2>,
    pub r_matrix: DMatrix<C::G2>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LinearPhfe<C: Pairing> {
    pub num_public_vars: usize,
    pub num_private_vars: usize,
    pub k: usize,
    _c: PhantomData<C>,
}

impl<C: Pairing> LinearPhfe<C> {
    pub fn new(num_public_vars: usize, num_private_vars: usize, k: usize) -> Self {
        Self {
            num_public_vars,
            num_private_vars,
            k,
            _c: PhantomData,
        }
    }

    pub fn setup<R: Rng>(&self, rng: &mut R) -> (LinearPhfeMpk<C>, LinearPhfeMsk<C>) {
        let a_matrix = DMatrix::<C::ScalarField>::from_fn(self.k, self.k + 1, |_, _| {
            C::ScalarField::rand(rng)
        });
        let w_matrix = DMatrix::<C::ScalarField>::from_fn(self.k, self.num_private_vars, |_, _| {
            C::ScalarField::rand(rng)
        });
        let u_matrix = DMatrix::<C::ScalarField>::from_fn(
            self.k + 1,
            self.k * self.num_public_vars,
            |_, _| C::ScalarField::rand(rng),
        );
        let v_matrix = DMatrix::<C::ScalarField>::from_fn(self.k + 1, self.k, |_, _| {
            C::ScalarField::rand(rng)
        });

        let mpk = LinearPhfeMpk {
            a_matrix: matrix_to_g1::<C>(&a_matrix),
            aw_matrix: matrix_to_g1::<C>(&(&a_matrix * &w_matrix)),
            au_matrix: matrix_to_g1::<C>(&(&a_matrix * &u_matrix)),
            av_matrix: matrix_to_g1::<C>(&(&a_matrix * &v_matrix)),
        };
        let msk = LinearPhfeMsk {
            w_matrix,
            u_matrix,
            v_matrix,
        };
        (mpk, msk)
    }

    pub fn enc<R: Rng>(
        &self,
        mpk: &LinearPhfeMpk<C>,
        x: DVector<C::ScalarField>,
        z: DVector<C::ScalarField>,
        rng: &mut R,
    ) -> LinearPhfeCt<C> {
        let s_vec = DVector::<C::ScalarField>::from_fn(self.k, |i, _| C::ScalarField::rand(rng));
        let sa_vec = mpk.a_matrix.transpose() * s_vec;
        let x_vec = DMatrix::<C::G1>::from_fn(self.k, self.num_public_vars, |i, j| {
            mpk.a_matrix
                .row(i)
                .iter()
                .fold(C::G1::zero(), |acc, x| acc + x)
                * x[j]
        });
        let z_vec = DMatrix::<C::G1>::from_fn(self.k, self.num_private_vars, |i, j| {
            mpk.aw_matrix
                .row(i)
                .iter()
                .fold(C::G1::zero(), |acc, x| acc + x)
                * z[j]
        });
        LinearPhfeCt {
            sa_vec,
            z_vec,
            x_vec,
            x,
        }
    }
}

// impl<C: Pairing> LinearPhfe<C> {
//     pub fn new(
//         num_public_vars: usize,
//         num_private_vars: usize,
//         lx_bar: DMatrix<Monomial<F>>,
//         dfx_coeffs: Vec<Polynomial<F>>,
//     ) -> Self {
//         debug_assert_eq!(num_private_vars, dfx_coeffs.len());
//         Self {
//             num_public_vars,
//             num_private_vars,
//             lx_bar,
//             dfx_coeffs,
//             _c: PhantomData,
//         }
//     }

//     // pub fn setup(

//     // )
// }
