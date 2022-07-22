// Даны два целых числа. Найти:
// а) их среднее арифметическое;
// б) их среднее геометрическое.

use std::f64::consts::PI;

mod utils;

fn mean(a: f64, b: f64) -> f64 {
    (a + b) / 2.0
}

fn geo_mean(a: f64, b: f64) -> f64 {
    (a * b).powf(0.5)
}

fn main() {
    let a = utils::read_float("a=");
    let b = utils::read_float("b=");

    println!("mean = {0}, geo_mean = {1}", mean(a, b), geo_mean(a, b));
}
