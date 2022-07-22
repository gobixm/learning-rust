// Написать программу, в которой рассчитывается сумма
// цифр двузначного числа, вводимого с клавиатуры.
mod utils;

fn main() {
    let num = utils::read_int("2 digits=");
    let dec = num / 10;
    let sing = num % 10;

    println!("sum of digits: {}", dec + sing);
}
