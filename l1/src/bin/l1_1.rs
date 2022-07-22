// Вывести на одной строке числа 31, 18 и 79 с одним про-
// белом между ними. Текст '31 18 79' не использовать.
fn main() {
    let nums = vec![31, 18, 79];
    let strings: Vec<String> = nums.iter().map(|x| x.to_string()).collect();
    let result = strings.join(" ");

    println!("{}", result);
}
