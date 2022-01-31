use std::io;
//By default, Rust has a few items defined in the standard library 
//that it brings into the scope of every program. 
//This set is called the prelude, 
//and you can see everything in it in the standard library documentation.
//https://doc.rust-lang.org/std/prelude/index.html

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {}", guess);
}
