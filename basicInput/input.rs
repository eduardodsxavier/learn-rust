use std::io::{self, Read};

fn main() {
    println!("give input: ");

    let mut input = String::new();

    let _ = io::stdin().read_line(&mut input);

    println!("input: {}", input.trim());
}
