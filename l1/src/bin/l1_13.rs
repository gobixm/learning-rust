use std::io;
use std::io::BufRead;

// Напишите программу, в которую вводится целое число,
// после чего на экран выводится следующее и предыдущее целое
// число. Например, при вводе числа 15 на экран должно быть вы-
// ведено:
// Следующее за числом 15 число – 16.
// Для числа 15 предыдущее число – 14.
fn main() {    
    let stdin = io::stdin();
    let mut input: String = String::new();
    
    stdin.lock().read_line(&mut input).expect("Failed to read from stdin");

    let number = input.trim().parse::<u64>().expect("Input was not integer");
    
    println!("Следующее за числом {0} число – {1}", number, number + 1);
    println!("Для числа {0} предыдущее число – {1}", number, number - 1);
}
