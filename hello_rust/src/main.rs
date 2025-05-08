use std::io;

fn main() {
    println!("Welcome to Rust !");
    println!("Please enter your name:");

    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let name = name.trim(); // Remove whitespace and newline
    println!("Hello, {}! Welcome to your first Rust program!", name);
}