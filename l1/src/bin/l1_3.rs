// Вывести на экран числа 50 и 10 одно под другим.
fn main() {
    let nums = vec![50, 10];
    nums.iter().for_each(|x| println!("{}", x));
}
