// Составить программу вычисления значения функции
// при любом значении а.

mod utils;

fn func(x: f64) -> f64 {
    (x*x + 10.0) / (x*x + 1.0).sqrt()
}



fn main() {
    let x = utils::read_float("a=");
    
    println!("{}", func(x));
}
