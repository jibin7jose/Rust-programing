/*
-----------------------------------------
File Name: result_example.rs
Description: Rust program demonstrating Result<T, E> for error handling while opening a file
Author: Jibin Jose
-----------------------------------------

Running Commands:
rustc result_example.rs
./result_example

Example Output (if file exists):
File opened successfully

Example Output (if file not found):
Error opening file: The system cannot find the file specified.
-----------------------------------------
*/

use std::fs::File;

fn main() {
    let file = File::open("test.txt");

    match file {
        Ok(_f) => println!("File opened successfully"),
        Err(e) => println!("Error opening file: {}", e),
    }
}
