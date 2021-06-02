use std::ops::Add;
use std::ops::Mul;
use std::ops::Rem;
use std::ops::Shr;
use std::ops::ShrAssign;
use std::ops::BitAnd;
use num_traits::identities::Zero;
use num_traits::identities::One;
use num_bigint::BigUint;

pub trait AdditiveGroup: Sized + Add<Output = Self> {
    fn zero(&self) -> Self;
    fn is_zero(&self) -> bool;
}
pub trait MultiplicativeGroup: Sized + Mul<Output = Self> {
    fn one(&self) -> Self;
    fn is_one(&self) -> bool;
}
pub trait Ring: AdditiveGroup + MultiplicativeGroup {}
pub trait EuclideanDomain: Ring + Rem<Output = Self> + PartialEq {}
pub trait Exponent: Zero + One + BitAnd<Output = Self> + Shr<u8, Output = Self> + ShrAssign<u8> + PartialEq {}

pub struct IntegerRingBigUint{
    v: BigUint,
    m: BigUint,
}

impl IntegerRingBigUint {
    fn new(v: BigUint, m: BigUint) -> Self {
        Self{ v: v % m.clone(), m }
    }
}

impl Add for IntegerRingBigUint {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        assert_eq!(self.m, other.m);
        Self{ v: (self.v + other.v) % self.m.clone(), m: self.m }
    }
}

impl AdditiveGroup for IntegerRingBigUint {
    fn zero(&self) -> Self {
        Self::new(BigUint::zero(), self.m.clone())
    }
    fn is_zero(&self) -> bool {
        self.v.is_zero()
    }
}

/*
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
*/
