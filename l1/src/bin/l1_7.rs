// Число e (основание натурального логарифма) приблизи-
// тельно равно 2,71828. Вывести на экран это число с точностью
// до десятых. Текст '2.7' не использовать.
fn main() {
    let exp = std::f64::consts::E;
    
    println!("{:.1}", exp);
}