use std::io;
use std::io::BufRead;

// Составить программу, которая запрашивает имя человека
// и повторяет его на экране.
fn main() {
    println!("Enter your name");
    
    let stdin = io::stdin();
    let mut input: String = String::new();
    
    stdin.lock().read_line(&mut input).expect("Failed to read from stdin");
    
    println!("Your name is {}", input.trim());
}
