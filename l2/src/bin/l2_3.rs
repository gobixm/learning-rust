// Составить программу:
// а) вычисления значения функции при любом
// значении а;

mod utils;

fn func_a(a: f64) -> f64 {
    (2.0 * a + (3.0 * a).abs().sin() / 3.56).sqrt()
}


fn main() {
    let a = utils::read_float("a=");

    println!("{}", func_a(a));
}
