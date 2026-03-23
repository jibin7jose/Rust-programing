/*
-----------------------------------------
File Name: vector_example.rs
Description: Rust program demonstrating vector creation, push operation, and iteration
Author: Jibin Jose
-----------------------------------------

Running Commands:
rustc vector_example.rs
./vector_example

Output:
Vector: [1, 2, 3]
Value: 1
Value: 2
Value: 3
-----------------------------------------
*/

fn main() {
    let mut numbers = Vec::new();

    numbers.push(1);
    numbers.push(2);
    numbers.push(3);

    println!("Vector: {:?}", numbers);

    for num in &numbers {
        println!("Value: {}", num);
    }
}
