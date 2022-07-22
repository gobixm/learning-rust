// Дана длина ребра куба. Найти объем куба и площадь его
// боковой поверхности.

mod utils;

fn volume(side: f64) -> f64 {
    side.powi(3)
}

fn area(side: f64) -> f64 {
    side.powi(2) * 6.0
}

fn main() {
    let side = utils::read_float("side=");
    
    println!("area = {0}, volume = {1}", area(side), volume(side));
}
