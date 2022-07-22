use std::io;
use std::io::BufRead;

// Напишите программу, в которую вводится имя человека
// и выводится на экран приветствие в виде слова «Привет», после
// которого должна стоять запятая, введенное имя и восклицатель-
// ный знак. После запятой должен стоять пробел, а перед воскли-
// цательным знаком пробела быть не должно.
fn main() {
    println!("Enter name");
    
    let stdin = io::stdin();
    let mut input: String = String::new();
    
    stdin.lock().read_line(&mut input).expect("Failed to read from stdin");
    
    println!("Привет, {}!", input.trim());
}
