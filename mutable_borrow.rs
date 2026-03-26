/*
-----------------------------------------
File Name: mutable_borrow.rs
Description: Rust program demonstrating mutable borrowing to modify a String
Author: Jibin Jose
-----------------------------------------

Running Commands:
rustc mutable_borrow.rs
./mutable_borrow

Output:
Hello World
-----------------------------------------
*/

fn update_value(s: &mut String) {
    s.push_str(" World");
}

fn main() {
    let mut s1 = String::from("Hello");

    update_value(&mut s1);

    println!("{}", s1);
}
