// Дан прямоугольник с размерами 543×130 мм. Сколько
// квадратов
// со стороной 130 мм можно отрезать от него?
mod utils;

fn split(side: i64, height: i64) -> i64 {
    if side > height {
        return 0;
    }
    
    height / side
}

fn main() {
    println!("{}", split(130, 543))
}
