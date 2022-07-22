// Составить программу решения линейного уравнения
// ax + b = 0 (a ≠ 0)

use std::f64::consts::PI;

mod utils;

fn solve(a: f64, b: f64) -> f64 {    
    -b/a
}

fn main() {
    let a = utils::read_float("a=");
    
    assert_ne!(a, 0.0, "a is 0");
    
    let b = utils::read_float("b=");

    println!("x = {0}", solve(a, b));
}
