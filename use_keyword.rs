/*
-----------------------------------------
File Name: use_keyword.rs
Description: Rust program demonstrating use of `use` keyword to simplify module access
Author: Jibin Jose
-----------------------------------------

Running Commands:
rustc use_keyword.rs
./use_keyword

Output:
Result: 20
-----------------------------------------
*/

mod math {
    pub fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }
}

use math::multiply;

fn main() {
    let result = multiply(4, 5);
    println!("Result: {}", result);
}
