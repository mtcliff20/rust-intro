fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    let guess: u32 = "42".parse().expect("Not a number!");

        // addition
        let sum = 5 + 10;

        // subtraction
        let difference = 95.5 - 4.3;
    
        // multiplication
        let product = 4 * 30;
    
        // division
        let quotient = 56.7 / 32.2;
        let floored = 2 / 3; // Results in 0
    
        // remainder
        let remainder = 43 % 5;

        //BOOLEAN
        let t = true;

        let f: bool = false; // with explicit type annotation


        // CHAR TYPE
        let c = 'z';
        let z = 'ℤ';
        let heart_eyed_cat = '😻';

        // THE TUPLE TYPE
        let tup: (i32, f64, u8) = (500, 6.4, 1);

}
