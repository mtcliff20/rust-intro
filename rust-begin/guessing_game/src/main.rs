use std::io;
// standard input output library
// If a type you want to use isn’t in the prelude, you have to bring 
// that type into scope explicitly with a use statement.
use rand::Rng;
// Rust doesn’t yet include random number functionality in its standard library.
// However, the Rust team does provide a rand crate with said functionality.


use std::cmp::Ordering;
// First we add another use statement, bringing a type called std::cmp::Ordering into scope from the 
// standard library. The Ordering type is another enum and has the variants Less, Greater, and Equal.
// These are the three outcomes that are possible when you compare two values.

// Note: You won’t just know which traits to use and which methods and functions to call from a crate,
// so each crate has documentation with instructions for using it. Another neat feature of Cargo is that running the 
// cargo doc --open command will build documentation provided by all of your dependencies locally and open it in your browser.
// If you’re interested in other functionality in the rand crate, for example,
// run cargo doc --open and click rand in the sidebar on the left.


//By default, Rust has a few items defined in the standard library 
//that it brings into the scope of every program. 
//This set is called the prelude, 
//and you can see everything in it in the standard library documentation.
//https://doc.rust-lang.org/std/prelude/index.html

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    // The gen_range method takes a range expression as an argument and generates a random number in the range. 
    // The kind of range expression we’re using here takes the form start..end and is inclusive on the lower bound but
    // exclusive on the upper bound, so we need to specify 1..101 to request a number between 1 and 100. Alternatively,
    // we could pass the range 1..=100, which is equivalent.

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

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

