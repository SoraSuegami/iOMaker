pub mod func;
pub mod linear_phfe;
pub mod utils;
use ark_ec::pairing::PairingOutput;
use ark_ec::{pairing::Pairing, Group};
use ark_ff::UniformRand;
pub use func::PhfeFunc;
use linear_phfe::*;
use nalgebra::{DMatrix, DVector, Dyn, Matrix, U1};
use rand::Rng;
use std::collections::HashMap;
use std::marker::PhantomData;
pub use utils::PhfeElement;
use utils::*;

use crate::poly::{Variable, VariableType};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PhfeMpk<C: Pairing> {
    pub a1_g1_matrix: DMatrix<PhfeElement<C>>,
    pub a2_g1_matrix: DMatrix<PhfeElement<C>>,
    pub a2_g2_matrix: DMatrix<PhfeElement<C>>,
    pub m_matrix: DMatrix<PhfeElement<C>>,
    pub linear_mpk: LinearPhfeMpk<C>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PhfeMsk<C: Pairing> {
    pub linear_msk: LinearPhfeMsk<C>,
    pub m_matrix: DMatrix<PhfeElement<C>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PhfeCt<C: Pairing> {
    pub y1_vec: DVector<PhfeElement<C>>,
    pub y2_vec: DVector<PhfeElement<C>>,
    pub linear_ct: LinearPhfeCt<C>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PhfeFsk<C: Pairing> {
    pub linear_fsk: LinearPhfeFsk<C>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Phfe<C: Pairing> {
    pub num_public_vars: usize,
    pub num_private_vars1: usize,
    pub num_private_vars2: usize,
    pub k1: usize,
    pub k2: usize,
    pub linear_phfe: LinearPhfe<C>,
    _c: PhantomData<C>,
}

impl<C: Pairing> Phfe<C> {
    pub fn new(
        num_public_vars: usize,
        num_private_vars1: usize,
        num_private_vars2: usize,
        k: usize,
    ) -> Self {
        let k1 = num_private_vars1 + 1;
        let k2 = num_private_vars2 + 1;
        Self {
            num_public_vars,
            num_private_vars1,
            num_private_vars2,
            k1,
            k2,
            linear_phfe: LinearPhfe::new(
                num_public_vars,
                k2 * num_private_vars1 + k1 * num_private_vars2,
                k,
            ),
            _c: PhantomData,
        }
    }

    pub fn setup<R: Rng>(&self, rng: &mut R) -> (PhfeMpk<C>, PhfeMsk<C>) {
        let (linear_mpk, linear_msk) = self.linear_phfe.setup(rng);
        let a1_matrix =
            DMatrix::<PhfeElement<C>>::from_fn(self.k1, self.num_private_vars1, |_, _| {
                PhfeElement::Scalar(C::ScalarField::rand(rng))
            });
        let a2_matrix =
            DMatrix::<PhfeElement<C>>::from_fn(self.k2, self.num_private_vars2, |_, _| {
                PhfeElement::Scalar(C::ScalarField::rand(rng))
            });
        let m_matrix = {
            let a1_rows = matrix_tensor_product_with_identity(&a1_matrix, self.num_private_vars2);
            let a2_rows = identity_tensor_product_with_matrix(&a2_matrix, self.num_private_vars1);
            // concat a1_rows and a2_rows
            debug_assert_eq!(a1_rows.ncols(), a2_rows.ncols());
            DMatrix::from_fn(
                a1_rows.nrows() + a2_rows.nrows(),
                a1_rows.ncols(),
                |i, j| {
                    if i < a1_rows.nrows() {
                        a1_rows[(i, j)].clone()
                    } else {
                        a2_rows[(i - a1_rows.nrows(), j)].clone()
                    }
                },
            )
        };
        let m_matrix = scalar_matrix_to_g2(&m_matrix);
        let a1_g1_matrix = scalar_matrix_to_g1(&a1_matrix);
        let a2_g1_matrix = scalar_matrix_to_g1(&a2_matrix);
        let a2_g2_matrix = scalar_matrix_to_g2(&a2_matrix);
        let mpk = PhfeMpk {
            a1_g1_matrix,
            a2_g1_matrix,
            a2_g2_matrix,
            m_matrix: m_matrix.clone(),
            linear_mpk,
        };
        let msk = PhfeMsk {
            linear_msk,
            m_matrix,
        };
        (mpk, msk)
    }

    pub fn enc<R: Rng>(
        &self,
        mpk: &PhfeMpk<C>,
        x: &DVector<C::ScalarField>,
        z1: &DVector<C::ScalarField>,
        z2: &DVector<C::ScalarField>,
        rng: &mut R,
    ) -> PhfeCt<C> {
        debug_assert_eq!(x.len(), self.num_public_vars);
        debug_assert_eq!(z1.len(), self.num_private_vars1);
        debug_assert_eq!(z2.len(), self.num_private_vars2);

        let s1 = DVector::<PhfeElement<C>>::from_fn(self.k1, |i, _| {
            PhfeElement::Scalar(C::ScalarField::rand(rng))
        });
        let s2 = DVector::<PhfeElement<C>>::from_fn(self.k2, |i, _| {
            PhfeElement::Scalar(C::ScalarField::rand(rng))
        });
        let z1 = z1.map(|s| PhfeElement::Scalar(s));
        let z2 = z2.map(|s| PhfeElement::Scalar(s));
        let y1 =
            (&s1.transpose() * &mpk.a1_g1_matrix + scalar_vec_to_g1(&z1).transpose()).transpose();
        let y2 =
            (&s2.transpose() * &mpk.a2_g2_matrix + scalar_vec_to_g2(&z2).transpose()).transpose();
        let linear_z = {
            let s1z2 = scalar_vec_to_g1(&s1.kronecker(&z2));
            let y1s2 = y1.kronecker(&s2);
            // concat s1z2 and y1s2
            DVector::from_fn(s1z2.len() + y1s2.len(), |i, _| {
                if i < s1z2.len() {
                    s1z2[i].clone()
                } else {
                    y1s2[i - s1z2.len()].clone()
                }
            })
        };
        let linear_ct = self.linear_phfe.enc(&mpk.linear_mpk, x, &linear_z, rng);
        PhfeCt {
            y1_vec: y1,
            y2_vec: y2,
            linear_ct,
        }
    }

    pub fn gen_fsk<R: Rng>(
        &self,
        msk: &PhfeMsk<C>,
        f: &PhfeFunc<C::ScalarField>,
        rng: &mut R,
    ) -> PhfeFsk<C> {
        let linear_fsk = self
            .linear_phfe
            .gen_fsk(&msk.linear_msk, f, &msk.m_matrix, rng);
        PhfeFsk { linear_fsk }
    }

    pub fn dec(
        &self,
        ct: &PhfeCt<C>,
        f: &PhfeFunc<C::ScalarField>,
        fsk: &PhfeFsk<C>,
    ) -> PairingOutput<C> {
        let linear_dec = self.linear_phfe.dec(&ct.linear_ct, f, &fsk.linear_fsk);
        println!("linear_dec: {:?}", linear_dec);
        let y1y2 = ct.y1_vec.kronecker(&ct.y2_vec);
        let mut x_assignment = HashMap::<Variable, C::ScalarField>::new();
        for (idx, val) in ct.linear_ct.x.into_iter().enumerate() {
            x_assignment.insert(
                Variable {
                    index: idx as u32,
                    variable_type: VariableType::Public,
                },
                *val,
            );
        }
        let fx = DVector::from_iterator(
            y1y2.len(),
            f.polys
                .iter()
                .map(|f| PhfeElement::Scalar(f.eval(&x_assignment))),
        );
        let y1y2f = y1y2.transpose() * &fx;
        println!("y1y2f: {:?}", y1y2f);
        if let PhfeElement::Gt(y1y2f) = y1y2f[(0, 0)] {
            y1y2f - linear_dec
        } else {
            panic!("Unexpected element type")
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ark_bn254::{Bn254, Fr};
    use ark_std::{end_timer, start_timer};
    use num_traits::{One, Zero};

    #[test]
    fn test_valid_case1() {
        let func_json = include_str!("./phfe/tests/test_phfe1.json");
        let func: PhfeFunc<Fr> = PhfeFunc::from_str(&func_json).unwrap();
        let phfe = Phfe::<Bn254>::new(
            func.num_public_vars,
            func.num_private_vars1,
            func.num_private_vars2,
            2,
        );
        let mut rng = rand::thread_rng();
        let (mpk, msk) = phfe.setup(&mut rng);
        let x = DVector::from_fn(3, |_, _| Fr::from(rng.gen_range(0..=1)));
        let z1 = DVector::from_fn(2, |_, _| Fr::from(rng.gen_range(0..=1)));
        let z2 = DVector::from_fn(3, |_, _| Fr::from(rng.gen_range(0..=1)));
        let ct = phfe.enc(&mpk, &x, &z1, &z2, &mut rng);
        let fsk = phfe.gen_fsk(&msk, &func, &mut rng);
        let out_gt = phfe.dec(&ct, &func, &fsk);
        // let out = if out_gt == PairingOutput::generator() {
        //     Fr::one()
        // } else if out_gt == PairingOutput::zero() {
        //     Fr::zero()
        // } else {
        //     panic!("Unexpected output")
        // };
        let expected_out = {
            let mut x_assignment = HashMap::<Variable, Fr>::new();
            for (idx, val) in x.iter().enumerate() {
                x_assignment.insert(
                    Variable {
                        index: idx as u32,
                        variable_type: VariableType::Public,
                    },
                    *val,
                );
            }
            let z1z2 = z1.kronecker(&z2);
            let fx = DVector::from_iterator(
                z1z2.len(),
                func.polys.iter().map(|f| f.eval(&x_assignment)),
            );
            (z1z2.transpose() * &fx)[(0, 0)]
        };
        assert_eq!(out_gt, PairingOutput::generator() * expected_out);
    }

    #[test]
    fn test_valid_case2() {
        let func_json = include_str!("./phfe/tests/test_phfe2.json");
        let func: PhfeFunc<Fr> = PhfeFunc::from_str(&func_json).unwrap();
        let phfe = Phfe::<Bn254>::new(
            func.num_public_vars,
            func.num_private_vars1,
            func.num_private_vars2,
            2,
        );
        let mut rng = rand::thread_rng();
        let (mpk, msk) = phfe.setup(&mut rng);
        let x = DVector::from_fn(14, |_, _| Fr::from(rng.gen_range(0..=1)));
        let z1 = DVector::from_fn(2, |_, _| Fr::from(rng.gen_range(0..=1)));
        let z2 = DVector::from_fn(5, |_, _| Fr::from(rng.gen_range(0..=1)));
        let ct = phfe.enc(&mpk, &x, &z1, &z2, &mut rng);
        let fsk = phfe.gen_fsk(&msk, &func, &mut rng);
        let out_gt = phfe.dec(&ct, &func, &fsk);
        let expected_out = {
            let mut x_assignment = HashMap::<Variable, Fr>::new();
            for (idx, val) in x.iter().enumerate() {
                x_assignment.insert(
                    Variable {
                        index: idx as u32,
                        variable_type: VariableType::Public,
                    },
                    *val,
                );
            }
            let z1z2 = z1.kronecker(&z2);
            let fx = DVector::from_iterator(
                z1z2.len(),
                func.polys.iter().map(|f| f.eval(&x_assignment)),
            );
            (z1z2.transpose() * &fx)[(0, 0)]
        };
        println!("expected_out: {:?}", expected_out);
        assert_eq!(out_gt, PairingOutput::generator() * expected_out);
    }

    #[test]
    fn test_valid_case3() {
        let func_json = include_str!("./phfe/tests/test_phfe3.json");
        let func: PhfeFunc<Fr> = PhfeFunc::from_str(&func_json).unwrap();
        let phfe = Phfe::<Bn254>::new(
            func.num_public_vars,
            func.num_private_vars1,
            func.num_private_vars2,
            2,
        );
        let mut rng = rand::thread_rng();
        let setup_timer = start_timer!(|| "setup keys");
        let (mpk, msk) = phfe.setup(&mut rng);
        end_timer!(setup_timer);
        let x = DVector::from_fn(20, |_, _| Fr::from(rng.gen_range(0..=1)));
        let z1 = DVector::from_fn(4, |_, _| Fr::from(rng.gen_range(0..=1)));
        let z2 = DVector::from_fn(5, |_, _| Fr::from(rng.gen_range(0..=1)));
        let enc_timer = start_timer!(|| "encryption");
        let ct = phfe.enc(&mpk, &x, &z1, &z2, &mut rng);
        end_timer!(enc_timer);
        let fsk_timer = start_timer!(|| "fsk generation");
        let fsk = phfe.gen_fsk(&msk, &func, &mut rng);
        end_timer!(fsk_timer);
        let dec_timer = start_timer!(|| "decryption");
        let out_gt = phfe.dec(&ct, &func, &fsk);
        end_timer!(dec_timer);
        let expected_out = {
            let mut x_assignment = HashMap::<Variable, Fr>::new();
            for (idx, val) in x.iter().enumerate() {
                x_assignment.insert(
                    Variable {
                        index: idx as u32,
                        variable_type: VariableType::Public,
                    },
                    *val,
                );
            }
            let z1z2 = z1.kronecker(&z2);
            let fx = DVector::from_iterator(
                z1z2.len(),
                func.polys.iter().map(|f| f.eval(&x_assignment)),
            );
            (z1z2.transpose() * &fx)[(0, 0)]
        };
        println!("expected_out: {:?}", expected_out);
        assert_eq!(out_gt, PairingOutput::generator() * expected_out);
    }
}
