/*
-----------------------------------------
File Name: string_concatenation.rs
Description: Rust program to concatenate two strings and find their length
Author: Jibin Jose
-----------------------------------------

Running Commands:
rustc string_concatenation.rs
./string_concatenation

Output:
Combined String: Hello World
Length: 11
-----------------------------------------
*/

fn main() {
    let mut str1 = String::from("Hello");
    let str2 = String::from("World");

    str1.push(' ');
    str1.push_str(&str2);

    println!("Combined String: {}", str1);

    println!("Length: {}", str1.len());
}
