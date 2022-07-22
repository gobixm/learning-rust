use std::io;
use std::io::BufRead;

// Составить программу, которая запрашивает название
// футбольной команды и повторяет его на экране со словами «–
// это чемпион!».
fn main() {
    println!("Enter team name");
    
    let stdin = io::stdin();
    let mut input: String = String::new();
    
    stdin.lock().read_line(&mut input).expect("Failed to read from stdin");
    
    println!("{} - это чемпион!", input.trim());
}
