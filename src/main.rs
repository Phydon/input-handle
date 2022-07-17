extern crate input_handle as input;

use input::*;

fn main() {
    println!("Enter your input:");
    let inp = input();
    println!("Input: {}", inp);
}
