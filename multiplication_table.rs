/*
-----------------------------------------
File Name: multiplication_table.rs
Description: Rust program to print multiplication table of a given number
Author: Jibin Jose
-----------------------------------------

Running Commands:
rustc multiplication_table.rs
./multiplication_table

Example Output:
Enter a number:
5
5 x 1 = 5
5 x 2 = 10
5 x 3 = 15
5 x 4 = 20
5 x 5 = 25
5 x 6 = 30
5 x 7 = 35
5 x 8 = 40
5 x 9 = 45
5 x 10 = 50
-----------------------------------------
*/

use std::io;

fn main() {

    let mut number = String::new();

    println!("Enter a number:");

    io::stdin()
        .read_line(&mut number)
        .expect("Failed");

    let num: i32 = number.trim().parse().expect("Invalid number");

    for i in 1..=10 {
        println!("{} x {} = {}", num, i, num * i);
    }
}
