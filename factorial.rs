/*
-----------------------------------------
File Name: factorial_while.rs
Description: Rust program to calculate factorial using while loop
Author: Jibin Jose
-----------------------------------------

Running Commands:
rustc factorial_while.rs
./factorial_while

Example Output:
Enter a number to calculate its factorial:
5
The factorial of 5 is 120
-----------------------------------------
*/

use std::io;

fn factorial(mut n: u64) -> u64 {
    let mut result = 1;

    while n > 0 {
        result *= n;
        n -= 1;
    }

    result
}

fn main() {
    println!("Enter a number to calculate its factorial:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let num: u64 = input.trim().parse().expect("Please enter a valid number");

    let result = factorial(num);

    println!("The factorial of {} is {}", num, result);
}
