// В купейном вагоне имеется 9 купе с четырьмя местами
// для пассажиров в каждом. Определить номер купе, в котором на-
// ходится место с заданным номером (нумерация мест сквозная,
// начинается с 1).
mod utils;

fn get_carriage(place: i64, carriages: i64) -> i64 {
    let index = place - 1;
    if index < 0 { return -1; };
    if index / 4 > carriages - 1 { return -1; }
    index / 4 + 1
}

fn main() {
    let n = utils::read_int("n=");
    println!("{}", get_carriage(n, 9))
}
