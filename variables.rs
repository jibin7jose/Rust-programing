/*
-----------------------------------------
File Name: variables.rs
Description: Rust program demonstrating variables and formatted output
Author: Jibin Jose
-----------------------------------------

Running Commands:
rustc variables.rs
./variables

Output:
My name is Jibin and I an 22 years old.
-----------------------------------------
*/

fn main() {
    let name = "Jibin";
    let age = 22;

    println!("My name is {} and I an {} years old.", name, age);
}