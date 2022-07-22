use std::io::BufRead;

pub fn read_int(query: &str) -> i64 {
    println!("{}", query);
    let mut input = String::new();
    std::io::stdin().lock().read_line(&mut input).expect("Failed to read from stdin");

    input.trim().parse::<i64>().expect("Failed to parse integer")
}

pub fn read_float(query: &str) -> f64 {
    println!("{}", query);
    let mut input = String::new();
    std::io::stdin().lock().read_line(&mut input).expect("Failed to read from stdin");

    input.trim().parse::<f64>().expect("Failed to parse integer")
}