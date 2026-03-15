/*
-----------------------------------------
File Name: for_loop.rs
Description: Rust program demonstrating a for loop to print numbers from 1 to 10
Author: Jibin Jose
-----------------------------------------

Running Commands:
rustc for_loop.rs
./for_loop

Output:
Number: 1
Number: 2
Number: 3
Number: 4
Number: 5
Number: 6
Number: 7
Number: 8
Number: 9
Number: 10
-----------------------------------------
*/

fn main() {
    for i in 1..=10 {
        println!("Number: {}", i);
    }
}
