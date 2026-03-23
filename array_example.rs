/*
-----------------------------------------
File Name: array_example.rs
Description: Rust program to demonstrate array usage and iteration
Author: Jibin Jose
-----------------------------------------

Running Commands:
rustc array_example.rs
./array_example

Output:
First element: 10
Value: 10
Value: 20
Value: 30
Value: 40
Value: 50
-----------------------------------------
*/

fn main() {
    let numbers = [10, 20, 30, 40, 50];

    println!("First element: {}", numbers[0]);

    for i in 0..numbers.len() {
        println!("Value: {}", numbers[i]);
    }
}
