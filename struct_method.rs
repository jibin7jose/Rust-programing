/*
-----------------------------------------
File Name: struct_method.rs
Description: Rust program demonstrating struct with methods (impl) to calculate area
Author: Jibin Jose
-----------------------------------------

Running Commands:
rustc struct_method.rs
./struct_method

Output:
Area: 50
-----------------------------------------
*/

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 10,
        height: 5,
    };

    println!("Area: {}", rect.area());
}
