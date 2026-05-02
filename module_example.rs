/*
-----------------------------------------
File Name: module_example.rs
Description: Rust program demonstrating use of modules and public functions
Author: Jibin Jose
-----------------------------------------

Running Commands:
rustc module_example.rs
./module_example

Output:
Sum: 8
Difference: 2
-----------------------------------------
*/

mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn sub(a: i32, b: i32) -> i32 {
        a - b
    }
}

fn main() {
    let sum = math::add(5, 3);
    let diff = math::sub(5, 3);

    println!("Sum: {}", sum);
    println!("Difference: {}", diff);
}
