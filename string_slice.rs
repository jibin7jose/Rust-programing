/*
-----------------------------------------
File Name: string_slice.rs
Description: Rust program demonstrating string slicing using ranges
Author: Jibin Jose
-----------------------------------------

Running Commands:
rustc string_slice.rs
./string_slice

Output:
Slice 1: Hello
Slice 2: Rust
-----------------------------------------
*/

fn main() {
    let text = String::from("Hello Rust");

    let slice1 = &text[0..5];
    let slice2 = &text[6..10];

    println!("Slice 1: {}", slice1);
    println!("Slice 2: {}", slice2);
}
