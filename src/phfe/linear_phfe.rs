use super::*;
use crate::poly::{Monomial, Polynomial};
use crate::poly::{Variable, VariableType};
use ark_ec::pairing::PairingOutput;
use ark_ec::{pairing::Pairing, Group};
use ark_ff::PrimeField;
use ark_ff::UniformRand;
use itertools::Itertools;
use nalgebra::Dyn;
use nalgebra::Matrix;
use nalgebra::U1;
use nalgebra::{DMatrix, DVector};
use num_traits::{One, Zero};
use rand::Rng;
use std::collections::HashMap;
use std::marker::PhantomData;
use utils::*;
// use utils::mul_scalar_vec_and_g1_matrix;
// use utils::scalar_vec_tensor_product_with_identity;
// use utils::scalar_vec_to_g1;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LinearPhfeMpk<C: Pairing> {
    pub a_matrix: DMatrix<PhfeElement<C>>,
    pub aw_matrix: DMatrix<PhfeElement<C>>,
    pub au_matrix: DMatrix<PhfeElement<C>>,
    pub av_matrix: DMatrix<PhfeElement<C>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LinearPhfeMsk<C: Pairing> {
    pub w_matrix: DMatrix<PhfeElement<C>>,
    pub u_matrix: DMatrix<PhfeElement<C>>,
    pub v_matrix: DMatrix<PhfeElement<C>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LinearPhfeCt<C: Pairing> {
    pub sa_vec: DVector<PhfeElement<C>>,
    pub z_vec: DVector<PhfeElement<C>>,
    pub x_vec: DVector<PhfeElement<C>>,
    pub x: DVector<C::ScalarField>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LinearPhfeFsk<C: Pairing> {
    pub tm_matrix: DMatrix<PhfeElement<C>>,
    pub l1_matrix: DMatrix<PhfeElement<C>>,
    pub l0_matrix: DMatrix<PhfeElement<C>>,
    pub r_matrix: DMatrix<PhfeElement<C>>,
    pub m_matrix: DMatrix<PhfeElement<C>>,
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
        let a_matrix = DMatrix::<PhfeElement<C>>::from_fn(self.k, self.k + 1, |_, _| {
            PhfeElement::Scalar(C::ScalarField::rand(rng))
        });
        let w_matrix =
            DMatrix::<PhfeElement<C>>::from_fn(self.k + 1, self.num_private_vars, |_, _| {
                PhfeElement::Scalar(C::ScalarField::rand(rng))
            });
        let u_matrix = DMatrix::<PhfeElement<C>>::from_fn(
            self.k + 1,
            self.k * self.num_public_vars,
            |_, _| PhfeElement::Scalar(C::ScalarField::rand(rng)),
        );
        let v_matrix = DMatrix::<PhfeElement<C>>::from_fn(self.k + 1, self.k, |_, _| {
            PhfeElement::Scalar(C::ScalarField::rand(rng))
        });

        let mpk = LinearPhfeMpk {
            a_matrix: scalar_matrix_to_g1::<C>(&a_matrix),
            aw_matrix: scalar_matrix_to_g1::<C>(&(&a_matrix * &w_matrix)),
            au_matrix: scalar_matrix_to_g1::<C>(&(&a_matrix * &u_matrix)),
            av_matrix: scalar_matrix_to_g1::<C>(&(&a_matrix * &v_matrix)),
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
        x: &DVector<C::ScalarField>,
        z_g1: &DVector<PhfeElement<C>>,
        rng: &mut R,
    ) -> LinearPhfeCt<C> {
        let s_vec = DVector::<PhfeElement<C>>::from_fn(self.k, |i, _| {
            PhfeElement::Scalar(C::ScalarField::rand(rng))
        })
        .transpose();
        let sa_vec: Matrix<PhfeElement<C>, U1, Dyn, _> = &s_vec * &mpk.a_matrix;
        let saw_vec: Matrix<PhfeElement<C>, U1, Dyn, _> = &s_vec * &mpk.aw_matrix;
        let z_vec: Matrix<PhfeElement<C>, U1, Dyn, _> = z_g1.transpose() + saw_vec;
        let sau_vec = &s_vec * &mpk.au_matrix;
        let sav_vec = &s_vec * &mpk.av_matrix;
        let x_tensored =
            vec_tensor_product_with_identity(&x.map(|s| PhfeElement::Scalar(s)), self.k);
        let x_vec = sau_vec * x_tensored + sav_vec;

        LinearPhfeCt {
            sa_vec: sa_vec.transpose(),
            z_vec: z_vec.transpose(),
            x_vec: x_vec.transpose(),
            x: x.clone(),
        }
    }

    pub fn gen_fsk<R: Rng>(
        &self,
        msk: &LinearPhfeMsk<C>,
        f: &PhfeFunc<C::ScalarField>,
        m: &DMatrix<PhfeElement<C>>,
        rng: &mut R,
    ) -> LinearPhfeFsk<C> {
        debug_assert!(self.k < f.l0.ncols());
        let t_matrix = DMatrix::<PhfeElement<C>>::from_fn(self.k + 1, f.l1.nrows(), |_, _| {
            PhfeElement::Scalar(C::ScalarField::rand(rng))
        });
        let r_matrix = DMatrix::<PhfeElement<C>>::from_fn(self.k, f.l0.ncols(), |_, _| {
            PhfeElement::Scalar(C::ScalarField::rand(rng))
        });
        let t_bar = t_matrix
            .columns(t_matrix.ncols() - m.ncols(), m.ncols())
            .map(|v| {
                if let PhfeElement::Scalar(s) = v {
                    PhfeElement::G2(C::G2::generator() * s)
                } else {
                    panic!("t_bar should be on G2")
                }
            });
        let wm = &msk.w_matrix * m;
        let tm_matrix = &t_bar + wm;
        let r_tensored = identity_tensor_product_with_matrix::<C>(&r_matrix, self.num_public_vars);
        let tl1 = &t_matrix * &f.l1.map(|s| PhfeElement::Scalar(s));
        let l1_matrix = tl1 + &msk.u_matrix * r_tensored;
        let tl0 = &t_matrix * &f.l0.map(|s| PhfeElement::Scalar(s));
        let vr = &msk.v_matrix * &r_matrix;
        let l0_matrix = tl0 + vr;

        LinearPhfeFsk {
            tm_matrix,
            l1_matrix: scalar_matrix_to_g2(&l1_matrix),
            l0_matrix: scalar_matrix_to_g2(&l0_matrix),
            r_matrix: scalar_matrix_to_g2(&r_matrix),
            m_matrix: m.clone(),
        }
    }

    pub fn dec(
        &self,
        ct: &LinearPhfeCt<C>,
        f: &PhfeFunc<C::ScalarField>,
        fsk: &LinearPhfeFsk<C>,
    ) -> PairingOutput<C> {
        let p1 = &ct.z_vec.transpose() * &fsk.m_matrix + ct.sa_vec.transpose() * (-&fsk.tm_matrix);
        let x_tensored =
            vec_tensor_product_with_identity(&ct.x.map(|s| PhfeElement::Scalar(s)), f.l0.ncols());
        let p2 = &ct.sa_vec.transpose() * (&fsk.l1_matrix * x_tensored + &fsk.l0_matrix)
            + (-&ct.x_vec.transpose()) * &fsk.r_matrix;
        let mut x_assignment = HashMap::<Variable, C::ScalarField>::new();
        for (idx, val) in ct.x.into_iter().enumerate() {
            x_assignment.insert(
                Variable {
                    index: idx as u32,
                    variable_type: VariableType::Public,
                },
                *val,
            );
        }
        let dfx_vec = DVector::<PhfeElement<C>>::from_fn(f.dfx_coeffs.len(), |i, _| {
            PhfeElement::Scalar(f.dfx_coeffs[i].eval(&x_assignment))
        });
        debug_assert_eq!(p1.nrows(), p2.nrows());
        let p12: DMatrix<PhfeElement<C>> =
            DMatrix::from_fn(p1.nrows(), p1.ncols() + p2.ncols(), |i, j| {
                if j < p1.ncols() {
                    p1[(i, j)].clone()
                } else {
                    p2[(i, j - p1.ncols())].clone()
                }
            });
        let result_gt: Matrix<PhfeElement<C>, Dyn, U1, _> = p12 * dfx_vec;
        match result_gt[(0, 0)] {
            PhfeElement::Gt(gt) => gt,
            _ => panic!("Pairing output should be on Gt"),
        }
    }
}
