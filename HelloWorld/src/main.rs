use std::time::{Instant};

fn main() {
    let n = 50;
    println!("Hello, world lets run fib tests for size: {}", n);
    let start = Instant::now();
    println!("Hello fibtail: {}",fib2(n));
    let duration = start.elapsed();
    println!("--->fibtail perf: {:?}",duration);

    let start = Instant::now();
    println!("Hello fib: {}",fib(n));
    let duration = start.elapsed();
    println!("--->fibt perf: {:?}",duration);
}

fn fib(n: i64) -> i64 {
    if n == 0 { return 0;}
    if n == 1 { return 1;}
    return fib (n-1) + fib(n-2);
}

fn fib2(n: i64) -> i64 {
    fn fib2_inner(a: i64, b:i64, n:i64) -> i64 {
        if n == 0 { return a;}
        return fib2_inner(b, a+b, n-1)
    }
    return fib2_inner(0, 1, n)
}



