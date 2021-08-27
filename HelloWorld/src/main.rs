use std::time::{Instant};

fn main() {
    // let n = 50;
    // println!("Hello, world lets run fib tests for size: {}", n);
    // let start = Instant::now();
    // println!("Hello fibtail: {}",fib2(n));
    // let duration = start.elapsed();
    // println!("--->fibtail perf: {:?}",duration);

    // let start = Instant::now();
    // println!("Hello fib: {}",fib(n));
    // let duration = start.elapsed();
    // println!("--->fibt perf: {:?}",duration);

    enum_fun();
}
use std::fmt;


#[derive(Debug)]
enum Number {
    Rational(i32,i32),
    Integer(i32),
    Float(f32)    
}


impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           Number::Rational(a,b) => write!(f, "rat {}/{}",a,b),
           Number::Integer(a) => write!(f, "int {}",a),
           Number::Float(a) => write!(f, "float {}",a),
       }
    }
}


fn enum_fun() {
    println!("enum_fun");

    let n1 = Number::Float(3.33);
    let n2 = Number::Integer(44);
    let n3 = Number::Rational(1,2);

    println!("--->n1 {:?}", n1);
    println!("--->n2 {:?}", n2);
    println!("--->n3 {:?}", n3);

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



