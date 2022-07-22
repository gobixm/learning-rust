// Написать программу, которая решает следующую задачу:
// «N школьников делят k яблок поровну так, чтобы каждому до-
// стались только целые яблоки, остальные яблоки остаются в кор-
// зинке. Определить, сколько яблок достанется каждому школьнику
// и сколько яблок останется в корзинке».
mod utils;

struct DivideResult {
    each: i64,
    remainder: i64
}

fn divide(kids: i64, apples: i64) -> DivideResult {
    DivideResult {
        each: apples/kids,
        remainder: apples % kids,
    }
}

fn main() {
    let kids = utils::read_int("kids=");
    let apples = utils::read_int("apples=");
    let result = divide(kids, apples);
    
    println!("result {0} each, remainder = {1}", result.each, result.remainder)
}
