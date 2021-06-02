pub mod algebra;
pub mod mat;

/*
use num_bigint::BigUint;
use algebra::EuclideanDomain;
use algebra::Exponent;

pub trait Index: EuclideanDomain + Exponent {}

impl Index for u64 {}
impl Index for BigUint {}

pub fn fibo_rec<T: Index + Clone>(n: T, m: T) -> T {
    if n.is_zero() {
        return T::zero();
    }
    if n.is_one() {
        return T::one();
    }
    (fibo_rec(n.clone() - T::one() - T::one(), m.clone()) + fibo_rec(n - T::one(), m.clone())) % m
}

pub fn fibo_seq<T: Index + Clone>(n: T, m: T) -> T {
    let mut a = T::zero();
    let mut b = T::one();
    let mut i = T::zero();
    while i != n {
        let t = (a.clone() + b.clone()) % m.clone();
        a = b;
        b = t;
        i += T::one();
    }
    a
}

pub fn fibo_mat<T: Index + Clone>(n: T, m: T) -> T {
    let mut t = mat::Mat::one();
    let mut i = T::zero();
    while i != n {
        t = t.mul(&mat::Mat::q_matrix(), m.clone());
        i += T::one();
    }
    t.0.1
}

pub fn fibo_mat_req<T: Index + Clone, U: EuclideanDomain + Clone>(n: T, m: U) -> U {
    mat::exp_req(&mat::Mat::q_matrix(), n, m).0.1
}

pub fn fibo_mat_loop<T: Index + Clone, U: EuclideanDomain + Clone>(n: T, m: U) -> U {
    mat::Mat::q_matrix().pow(n, m).0.1
}
*/

#[cfg(test)]
mod tests {
    /*
    use super::*;
    #[test]
    fn fibo_rec_n_10_u32() {
        assert_eq!(fibo_rec(10, 1000), 55);
    }
    #[test]
    fn fibo_rec_n_10_big_uint() {
        assert_eq!(fibo_rec(BigUint::from(10u32), BigUint::from(1000u32)), BigUint::from(55u32));
    }
    #[test]
    fn fibo_seq_n_10_u32() {
        assert_eq!(fibo_seq(1000, 1000_000_000), 849228875);
    }
    #[test]
    fn fibo_seq_n_10_big_uint() {
        assert_eq!(fibo_seq(BigUint::from(1000u32), BigUint::from(1000_000_000u32)), BigUint::from(849228875u32));
    }
    #[test]
    fn fibo_mat_1000000_1000_u32() {
        assert_eq!(fibo_mat(1000_000, 1000), 875);
    }
    #[test]
    fn fibo_mat_1000000_1000_big_uint() {
        assert_eq!(fibo_mat(BigUint::from(1000_000u32), BigUint::from(1000u32)), BigUint::from(875u32));
    }
    #[test]
    fn fibo_mat_req_1000000_1000_u32() {
        assert_eq!(fibo_mat_req(1000_000, 1000), 875);
    }
    #[test]
    fn fibo_mat_req_1000000_1000_big_uint() {
        assert_eq!(fibo_mat_req(BigUint::from(1000_000u32), BigUint::from(1000u32)), BigUint::from(875u32));
    }
    #[test]
    fn fibo_mat_loop_1000000_1000_u32() {
        assert_eq!(fibo_mat_loop(1000_000, 1000), 875);
    }
    #[test]
    fn fibo_mat_loop_1000000_1000_big_uint() {
        assert_eq!(fibo_mat_loop(BigUint::from(1000_000u32), BigUint::from(1000u32)), BigUint::from(875u32));
    }
    */
}
