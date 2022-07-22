// Дан радиус окружности. Найти ее диаметр.

mod utils;

fn circle_diam(r: f64) -> f64 {
    2.0 * r    
}

fn main() {
    let a = utils::read_float("radius=");

    println!("diam = {}", circle_diam(a));
}
