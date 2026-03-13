/*
-----------------------------------------
File Name: calculator.rs
Description: Rust program to add two numbers using user input
Author: Jibin Jose
-----------------------------------------

Running Commands:
rustc calculator.rs
./calculator

Example Output:
Enter the first number:
3
Enter the second number:
4
The sum of 3 and 4 is 7
-----------------------------------------
*/

use std::io;

fn main() {
    let mut num1 = String::new();
    let mut num2 = String::new();

    println!("Enter the first number:");
    io::stdin()
        .read_line(&mut num1)
        .expect("Failed to read line");

    println!("Enter the second number:");
    io::stdin()
        .read_line(&mut num2)
        .expect("Failed to read line");

    let num1: f64 = num1.trim().parse().expect("Please enter a valid number");
    let num2: f64 = num2.trim().parse().expect("Please enter a valid number");

    let sum = num1 + num2;

    println!("The sum of {} and {} is {}", num1, num2, sum);
}