use ark_ec::{
    pairing::{Pairing, PairingOutput},
    Group,
};
use ark_ff::PrimeField;
use core::panic;
use nalgebra::{base::Scalar, ClosedAddAssign};
use nalgebra::{DMatrix, DVector};
use num_traits::{One, Zero};
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg};

use crate::Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PhfeElement<C: Pairing> {
    Scalar(C::ScalarField),
    G1(C::G1),
    G2(C::G2),
    Gt(PairingOutput<C>),
}

impl<C: Pairing> Add for PhfeElement<C> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (PhfeElement::Scalar(scalar1), PhfeElement::Scalar(scalar2)) => {
                PhfeElement::Scalar(scalar1 + scalar2)
            }
            (PhfeElement::G1(g1), PhfeElement::G1(g2)) => PhfeElement::G1(g1 + g2),
            (PhfeElement::G2(g1), PhfeElement::G2(g2)) => PhfeElement::G2(g1 + g2),
            (PhfeElement::Gt(gt1), PhfeElement::Gt(gt2)) => PhfeElement::Gt(gt1 + gt2),
            _ => panic!("Addition not supported for these types"),
        }
    }
}

impl<C: Pairing> AddAssign for PhfeElement<C> {
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

impl<C: Pairing> Mul for PhfeElement<C> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match (self, other) {
            (PhfeElement::Scalar(scalar1), PhfeElement::Scalar(scalar2)) => {
                PhfeElement::Scalar(scalar1 * scalar2)
            }
            (PhfeElement::G1(g1), PhfeElement::Scalar(scalar)) => PhfeElement::G1(g1 * scalar),
            (PhfeElement::Scalar(scalar), PhfeElement::G1(g1)) => PhfeElement::G1(g1 * scalar),
            (PhfeElement::G2(g2), PhfeElement::Scalar(scalar)) => PhfeElement::G2(g2 * scalar),
            (PhfeElement::Scalar(scalar), PhfeElement::G2(g2)) => PhfeElement::G2(g2 * scalar),
            (PhfeElement::Gt(gt), PhfeElement::Scalar(scalar)) => PhfeElement::Gt(gt * scalar),
            (PhfeElement::Scalar(scalar), PhfeElement::Gt(gt)) => PhfeElement::Gt(gt * scalar),
            (PhfeElement::G1(g1), PhfeElement::G2(g2)) => PhfeElement::Gt(C::pairing(g1, g2)),
            (PhfeElement::G2(g2), PhfeElement::G1(g1)) => PhfeElement::Gt(C::pairing(g1, g2)),
            _ => panic!("Multiplication not supported for these types"),
        }
    }
}

impl<C: Pairing> MulAssign for PhfeElement<C> {
    fn mul_assign(&mut self, other: Self) {
        *self = self.clone() * other;
    }
}

impl<C: Pairing> Neg for PhfeElement<C> {
    type Output = Self;

    fn neg(self) -> Self {
        match self {
            PhfeElement::Scalar(scalar) => PhfeElement::Scalar(-scalar),
            PhfeElement::G1(g1) => PhfeElement::G1(-g1),
            PhfeElement::G2(g2) => PhfeElement::G2(-g2),
            PhfeElement::Gt(gt) => PhfeElement::Gt(-gt),
        }
    }
}

impl<C: Pairing> Zero for PhfeElement<C> {
    fn zero() -> Self {
        PhfeElement::Scalar(C::ScalarField::zero())
    }

    fn is_zero(&self) -> bool {
        match self {
            PhfeElement::Scalar(scalar) => scalar.is_zero(),
            PhfeElement::G1(g1) => g1.is_zero(),
            PhfeElement::G2(g2) => g2.is_zero(),
            PhfeElement::Gt(gt) => gt.is_zero(),
            _ => false,
        }
    }
}

impl<C: Pairing> One for PhfeElement<C> {
    fn one() -> Self {
        PhfeElement::Scalar(C::ScalarField::one())
    }

    fn is_one(&self) -> bool {
        match self {
            PhfeElement::Scalar(scalar) => scalar.is_one(),
            PhfeElement::G1(g1) => g1 == &C::G1::generator(),
            PhfeElement::G2(g2) => g2 == &C::G2::generator(),
            PhfeElement::Gt(gt) => gt == &PairingOutput::generator(),
            _ => false,
        }
    }
}

// pub fn matrix_to_g1<C: Pairing>(matrix: &DMatrix<C::ScalarField>) -> DMatrix<C::G1> {
//     matrix.map(|scalar| C::G1::generator() * scalar)
// }

