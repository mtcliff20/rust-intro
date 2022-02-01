use std::io;
 // standard input output library
//If a type you want to use isn’t in the prelude, you have to bring 
//that type into scope explicitly with a use statement.

//By default, Rust has a few items defined in the standard library 
//that it brings into the scope of every program. 
//This set is called the prelude, 
//and you can see everything in it in the standard library documentation.
//https://doc.rust-lang.org/std/prelude/index.html

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");
    // variables are immutable by default so we add "mut" to allow mutability
    // let apples = 5; // immutable
    // let mut bananas = 5; // mutable
    let mut guess = String::new();
    // String::new, a function that returns a new instance of a String
    // The :: syntax in the ::new line indicates that new is an associated function of the String type.
    // An associated function is a function that’s implemented on a type, in this case String.
    // This new function creates a new, empty string. You’ll find a new function on many types,
    // because it’s a common name for a function that makes a new value of some kind.
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {}", guess);
}


