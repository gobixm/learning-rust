use std::io;
use std::io::BufRead;

// Составить программу вывода на экран в одну строку трех
// любых чисел, вводимых с клавиатуры, с двумя пробелами между
// ними.
fn main() {    
    let stdin = io::stdin();
    let mut input: String = String::new();
    let numbers: Vec<String> = (0..3)
        .map(|_|{
            input.clear();
            stdin.lock().read_line(&mut input).expect("Failed to read from stdin");
            let number = input.trim().parse::<u64>().expect("Input was not integer");
            number.to_string()
        })        
        .collect();
    
    println!("{}", numbers.join("  "));
}
