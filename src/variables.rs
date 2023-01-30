/// variables are immutable by default
/// to make them mutable use the `mut` keyword
pub fn variables() {
    println!("******************************");
    println!("*** Variables ***");
    let mut x = 5;
    println!("'let mut x = 5': x = {}", x);

    x = 6;
    println!("'x = 6': x = {}", x);

    // x = "hello";        //ERROR - expected integer, found `&str`

    // Conclusion: 'mut' allows a variable to take different value of the same type
}

/// constants needs to be declared with:
/// 1. `const` keyword
/// 2. data type
pub fn constants() {
    println!("******************************");
    println!("*** CONSTANTS ***");
    const PI: f64 = 3.1415;

    println!("'const PI: f64 = 3.14': PI = {}", PI);

    // Conclusions: Syntax: const <VAR_NAME>: <data_type> = <value>;
}

/// `let` creates a new variable everytime
/// and assigns it the value from the expression on the RHS
/// the old variable is dereferenced, i.e. is lost
pub fn shadowing() {
    println!("******************************");
    println!("*** Shadowing ***");
    let x = 5;
    println!("'let x = 5': x = {}", x);

    let x = x + 1;
    // let x += 1;                         //ERROR: can't reassign to an uninitialized variable
    println!("'let x = x + 1': x = {}", x);
    println!("'let x += 1': ERROR - can't reassign to an uninitialized variable");

    // lifespan of this x ends with the block
    {
        let x = 3.1415;
        println!("Block: 'let x = 3.1415': x = {}", x);
    }
    println!("After Block: x = {}", x);

    let x = "hello";
    println!("'let x = \"hello\"': x = {}", x);

    // Conclusion: let x creates a new variable and its value exists only within the block
}
