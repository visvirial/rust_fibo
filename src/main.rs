
extern crate fibo;
use fibo::mat;

fn fibo_rec(x: u64, m: u64) -> u64 {
    if x == 0 {
        return 0;
    }
    if x == 1 {
        return 1;
    }
    (fibo_rec(x - 2, m) + fibo_rec(x - 1, m)) % m
}

#[test]
fn fibo_rec_n_10() {
    assert_eq!(fibo_rec(10, 1000), 55);
}

fn fibo_seq(x: u64, m: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;
    for _i in 0..x {
        let t = (a + b) % m;
        a = b;
        b = t;
    }
    a
}

#[test]
fn fibo_seq_n_10() {
    assert_eq!(fibo_seq(1000, 1000_000_000), 849228875);
}

fn fibo_mat(x: u64, m: u64) -> u64 {
    let mut t = mat::Mat::I;
    for _i in 0..x {
        t = t.mul(&mat::Mat::M, m);
    }
    t.0.1
}

#[test]
fn fibo_mat_1000_000_1000() {
    assert_eq!(fibo_mat(1000_000, 1000), 875);
}

fn fibo_mat_req(x: u64, m: u64) -> u64 {
    mat::exp_req(&mat::Mat::M, x, m).0.1
}

#[test]
fn fibo_mat_req_1000_000_1000() {
    assert_eq!(fibo_mat_req(1000_000, 1000), 875);
}

fn fibo_mat_loop(x: u64, m: u64) -> u64 {
    mat::Mat::M.pow(x, m).0.1
}

#[test]
fn fibo_mat_loop_1000_000_1000() {
    assert_eq!(fibo_mat_loop(1000_000, 1000), 875);
}

fn run(label: &str, func: fn(u64, u64) -> u64, x: u64) {
    let begin = std::time::Instant::now();
    let m = 1000_000_000;
    let y = func(x, m);
    let elapsed = 1000 * begin.elapsed().as_secs() + begin.elapsed().subsec_millis() as u64;
    println!("{}: n={:>20}, F(n)%{}={:>9} ({:>3}ms)", label, x, m, y, elapsed);
}

fn main() {
    run("Recursive ", fibo_rec     ,         35);
    run("Sequencial", fibo_seq     , 10_000_000);
    run("Matrix    ", fibo_mat     , 10_000_000);
    run("Mat (req) ", fibo_mat_req , 18_446_744_073_709_551_615);
    run("Mat (loop)", fibo_mat_loop, 18_446_744_073_709_551_615);
}

