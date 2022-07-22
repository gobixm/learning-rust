// Известны объем и масса тела. Определить плотность ма-
// териала этого тела.

use std::f64::consts::PI;

mod utils;

fn density(volume: f64, mass: f64) -> f64 {
    volume/mass
}

fn main() {
    let volume = utils::read_float("volume=");
    let mass = utils::read_float("mass=");

    println!("density = {0}", density(volume, mass));
}
