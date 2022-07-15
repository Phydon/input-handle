use std::{io};

pub fn input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    match input.trim() {
        _ => return input,
    }

    // match input.trim() {
    //     _ => return input.trim().to_string(),
    // }
}
