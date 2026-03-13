/*
-----------------------------------------
File Name: compare_numbers.rs
Description: Rust program to compare two numbers
Author: Jibin Jose
-----------------------------------------

Running Commands:
rustc compare_numbers.rs
./compare_numbers

Example Output:
Enter the first number:
10
Enter the second number:
5
10 is greater than 5
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

    if num1 > num2 {
        println!("{} is greater than {}", num1, num2);
    } else if num1 < num2 {
        println!("{} is less than {}", num1, num2);
    } else {
        println!("{} is equal to {}", num1, num2);
    }
}