pub fn scalar_matrix_to_g1<C: Pairing>(
    matrix: &DMatrix<PhfeElement<C>>,
) -> DMatrix<PhfeElement<C>> {
    matrix.map(|val| {
        if let PhfeElement::Scalar(s) = val {
            PhfeElement::G1(C::G1::generator() * s)
        } else {
            panic!("Expected scalar, got {:?}", val);
        }
    })
}

pub fn scalar_vec_to_g1<C: Pairing>(vec: &DVector<PhfeElement<C>>) -> DVector<PhfeElement<C>> {
    vec.map(|val| {
        if let PhfeElement::Scalar(s) = val {
            PhfeElement::G1(C::G1::generator() * s)
        } else {
            panic!("Expected scalar, got {:?}", val);
        }
    })
}

pub fn scalar_matrix_to_g2<C: Pairing>(
    matrix: &DMatrix<PhfeElement<C>>,
) -> DMatrix<PhfeElement<C>> {
    matrix.map(|val| {
        if let PhfeElement::Scalar(s) = val {
            PhfeElement::G2(C::G2::generator() * s)
        } else {
            panic!("Expected scalar, got {:?}", val);
        }
    })
}

pub fn scalar_vec_to_g2<C: Pairing>(vec: &DVector<PhfeElement<C>>) -> DVector<PhfeElement<C>> {
    vec.map(|val| {
        if let PhfeElement::Scalar(s) = val {
            PhfeElement::G2(C::G2::generator() * s)
        } else {
            panic!("Expected scalar, got {:?}", val);
        }
    })
}

pub fn parse_field_str<F: PrimeField>(s: &str) -> Result<F, Error> {
    let is_minus = s.starts_with('-');
    if is_minus {
        let scalar = F::from_str(&s[1..]).map_err(|_| Error::InvalidFieldString(s.to_string()))?;
        Ok(-scalar)
    } else {
        Ok(F::from_str(s).map_err(|_| Error::InvalidFieldString(s.to_string()))?)
    }
}

// pub fn mul_scalar_and_g1_matrixes<C: Pairing>(
//     x: &DMatrix<C::ScalarField>,
//     y: &DMatrix<C::G1>,
// ) -> DMatrix<C::G1> {
//     debug_assert_eq!(x.ncols(), y.nrows());

//     let mut result = DMatrix::from_element(x.nrows(), y.ncols(), C::G1::zero());

//     for i in 0..x.nrows() {
//         for j in 0..y.ncols() {
//             for k in 0..x.ncols() {
//                 result[(i, j)] += y[(k, j)] * x[(i, k)];
//             }
//         }
//     }
//     result
// }

// pub fn scalar_vec_to_g1<C: Pairing>(vec: &DVector<C::ScalarField>) -> DVector<C::G1> {
//     vec.map(|scalar| C::G1::generator() * scalar)
// }

pub fn vec_tensor_product_with_identity<C: Pairing>(
    x: &DVector<PhfeElement<C>>,
    k: usize,
) -> DMatrix<PhfeElement<C>> {
    let x_t = x.transpose();
    let mut result = DMatrix::<PhfeElement<C>>::zeros(x.len() * k, k);

    for i in 0..x.len() {
        for j in 0..k {
            result[(i * k + j, j)] = x_t[i].clone();
        }
    }

    result
}

pub fn identity_tensor_product_with_matrix<C: Pairing>(
    x: &DMatrix<PhfeElement<C>>,
    k: usize,
) -> DMatrix<PhfeElement<C>> {
    let n = x.nrows();
    let m = x.ncols();
    let mut result = DMatrix::<PhfeElement<C>>::zeros(n * k, m * k);

    for p in 0..k {
        for i in 0..n {
            for j in 0..m {
                result[(n * p + i, m * p + j)] = x[(i, j)].clone();
            }
        }
    }
    result
}

pub fn matrix_tensor_product_with_identity<C: Pairing>(
    x: &DMatrix<PhfeElement<C>>,
    k: usize,
) -> DMatrix<PhfeElement<C>> {
    let n = x.nrows();
    let m = x.ncols();
    let mut result = DMatrix::<PhfeElement<C>>::zeros(n * k, m * k);

    for i in 0..n {
        for j in 0..m {
            let value = x[(i, j)].clone();
            for p in 0..k {
                result[(i * k + p, j * k + p)] = value.clone();
            }
        }
    }
    result
}
