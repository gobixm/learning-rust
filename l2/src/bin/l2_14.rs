// Даны катеты прямоугольного треугольника. Найти его
// гипотенузу.
use std::f64::consts::PI;

mod utils;

fn hypo(a: f64, b: f64) -> f64 {
    (a.powi(2) + b.powi(2)).sqrt()
}

fn main() {
    let a = utils::read_float("a=");    
    let b = utils::read_float("b=");

    println!("hypo = {0}", hypo(a, b));
}
