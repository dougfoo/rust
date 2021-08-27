
fn main() {
    enum_fun();
}
use std::fmt;

// #[derive(Debug)]
enum Number {
    Rational(i32,i32),
    Integer(i32),
    Float(f32)    
}

impl Number {
    fn show(&self) {
        println!("call ({})", self);
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           Number::Rational(a,b) => write!(f, "rat {}/{} = {:.2}",a,b,a as f32 / b as f32),
           Number::Integer(a) => write!(f, "int {}",a),
           Number::Float(a) => write!(f, "float {}",a),
       }
    }
}

fn to_string(n:Number) -> String {
    match n {
        Number::Rational(a,b)=> format!("{}",a as f32 / b as f32),
        Number::Integer(a)=> format!("{}",a),
        Number::Float(a)=> format!("{}",a),
    }
}

fn add(l:Number, r:Number) -> Number {
    let left = match l {
        Number::Rational(a,b) => (a as f32 / b as f32),
        Number::Integer(a) => a as f32,
        Number::Float(a) => a as f32,
    };
    let right = match r {
        Number::Rational(a,b) => (a as f32 / b as f32),
        Number::Integer(a) => a as f32,
        Number::Float(a) => a as f32,
    };
    return Number::Float(left + right);
}

// fn get_rat(rat: Number::Rational) -> Option<f32> {
//     if rat(a,b) != 0 {
//         return Some(rat::num / rat::den);
//     }
//     None;
// }

use std::error::Error;
#[derive(Debug)]
pub enum NumberError {
    NotRational,
    DivideByZero,
}

fn get_rat(rat:Number) -> Option<f32> {
    if let Number::Rational(a, b @ 0) = rat {
        return None;
    }
    match rat {
        Number::Rational(a,b) => Some(a as f32 / b as f32),
        _ => None
    }
}


fn get_rat2(rat: Number) -> Result<f32, NumberError> {
    match rat {
        Number::Rational(a,b) => Ok(a as f32 / b as f32),
        _ => Err(NumberError::NotRational),
    }
}
 
fn checked_division(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        None
    } else {
        Some(dividend / divisor)
    }
}

fn try_division(dividend: i32, divisor: i32) {
    match checked_division(dividend, divisor) {
        None => println!("{} / {} failed!", dividend, divisor),
        Some(quotient) => {
            println!("{} / {} = {}", dividend, divisor, quotient)
        },
    }
}

fn enum_fun() {
    println!("enum_fun");

    let nums = [Number::Float(3.33), Number::Integer(44),Number::Rational(13,2)];
    for n in nums {  n.show();  }
    
    let rat2     = get_rat(Number::Rational(13,3)); // returns Ok(0)
    let not_rat2 = get_rat(Number::Rational(13,0)); // returns Err(div by 0)

    println!("rat2 {}", rat2.unwrap());
//    println!("not_rat2 {}", not_rat2.unwrap());

    // let s1 = to_string(n1);
    // let s2 = to_string(n2);
    // let s3 = to_string(n3);

    // println!("{}",s1);


    // println!("{}",s2);
    // println!("{}",s3);
    // println!("add {}", add(n1,n2));

    let result = get_rat(Number::Rational(13,0));
    match result {
        Some(val) => {                      // similarly to handle Ok()
            println!("rational is: {}", val)
        },
        _ => println!("failed for some reason"),
    }  // None or Err
}



