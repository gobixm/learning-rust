// С некоторого момента прошло 234 дня. Сколько полных
// недель прошло за этот период?
mod utils;

fn full_weeks(days: f64) -> f64 {
    (days / 7.0).trunc()
}

fn main() {
    let days = utils::read_float("days=");
    
    println!("full weeks= {}", full_weeks(days))
}
