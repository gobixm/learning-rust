use std::io;
use std::io::BufRead;

// Составить программу вывода на экран следующей ин-
// формации:
// а) 2 кг б) а 1 в) x y
// 13 17      19 b   5 y
// Примечание
// a, b, x и y – переменные величины целого типа, значения
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
    let a = read_var("a");
    let b = read_var("b");
    let x = read_var("x");
    let y = read_var("y");

    println!("2 кг 13 17");
    println!("{0} 1 19 {1}", a, b);
    println!("{0} {1}", x, y);
    println!("{0} {1} 5 {1}", x, y);
}
