// Составить программу:
// а) вычисления значения функции y = 17x2 – 6x + 13 при любом
// значении x;
// б) вычисления значения функции y = 3a2 + 5a – 21 при любом
// значении а.

mod utils;

fn func_a(x: f64) -> f64 {
    17f64 * x * x - 6f64 * x + 13f64
}

fn func_b(a: f64) -> f64 {
    3f64 * a * a + 5f64 * a - 21f64
}

fn main() {
    let a = utils::read_float("x=");
    let b = utils::read_float("a=");
    
    println!("{}", func_a(a));
    println!("{}", func_b(b));
}
