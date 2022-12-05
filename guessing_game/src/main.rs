use std::io;
use std::cmp::Ordering;
//   The io and cmp library comes from the standard library.
//   Rust has a set of items defined in the standard library.
//   This set is called the prelude.

use rand::Rng; 
//   For random num generation. 
//   rand is dep that we need get from Crate. 

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();
//    In Rust, variables are immutable by default.
//    To make a variable mutable, we add "mut".

//    The :: syntax in the ::new line indicates that
//    new is an associated function of the String type. 
//    This "new" function before "String" creates a new empty string. 

    io::stdin()
        .read_line(&mut guess) 
//         The & indicates that this argument is a reference.

        .expect("Failed to read line");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too Big"),
        Ordering::Equal => println!("You Win!!!"),
    }
}
