use std::io;

fn main() {
    // Prompt the user to enter the first number
    println!("Hello, world!");

    let mut input1 = String::new();
    io::stdin().read_line(&mut input1)
     .expect("Failed to read line");
    let input1: f64 = input1.trim()
     .parse().expect("Please enter a valid number");

    // Prompt the user to enter the second number
    println!("Enter the second number:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2)
     .expect("Failed to read line");
    let input2: f64 = input2.trim()
     .parse().expect("Please enter a valid number");

    // Perform addition
    let addition_result = input1 + input2;
    println!("Addition: {} + {} = {}", input1, input2 addition_result);

    // Perform subtraction
    let subtraction_result = input1 - input2;
    println!("Subtraction: {} - {} = {}", input1, input2, subtraction_result);

    //Perform multiplication
    let multiplication_result = input1 * input2;
    println!("Multiplication: {} * {} = {}",input1, input2, multiplication_result);

    // Perform division
    if input2 != 0.0 {
        let division_result = input1 / input2;
        println!("Division: {} / {} = {}",input1, input2, division_result);
    } else {
        println!("Division by zero is not allowed.");
    }

    // Perform modulo
    if input2 != 0.0 {
        let modulo_result = input1 % input2;
        println!("Modulo: {} % {} = {}", input1, input2, modulo_result);
    } else {
        println!("Modulo by zero is not allowed.");
    }

    // Perform exponentiation
    let exponentiation_result = input1.powf(input2);
    println!("{} ^ {} = {}", input1, input2, exponentiation_result);
}
