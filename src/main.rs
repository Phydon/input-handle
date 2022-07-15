extern crate input_handle as input;

fn main() {
    println!("Enter your input:");
    let inp = input::input();
    println!("Input: {}", inp);
}
