// Даны два целых числа a и b. Если a делится на b или b
// делится на a, то вывести 1, иначе – любое другое число. Условные
// операторы и операторы цикла не использовать.

mod utils;

fn main() {
    let a = utils::read_int("a=");
    let b = utils::read_int("b=");

    let ab = a % b;
    let ba = b % a;

    println!("res: {0}", ab * ba + 1);
}
