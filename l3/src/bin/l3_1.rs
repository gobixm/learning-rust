// Дано расстояние в сантиметрах. Найти число полных метров
// в нем.
mod utils;

fn meters(cm: f64) -> f64 {
    (cm / 100.0).trunc()
}

fn main() {
    let cm = utils::read_float("cm=");
    
    println!("meters= {}", meters(cm))
}
