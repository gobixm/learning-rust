// Известны количество жителей в государстве и площадь
// его территории. Определить плотность населения в этом госу-
// дарстве.

use std::f64::consts::PI;

mod utils;

fn density(citizens: f64, area: f64) -> f64 {
    citizens/area
}

fn main() {
    let citizens = utils::read_float("citizens=");
    let area = utils::read_float("area=");

    println!("density = {0}", density(citizens, area));
}
