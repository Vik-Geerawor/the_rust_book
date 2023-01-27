/// variables are immutable by default
/// to make them mutable use the `mut` keyword
pub fn variables() {
    println!("******************************");
    println!("*** Variables ***");
    let mut x = 5;
    println!("let mut X = {}", x);

    x = 6;
    println!("'X = 6' = {}", x);
}

/// constants needs to be declared with:
/// 1. `const` keyword
/// 2. data type
pub fn constants() {
    println!("******************************");
    println!("*** CONSTANTS ***");
    const PI: f64 = 3.1415;

    println!("const PI: f64 = {}", PI);
}

/// `let` creates a new variable everytime
/// and assigns it the value from the expression on the RHS
/// the old variable is dereferenced, i.e. is lost
pub fn shadowing() {
    println!("******************************");
    println!("*** Shadowing ***");
    let x = 5;

    println!("let x = {}", x);

    let x = x + 1;
    // let x += 1;                         //ERROR: can't reassign to an uninitialized variable
    println!("'let x = x + 1' = {}", x);
    println!("let x += 1 - ERROR - can't reassign to an uninitialized variable");
}
