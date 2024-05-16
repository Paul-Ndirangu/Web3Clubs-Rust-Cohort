use std::io;

fn main() {
    println!("Enter an integer:");

    let mut input = String:: new();

    // Read input from the terminal
    io::stdin().read_line(&mut input)
    .expect("Failed to read line");

    // Parse input as an integer
    let input: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter an integer.");
            return;
        }
    };

    // Print numbers from 0 to input
    for i in 0..=input {
        println!("{}", i);
    }
}
