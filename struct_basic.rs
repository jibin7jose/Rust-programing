/*
-----------------------------------------
File Name: struct_basic.rs
Description: Rust program demonstrating basic struct definition and usage
Author: Jibin Jose
-----------------------------------------

Running Commands:
rustc struct_basic.rs
./struct_basic

Output:
Name: Jibin
Age: 22
-----------------------------------------
*/

struct Person {
    name: String,
    age: u32,
}

fn main() {
    let person1 = Person {
        name: String::from("Jibin"),
        age: 22,
    };

    println!("Name: {}", person1.name);
    println!("Age: {}", person1.age);
}
