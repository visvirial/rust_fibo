extern crate fibo;

fn run(label: &str, func: fn(u64, u64) -> u64, x: u64) {
    let begin = std::time::Instant::now();
    let m = 1000_000_000;
    let y = func(x, m);
    let elapsed = 1000 * begin.elapsed().as_secs() + begin.elapsed().subsec_millis() as u64;
    println!("{}: n={:>20}, F(n)%{}={:>9} ({:>3}ms)", label, x, m, y, elapsed);
}

fn main() {
    /*
    run("Recursive ", fibo::fibo_rec     ,         35);
    run("Sequencial", fibo::fibo_seq     , 10_000_000);
    run("Matrix    ", fibo::fibo_mat     , 10_000_000);
    run("Mat (req) ", fibo::fibo_mat_req , 18_446_744_073_709_551_615);
    run("Mat (loop)", fibo::fibo_mat_loop, 18_446_744_073_709_551_615);
    */
}
