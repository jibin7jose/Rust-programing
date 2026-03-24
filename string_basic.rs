/*
-----------------------------------------
File Name: string_example.rs
Description: Rust program demonstrating difference between string slice (&str) and String, and modifying String
Author: Jibin Jose
-----------------------------------------

Running Commands:
rustc string_example.rs
./string_example

Output:
Name: Jibin
City: Calicut
Updated City: Calicut Kerala
-----------------------------------------
*/

fn main() {
    let name = "Jibin"; // &str (string slice)

    let mut city = String::from("Calicut"); // String (heap allocated)

    println!("Name: {}", name);
    println!("City: {}", city);

    city.push_str(" Kerala");

    println!("Updated City: {}", city);
}
