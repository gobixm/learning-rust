// Дана сторона квадрата. Найти его периметр.

mod utils;

fn sqare_perimetr(side: f64) -> f64 {
    4.0 * side    
}


fn main() {
    let a = utils::read_float("square size=");

    println!("{}", sqare_perimetr(a));
}
