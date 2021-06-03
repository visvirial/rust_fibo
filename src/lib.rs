pub mod mat;

use std::ops::Neg;
use num_bigint::BigInt;
use mat::EuclideanDomain;
use mat::Exponent;

pub trait Index: EuclideanDomain + Exponent + PartialOrd + Neg<Output = Self> + Clone {}

impl Index for i64 {}
impl Index for BigInt {}

fn fibo<T: Index>(func: fn(T, T) -> T, n: T, m: T) -> T {
    if n < T::zero() {
        if (n.clone() & T::one()).is_zero() {
            -func(-n, m)
        } else {
            func(-n, m)
        }
    } else {
        func(n, m)
    }
}

fn fibo_rec_pos<T: Index>(n: T, m: T) -> T {
    if n.is_zero() {
        return T::zero();
    }
    if n.is_one() {
        return T::one();
    }
    (fibo_rec(n.clone() - T::one() - T::one(), m.clone()) + fibo_rec(n - T::one(), m.clone())) % m
}

/// Compute the n-th Fibonacci number modulo m using recurssion (F(n) <= F(n-1) + F(n-2)).
/// Since the recurrsion computes the same Fibonacci numbers multiple times,
/// it is very slow to compute.
/// The computation time is O(F(n)) = O(φ^n).
/// Maybe n = ~40 is the maximum value that this function can compute the Fibonacci number.
pub fn fibo_rec<T: Index>(n: T, m: T) -> T {
    fibo(fibo_rec_pos, n, m)
}

fn fibo_seq_pos<T: Index>(n: T, m: T) -> T {
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

/// Compute the n-th Fibonacci number modulo m using sequencial computation ((F(n), F(n-1)) <= (F(n-1) + F(n-2), F(n-1))).
/// The computation time is O(n).
pub fn fibo_seq<T: Index>(n: T, m: T) -> T {
    fibo(fibo_seq_pos, n, m)
}

fn fibo_mat_pos<T: Index>(n: T, m: T) -> T {
    let mut t = mat::Mat::one();
    let mut i = T::zero();
    while i != n {
        t = t.mul(&mat::Mat::q_matrix(), m.clone());
        i += T::one();
    }
    t.0.1
}

/// Compute the n-th Fibonacci number modulo m using a Q-matrix with sequential matrix
/// multiplication.
/// The computation time is O(n).
pub fn fibo_mat<T: Index>(n: T, m: T) -> T {
    fibo(fibo_mat_pos, n, m)
}

fn fibo_mat_req_pos<T: Index>(n: T, m: T) -> T {
    mat::exp_req(&mat::Mat::q_matrix(), n, m).0.1
}

/// Compute the n-th Fibonacci number modulo m using a Q-matrix with repeated squaring
/// (using recurrsion).
/// The computation time is O(log(n)).
pub fn fibo_mat_req<T: Index>(n: T, m: T) -> T {
    fibo(fibo_mat_req_pos, n, m)
}

fn fibo_mat_loop_pos<T: Index>(n: T, m: T) -> T {
    mat::Mat::q_matrix().pow(n, m).0.1
}

/// Compute the n-th Fibonacci number modulo m using a Q-matrix with repeated squaring
/// (using loops).
/// Slightly faster than `fibo_mat_req()` since there are no function call overhead.
/// The computation time is O(log(n)).
pub fn fibo_mat_loop<T: Index>(n: T, m: T) -> T {
    fibo(fibo_mat_loop_pos, n, m)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fibo_rec_10_1000_i64() {
        assert_eq!(fibo_rec(10i64, 1000i64), 55i64);
    }
    #[test]
    fn fibo_rec_minus10_1000_i64() {
        assert_eq!(fibo_rec(-10i64, 1000i64), -55i64);
    }
    #[test]
    fn fibo_rec_10_1000_big_int() {
        assert_eq!(fibo_rec(BigInt::from(10i64), BigInt::from(1000i64)), BigInt::from(55i64));
    }
    #[test]
    fn fibo_seq_1000_1000000000_i64() {
        assert_eq!(fibo_seq(1000i64, 1000_000_000i64), 849228875i64);
    }
    #[test]
    fn fibo_seq_minus1000_1000000000_i64() {
        assert_eq!(fibo_seq(-1000i64, 1000_000_000i64), -849228875i64);
    }
    #[test]
    fn fibo_seq_1000_1000000000_big_int() {
        assert_eq!(fibo_seq(BigInt::from(1000i64), BigInt::from(1000_000_000i64)), BigInt::from(849228875i64));
    }
    #[test]
    fn fibo_mat_1000000_1000_i64() {
        assert_eq!(fibo_mat(1000_000i64, 1000i64), 875i64);
    }
    #[test]
    fn fibo_mat_minus1000000_1000_i64() {
        assert_eq!(fibo_mat(-1000_000i64, 1000i64), -875i64);
    }
    #[test]
    fn fibo_mat_1000000_1000_big_int() {
        assert_eq!(fibo_mat(BigInt::from(1000i64), BigInt::from(1000_000_000i64)), BigInt::from(849228875i64));
    }
    #[test]
    fn fibo_mat_req_1000000_1000_i64() {
        assert_eq!(fibo_mat_req(1000_000i64, 1000i64), 875i64);
    }
    #[test]
    fn fibo_mat_req_minus1000000_1000_i64() {
        assert_eq!(fibo_mat_req(-1000_000i64, 1000i64), -875i64);
    }
    #[test]
    fn fibo_mat_req_1000000_1000_big_int() {
        assert_eq!(fibo_mat_req(BigInt::from(1000_000i64), BigInt::from(1000i64)), BigInt::from(875i64));
    }
    #[test]
    fn fibo_mat_loop_1000000_1000_i64() {
        assert_eq!(fibo_mat_loop(1000_000i64, 1000i64), 875i64);
    }
    #[test]
    fn fibo_mat_loop_minus1000000_1000_i64() {
        assert_eq!(fibo_mat_loop(-1000_000i64, 1000i64), -875i64);
    }
    #[test]
    fn fibo_mat_loop_1000000_1000_big_int() {
        assert_eq!(fibo_mat_loop(BigInt::from(1000_000i64), BigInt::from(1000i64)), BigInt::from(875i64));
    }
}
