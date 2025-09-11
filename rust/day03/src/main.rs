use std::io;

fn main() {
    println!("Simple calculator");
    println!("Available operations: +, -, *, /");
    println!("Enter your expression (e.g., 5 + 3): ");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let tokens: Vec<&str> = input.trim().split_whitespace().collect();
}
