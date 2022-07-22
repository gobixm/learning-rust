// Вывести на одной строке числа 47, 52 и 150 с двумя про-
// белами между ними. Текст '47 52 150' не использовать.
fn main() {
    let nums = vec![47, 52, 150];
    let strings: Vec<String> = nums.iter().map(|x| x.to_string()).collect();
    let result = strings.join("  ");

    println!("{}", result);
}
