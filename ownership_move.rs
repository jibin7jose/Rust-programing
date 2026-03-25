/*
-----------------------------------------
File Name: ownership_move.rs
Description: Rust program demonstrating ownership move (value transfer)
Author: Jibin Jose
-----------------------------------------

Running Commands:
rustc ownership_move.rs
./ownership_move

Output:
Hello
-----------------------------------------
*/

fn main() {
    let s1 = String::from("Hello");
    let s2 = s1;

    println!("{}", s2);
    // println!("{}", s1); ❌ ERROR (ownership moved)
}
