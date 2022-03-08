fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    let guess: u32 = "42".parse().expect("Not a number!");

    // Floating Point
        let x = 2.0; // f64
        let y: f32 = 3.0; // f32

    // Numeric Operations

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

    // BOOLEAN
        let t = true;
        let f: bool = false; // with explicit type annotation

    // CHAR TYPE
        let c = 'z';
        let z = 'ℤ';
        let heart_eyed_cat = '😻';

    // THE TUPLE TYPE
        let tup = (500, 6.4, 1);
        let (x, y, z) = tup;
        println!("The value of y is: {}", y);

    // ARRAY TYPE
        let a = [1, 2, 3, 4, 5];

    // CALLING FUNCTIONS
        another_function();
    // CALLING FUNCTION WITH PARAMS
        another_function_with_param(5);
    // CALLING FUNCTION WITH MULTIPLE PARAMS
        print_labeled_measurement(5, 'h');
    // CALLING FUNCTION WITH RETURN VALUE
        let x = function_with_return_value();
        println!("The value of x is: {}", x);

    // Basic Comments
    let lucky_number = 7; // I’m feeling lucky today


}

fn another_function() {
    println!("Another function.");
}

fn another_function_with_param(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn function_with_return_value() -> i32 {
    5
}
