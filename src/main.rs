extern crate fibo;

use core::fmt::Display;
use std::env;
use std::process;
use std::str::FromStr;
use num_bigint::BigUint;

fn run<T: Display + Clone>(func: fn(T, T) -> T, n: T, m: T) {
    let begin = std::time::Instant::now();
    let y = func(n.clone(), m.clone());
    let elapsed = 1000 * begin.elapsed().as_secs() + begin.elapsed().subsec_millis() as u64;
    println!("F({}) = {} mod {} ({}ms)", n, y, m, elapsed);
}

fn main() {
    let argv: Vec<String> = env::args().collect();
    if argv.len() < 3 {
        println!("Usage: {} N M - Computes the N-th Fibonacci number with modulo M", argv[0]);
        process::exit(0)
    }
    let n = BigUint::from_str(&argv[1]).unwrap();
    let m = BigUint::from_str(&argv[2]).unwrap();
    run(fibo::fibo_mat_loop, n, m);
}
