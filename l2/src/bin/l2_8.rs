// Дан радиус окружности. Найти длину окружности и пло-
// щадь круга.

use std::f64::consts::PI;

mod utils;

fn length(r: f64) -> f64 {
    2.0 * PI * r
}

fn area(r: f64) -> f64 {
    PI * r.powi(2)
}

fn main() {
    let r = utils::read_float("radius=");

    println!("area = {0}, length = {1}", area(r), length(r));
}
