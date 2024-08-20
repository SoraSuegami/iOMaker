use ark_ec::{pairing::Pairing, Group};
use nalgebra::DMatrix;

pub fn matrix_to_g1<C: Pairing>(matrix: &DMatrix<C::ScalarField>) -> DMatrix<C::G1> {
    matrix.map(|scalar| C::G1::generator() * scalar)
}

pub fn matrix_to_g2<C: Pairing>(matrix: &DMatrix<C::ScalarField>) -> DMatrix<C::G2> {
    matrix.map(|scalar| C::G2::generator() * scalar)
}

// pub fn mul_scalar_and_matrix_on_g1<C: Pairing>()
