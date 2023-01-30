use std::ops::*;

/// Scalar Types - represent a single value
/// 4 primary scalar types:
/// 1. integers
/// 2. floating point
/// 3. Booleans
/// 4. Characters

pub fn integers() {
    println!("******************************");
    println!("*** Integers ***");
    let a: u8 = 255;
    let (b, is_overflow) = a.overflowing_add(1);
    println!("a = {}, b = {} is_overflow = {}", a, b, is_overflow);
}

pub fn floats() {
    println!("******************************");
    println!("*** Floats ***");

    let a: f64 = 3.5;
    println!("a = {}", a);
}

pub fn numeric_ops() {
    println!("******************************");
    println!("*** Numeric Operations ***");

    let a: i32 = 2;
    let b = 15.5;
    // let sum: f64 = a + b; // ERROR: no implementation for `{integer} + {float}`
    let sum = a.add(3);
    println!("sum = {}", sum);

    // let diff = b - a;   // ERROR: no implementation for `{float} - {integer}`

    // let product = a * b; // ERROR: no implementation for `{integer} * {float}`

    // let quotient = b / a;   // ERROR: no implementation for `{float} / {integer}`
    // println!("quotient = {}", quotient);
    let q = 10.div(a);
    println!("q = {}", q);

    // Conclusion: all the ops are traits/methods of the data type of the RHS operand
    // the LHS operand becomes the args of the traits called 'other'
    // std::ops crate contains the implementations
}

pub fn boolean() {
    let t = true;
    println!("t = {}", t);
    println!("t as u8 = {}", t as u8);

    let f: bool = false;
    println!("f as u8 = {}", f as u8);
}

/// Compound Types - can group multiple values into one type
/// 2 primary compound types:
/// 1. tupes
/// 2. arrays

pub fn tuples() {
    // Syntax: let <var_name>: (<datatype>, <datatype>, ...) = (val1, val2, ...);
}
