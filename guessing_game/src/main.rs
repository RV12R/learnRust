use std::io;
//   The io library comes from the standard library.
//   Rust has a set of items defined in the standard library.
//   This set is called the prelude. 

fn main() {
    println!("Guess the number!");

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
}
