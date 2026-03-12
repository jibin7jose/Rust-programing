/*
-----------------------------------------
File Name: input.rs
Description: Rust program to take user input and greet the user
Author: Jibin Jose
-----------------------------------------

Running Commands:
rustc input.rs
./input

Example Output:
What is your name?
Jibin
Hello, Jibin!
-----------------------------------------
*/

use std::io;

fn main() {
    let mut name = String::new();

    println!("What is your name?");

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    println!("Hello, {}!", name.trim());
}