use std::io;
use std::io::BufRead;

// Составить программу вывода на экран числа, вводимого
// с клавиатуры. Выводимому числу должно предшествовать со-
// общение «Вы ввели число».
fn main() {
    let stdin = io::stdin();
    let mut input: String = String::new();
    
    stdin.lock().read_line(&mut input).expect("Failed to read from stdin");
    
    let number = input.trim().parse::<u64>().expect("Input was not integer");

    println!("Вы ввели число {}", number);
}
