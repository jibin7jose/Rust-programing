/*
-----------------------------------------
File Name: even_odd.rs
Description: Rust program to check whether a number is even or odd
Author: Jibin Jose
-----------------------------------------

Running Commands:
rustc even_odd.rs
./even_odd

Example Output:
Enter a number:
8
8 is an even number
-----------------------------------------
*/

use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter a number:");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let num: i32 = input
        .trim()
        .parse()
        .expect("Please enter a valid number");

    if num % 2 == 0 {
        println!("{} is an even number", num);
    } else {
        println!("{} is an odd number", num);
    }
}