/*
-----------------------------------------
File Name: dangling_reference.rs
Description: Rust program demonstrating how Rust prevents dangling references by returning ownership safely
Author: Jibin Jose
-----------------------------------------

Running Commands:
rustc dangling_reference.rs
./dangling_reference

Output:
Hello
-----------------------------------------
*/

fn dangle() -> String {
    let s = String::from("Hello");
    s
}

fn main() {
    let result = dangle();
    println!("{}", result);
}
