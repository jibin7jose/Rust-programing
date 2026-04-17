/*
-----------------------------------------
File Name: option_example.rs
Description: Rust program demonstrating use of Option<T> with match
Author: Jibin Jose
-----------------------------------------

Running Commands:
rustc option_example.rs
./option_example

Output:
Value: 10
-----------------------------------------
*/

fn main() {
    let number = Some(10);

    match number {
        Some(n) => println!("Value: {}", n),
        None => println!("No value found"),
    }
}
