// Получить на экране следующее:
// 1
// 2
fn main() {
    let nums = vec![1, 2];
    nums.iter().for_each(|x| println!("{}", x));
}
