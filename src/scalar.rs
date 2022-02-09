use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use ff::{Field, PrimeField};
use num_bigint::BigUint;
use num_traits::{pow::Pow, One, Zero};
use rand::{distributions::Distribution, Rng};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Default)]
pub struct Scalar<F: PrimeField>(pub F);

impl<F: PrimeField> Add for Scalar<F> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let f = self.0 + rhs.0;
        Self(f)
    }
}

impl<F: PrimeField> Sub for Scalar<F> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let f = self.0 - rhs.0;
        Self(f)
    }
}

impl<F: PrimeField> Mul for Scalar<F> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let f = self.0 * rhs.0;
        Self(f)
    }
}

impl<F: PrimeField> Div for Scalar<F> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let f = self.0 * rhs.0.invert().unwrap();
        Self(f)
    }
}

impl<'a, F: PrimeField> Add for &'a Scalar<F> {
    type Output = Scalar<F>;
    fn add(self, rhs: Self) -> Self::Output {
        let f = self.0 + rhs.0;
        Scalar(f)
    }
}

impl<'a, F: PrimeField> Sub for &'a Scalar<F> {
    type Output = Scalar<F>;
    fn sub(self, rhs: Self) -> Self::Output {
        let f = self.0 - rhs.0;
        Scalar(f)
    }
}

impl<'a, F: PrimeField> Mul for &'a Scalar<F> {
    type Output = Scalar<F>;
    fn mul(self, rhs: Self) -> Self::Output {
        let f = self.0 * rhs.0;
        Scalar(f)
    }
}

impl<'a, F: PrimeField> Div for &'a Scalar<F> {
    type Output = Scalar<F>;
    fn div(self, rhs: Self) -> Self::Output {
        let f = self.0 * rhs.0.invert().unwrap();
        Scalar(f)
    }
}

impl<F: PrimeField> AddAssign for Scalar<F> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl<F: PrimeField> SubAssign for Scalar<F> {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl<F: PrimeField> MulAssign for Scalar<F> {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}

impl<F: PrimeField> DivAssign for Scalar<F> {
    fn div_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0.invert().unwrap();
    }
}

impl<F: PrimeField> Zero for Scalar<F> {
    fn zero() -> Self {
        Self(Field::zero())
    }

    fn is_zero(&self) -> bool {
        Self::zero() == *self
    }
}

impl<F: PrimeField> One for Scalar<F> {
    fn one() -> Self {
        Self(Field::one())
    }
}

impl<'b, F: PrimeField> Pow<&'b BigUint> for Scalar<F> {
    type Output = Scalar<F>;
    fn pow(self, rth: &BigUint) -> Self::Output {
        let exp_vec = rth.to_u64_digits();
        let f = self.0.pow_vartime(&exp_vec);
        Self(f)
    }
}

impl<F: PrimeField> From<usize> for Scalar<F> {
    fn from(from: usize) -> Self {
        let from_bytes = from.to_le_bytes();
        let from_bits: Vec<bool> = from_bytes
            .iter()
            .map(|byte| (0..8).map(move |i| (byte >> i) & 1u8 == 1u8))
            .flatten()
            .collect();
        let (f, _) = from_bits.iter().fold((F::zero(), F::one()), |(sum, t), b| {
            let new_sum = if *b { sum + t } else { sum };
            (new_sum, t.double())
        });
        Self(f)
    }
}

impl<F: PrimeField> fmt::Display for Scalar<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl<F: PrimeField> Scalar<F> {
    pub fn pow_generator(exp: &BigUint) -> Self {
        let g = <F as PrimeField>::multiplicative_generator();
        Self(g).pow(exp)
    }
}

pub struct ScalarDistribution;

impl<F: PrimeField> Distribution<Scalar<F>> for ScalarDistribution {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Scalar<F> {
        Scalar(F::random(rng))
    }
}

pub mod bls12_381 {
    use super::*;

    // This is a scalar field of subgroup in the BLS12-381 curve.
    #[derive(PrimeField)]
    #[PrimeFieldModulus = "52435875175126190479447740508185965837690552500527637822603658699938581184513"]
    #[PrimeFieldGenerator = "2"]
    #[PrimeFieldReprEndianness = "little"]
    pub struct BLS12F([u64; 4]);
    pub type BLS12Scalar = Scalar<BLS12F>;
}

#[cfg(test)]
mod test {
    use crate::scalar::bls12_381::BLS12F;

    use super::bls12_381::BLS12Scalar;
    use super::*;
    #[test]
    fn from_usize_test() {
        println!("S: {}",BLS12F::S);
        let five_from_usize = BLS12Scalar::from(5);
        let one = BLS12Scalar::one();
        let five = one + one + one + one + one;
        assert_eq!(five_from_usize, five);
    }
}
