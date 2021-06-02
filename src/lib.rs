
pub mod mat;

pub fn fibo_rec(x: u64, m: u64) -> u64 {
    if x == 0 {
        return 0;
    }
    if x == 1 {
        return 1;
    }
    (fibo_rec(x - 2, m) + fibo_rec(x - 1, m)) % m
}

pub fn fibo_seq(x: u64, m: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;
    for _i in 0..x {
        let t = (a + b) % m;
        a = b;
        b = t;
    }
    a
}

pub fn fibo_mat(x: u64, m: u64) -> u64 {
    let mut t = mat::Mat::I;
    for _i in 0..x {
        t = t.mul(&mat::Mat::M, m);
    }
    t.0.1
}

pub fn fibo_mat_req(x: u64, m: u64) -> u64 {
    mat::exp_req(&mat::Mat::M, x, m).0.1
}

pub fn fibo_mat_loop(x: u64, m: u64) -> u64 {
    mat::Mat::M.pow(x, m).0.1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fibo_rec_n_10() {
        assert_eq!(fibo_rec(10, 1000), 55);
    }
    #[test]
    fn fibo_seq_n_10() {
        assert_eq!(fibo_seq(1000, 1000_000_000), 849228875);
    }
    #[test]
    fn fibo_mat_1000_000_1000() {
        assert_eq!(fibo_mat(1000_000, 1000), 875);
    }
    #[test]
    fn fibo_mat_req_1000_000_1000() {
        assert_eq!(fibo_mat_req(1000_000, 1000), 875);
    }
    #[test]
    fn fibo_mat_loop_1000_000_1000() {
        assert_eq!(fibo_mat_loop(1000_000, 1000), 875);
    }
}
