/*
-----------------------------------------
File Name: string_basic.rs
Description: Rust program demonstrating string slice (&str) and String with modification
Author: Jibin Jose
-----------------------------------------

Running Commands:
rustc string_basic.rs
./string_basic

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
