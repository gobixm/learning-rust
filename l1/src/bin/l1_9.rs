use std::io;
use std::io::BufRead;

// Составить программу вывода на экран числа, вводимого
// с клавиатуры. После выводимого числа должно следовать сообще-
// ние «– вот какое число Вы ввели».
fn main() {
    let stdin = io::stdin();
    let mut input: String = String::new();
    
    stdin.lock().read_line(&mut input).expect("Failed to read from stdin");
    
    let number = input.trim().parse::<u64>().expect("Input was not integer");

    println!("{} – вот какое число Вы ввели", number);
}
