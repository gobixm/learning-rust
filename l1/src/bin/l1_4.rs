// Вывести на экран числа 5, 10 и 21 одно под другим.
fn main() {
    let nums = vec![5, 10, 21];
    nums.iter().for_each(|x| println!("{}", x));
}
