use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Rem;
use std::ops::Shr;
use std::ops::ShrAssign;
use std::ops::BitAnd;
use num_traits::identities::Zero;
use num_traits::identities::One;
use num_bigint::BigUint;

pub trait AdditiveGroup: Zero + Add<Output = Self> + AddAssign + Sub<Output = Self> + SubAssign {}
pub trait MultiplicativeGroup: One + Mul<Output = Self> + MulAssign {}
pub trait Ring: AdditiveGroup + MultiplicativeGroup {}
pub trait EuclideanDomain: Ring + Rem<Output = Self> + PartialEq {}
pub trait Exponent: Zero + One + BitAnd<Self, Output = Self> + Shr<u8, Output = Self> + ShrAssign<u8> + PartialEq {}

impl AdditiveGroup       for u64 {}
impl MultiplicativeGroup for u64 {}
impl Ring                for u64 {}
impl EuclideanDomain     for u64 {}
impl Exponent            for u64 {}

impl AdditiveGroup       for BigUint {}
impl MultiplicativeGroup for BigUint {}
impl Ring                for BigUint {}
impl EuclideanDomain     for BigUint {}
impl Exponent            for BigUint {}

#[derive(Debug, PartialEq, Clone)]
pub struct Mat<T>(pub (T, T), pub (T, T));

impl<T: EuclideanDomain> Mat<T> {
    pub fn one() -> Self {
        Mat((T::one() , T::zero()), (T::zero(), T::one() ))
    }
    pub fn q_matrix() -> Self {
        Mat((T::zero(), T::one() ), (T::one() , T::one() ))
    }
}

fn mul<T: EuclideanDomain + Clone>(x: &Mat<T>, y: &Mat<T>, m: T) -> Mat<T> {
    Mat(((x.0.0.clone() * y.0.0.clone() + x.0.1.clone() * y.1.0.clone()) % m.clone(),
         (x.0.0.clone() * y.0.1.clone() + x.0.1.clone() * y.1.1.clone()) % m.clone()),
        ((x.1.0.clone() * y.0.0.clone() + x.1.1.clone() * y.1.0.clone()) % m.clone(),
         (x.1.0.clone() * y.0.1.clone() + x.1.1.clone() * y.1.1.clone()) % m.clone()))
}

impl<T: EuclideanDomain + Clone> Mat<T> {
    pub fn mul(&self, other: &Self, m: T) -> Self {
        mul(self, other, m)
    }
}

pub fn exp_req<'a, T: EuclideanDomain + Clone, U: Exponent + Clone>(x: &Mat<T>, n: U, m: T) -> Mat<T> {
    if n.is_zero() {
        return Mat::one();
    }
    let mut y = exp_req(x, n.clone() >> 1u8, m.clone());
    y = y.mul(&y, m.clone());
    if (n & U::one()).is_zero() {
        y
    } else {
        y.mul(&x, m)
    }
}

pub fn exp_loop<T: EuclideanDomain + Clone, U: Exponent + Clone>(x: &Mat<T>, mut n: U, m: T) -> Mat<T> {
    let mut y: Mat<T> = x.clone();
    let mut z = Mat::one();
    while !n.clone().is_zero() {
        if (n.clone() & U::one()).is_one() {
            z = z.mul(&y, m.clone());
        }
        y = y.mul(&y, m.clone());
        n >>= 1u8;
    }
    z
}

impl<T: EuclideanDomain + Clone> Mat<T> {
    pub fn pow<U: Exponent + Clone>(&self, n: U, m: T) -> Self {
        exp_loop(self, n, m)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mul_1234_5678_u32() {
        let a = Mat((1, 2), (3, 4));
        let b = Mat((5, 6), (7, 8));
        let c = Mat((19, 22), (43, 50));
        assert_eq!(a.mul(&b, 1000), c);
    }
    #[test]
    fn mul_1234_5678_big_uint() {
        let a = Mat((BigUint::from( 1u32), BigUint::from( 2u32)), (BigUint::from( 3u32), BigUint::from( 4u32)));
        let b = Mat((BigUint::from( 5u32), BigUint::from( 6u32)), (BigUint::from( 7u32), BigUint::from( 8u32)));
        let c = Mat((BigUint::from(19u32), BigUint::from(22u32)), (BigUint::from(43u32), BigUint::from(50u32)));
        assert_eq!(a.mul(&b, BigUint::from(1000u32)), c);
    }
    #[test]
    fn exp_req_1234_100_u32() {
        let a = Mat((1, 2), (3, 4));
        let b = Mat((999, 250), (875, 874));
        assert_eq!(exp_req(&a, 100, 1000), b);
    }
    #[test]
    fn exp_req_1234_100_big_uint() {
        let a = Mat((BigUint::from(  1u32), BigUint::from(  2u32)), (BigUint::from(  3u32), BigUint::from(  4u32)));
        let b = Mat((BigUint::from(999u32), BigUint::from(250u32)), (BigUint::from(875u32), BigUint::from(874u32)));
        assert_eq!(exp_req(&a, BigUint::from(100u32), BigUint::from(1000u32)), b);
    }
    #[test]
    fn exp_loop_1234_100_u32() {
        let a = Mat((1, 2), (3, 4));
        let b = Mat((999, 250), (875, 874));
        assert_eq!(exp_loop(&a, 100, 1000), b);
    }
    #[test]
    fn exp_loop_1234_100_big_uint() {
        let a = Mat((BigUint::from(  1u32), BigUint::from(  2u32)), (BigUint::from(  3u32), BigUint::from(  4u32)));
        let b = Mat((BigUint::from(999u32), BigUint::from(250u32)), (BigUint::from(875u32), BigUint::from(874u32)));
        assert_eq!(exp_loop(&a, BigUint::from(100u32), BigUint::from(1000u32)), b);
    }
}
