use std::io;
use std::io::BufRead;

// Составить программу вывода на экран следующей ин-
// формации:
// а) 5 10 б) 100 t в) x 25
// 7 см 1949 v x y
// Примечание
// t, v, x и y – переменные величины целого типа, значения
// которых вводятся с клавиатуры и должны быть выведены
// вместо имен величин.

fn read_var(name: &str) -> u64 {
    println!("{}=", name);

    let mut input: String = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut input).expect("Failed to read from stdin");
    input.trim().parse::<u64>().expect("Input was not integer")
}

fn main() {
    let t = read_var("t");
    let v = read_var("v");
    let x = read_var("x");
    let y = read_var("y");

    println!("5 10");
    println!("100 {}", t);
    println!("{} 25", x);
    println!("7 см 1949 {0} {1} {2}", v, x, y);
}
