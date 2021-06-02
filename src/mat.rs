
#[derive(Debug, PartialEq, Clone)]
pub struct Mat(pub (u64, u64), pub (u64, u64));

impl Mat {
    pub const O: Mat = Mat((0, 0), (0, 0));
    pub const I: Mat = Mat((1, 0), (0, 1));
    pub const M: Mat = Mat((0, 1), (1, 1));
}

fn mul(x: &Mat, y: &Mat, m: u64) -> Mat {
    Mat(( (x.0.0 * y.0.0 + x.0.1 * y.1.0) % m, (x.0.0 * y.0.1 + x.0.1 * y.1.1) % m ),
        ( (x.1.0 * y.0.0 + x.1.1 * y.1.0) % m, (x.1.0 * y.0.1 + x.1.1 * y.1.1) % m ))
}

impl Mat {
    pub fn mul(&self, other: &Self, m: u64) -> Self {
        mul(self, other, m)
    }
}

pub fn exp_req(x: &Mat, n: u64, m: u64) -> Mat {
    if n == 0 {
        return Mat::I;
    }
    let mut y = exp_req(x, n >> 1, m);
    y = y.mul(&y, m);
    if n % 2 == 0 {
        y
    } else {
        y.mul(&x, m)
    }
}

pub fn exp_loop(x: &Mat, mut n: u64, m: u64) -> Mat {
    let mut y: Mat = x.clone();
    let mut z = Mat::I;
    while n > 0 {
        if n % 2 == 1 {
            z = z.mul(&y, m);
        }
        y = y.mul(&y, m);
        n >>= 1;
    }
    z
}

impl Mat {
    pub fn pow(&self, n: u64, m: u64) -> Self {
        exp_loop(self, n, m)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mul_1234_5678() {
        let a = Mat((1, 2), (3, 4));
        let b = Mat((5, 6), (7, 8));
        let c = Mat((19, 22), (43, 50));
        assert_eq!(a.mul(&b, 1000), c);
    }
    #[test]
    fn exp_req_1234_100() {
        let a = Mat((1, 2), (3, 4));
        let b = Mat((999, 250), (875, 874));
        assert_eq!(exp_req(&a, 100, 1000), b);
    }
    #[test]
    fn exp_loop_1234_100() {
        let a = Mat((1, 2), (3, 4));
        let b = Mat((999, 250), (875, 874));
        assert_eq!(exp_loop(&a, 100, 1000), b);
    }
}

