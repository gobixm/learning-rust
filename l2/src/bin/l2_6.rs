// Считая, что Земля – идеальная сфера с радиусом R ≈
// 6350 км, определить расстояние до линии горизонта от точки
// с заданной высотой над Землей.
// D = [(R+h)2 - R2]1/2

mod utils;

fn horizon(h: f64, r: f64) -> f64 {
    ((r + h).powi(2) - r.powi(2)).sqrt()
}

fn main() {
    let h = utils::read_float("h=");
    let r = 6_350_000.0;

    println!("horizon = {}", horizon(h, r));
}
