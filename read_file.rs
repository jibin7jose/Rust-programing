/*
-----------------------------------------
File Name: read_file.rs
Description: Rust program to read a file and handle errors using Result<T, E>
Author: Jibin Jose
-----------------------------------------

Running Commands:
rustc read_file.rs
./read_file

Example Output (if file exists):
File Content:
Hello World

Example Output (if file not found):
Error reading file: The system cannot find the file specified.
-----------------------------------------
*/

use std::fs;

fn main() {
    let content = fs::read_to_string("test.txt");

    match content {
        Ok(text) => println!("File Content:\n{}", text),
        Err(e) => println!("Error reading file: {}", e),
    }
}
