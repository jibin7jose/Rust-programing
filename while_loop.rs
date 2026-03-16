/*
-----------------------------------------
File Name: while_loop.rs
Description: Rust program demonstrating a while loop to print numbers from 1 to 10
Author: Jibin Jose
-----------------------------------------

Running Commands:
rustc while_loop.rs
./while_loop

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
    let mut i = 1;

    while i <= 10 {
        println!("Number: {}", i);
        i += 1;
    }
}
