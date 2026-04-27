/*
-----------------------------------------
File Name: write_file.rs
Description: Rust program to write data to a file using fs::write with error handling
Author: Jibin Jose
-----------------------------------------

Running Commands:
rustc write_file.rs
./write_file

Example Output:
File written successfully

File Created:
output.txt → contains: Hello from Rust!
-----------------------------------------
*/

use std::fs;

fn main() {
    let data = "Hello from Rust!";

    match fs::write("output.txt", data) {
        Ok(_) => println!("File written successfully"),
        Err(e) => println!("Error writing file: {}", e),
    }
}
