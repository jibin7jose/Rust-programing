/*
-----------------------------------------
File Name: borrow_example.rs
Description: Rust program demonstrating borrowing using references (&String)
Author: Jibin Jose
-----------------------------------------

Running Commands:
rustc borrow_example.rs
./borrow_example

Output:
Value: Rust
Still usable: Rust
-----------------------------------------
*/

fn print_value(s: &String) {
    println!("Value: {}", s);
}

fn main() {
    let s1 = String::from("Rust");

    print_value(&s1);

    println!("Still usable: {}", s1);
}
