/// # Declare External Dependencies
/// `extern crate` <crate_name>;
extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
/// # Bring Things in Scope
/// `use` <lib>`::`<module>`::`<func>;
use std::io;

/// # Declare and initialise a variable
/// `let [mut]` <var_name>`: `<data_type> `=` <value>
/// `mut` - mutable variable
/// instantiate an empty string - `String::new()`
/// `&` - is a reference to a variable; ownership does not change
pub fn guessing_game() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            } //TODO: ends with comma
        }
    }

    println!("The secret number was: {}", secret_number);

    println!("Game Over!");
}
