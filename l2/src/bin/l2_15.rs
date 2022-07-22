// Найти площадь кольца по заданным внешнему и внутреннему
// радиусам.
use std::f64::consts::PI;

mod utils;

fn area(r: f64) -> f64 {
    PI * r.powi(2)
}

fn main() {
    let r_ext = utils::read_float("external_radius=");
    let r_int = utils::read_float("internal_radius=");
    
    assert!(r_ext >= r_int, "external radius is less than internal");

    println!("area = {0}", area(r_ext) - area(r_int));
}
