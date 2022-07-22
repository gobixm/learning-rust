// Дано двузначное число. Получить число, образованное
// при перестановке цифр заданного числа.
mod utils;

fn main() {
    let num = utils::read_int("2 digits=");
    let dec = num / 10;
    let sing = num % 10;

    println!("reverse: {}", sing * 10 + dec);
}
