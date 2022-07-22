// С начала суток прошло n секунд. Определить:
// а) сколько полных часов прошло с начала суток;
// б) сколько полных минут прошло с начала очередного часа;
// в) сколько полных секунд прошло с начала очередной минуты.
mod utils;

fn main() {
    let seconds = utils::read_int("seconds=");
    let hours = seconds / 3_600;
    let minutes = (seconds % 3_600) / 60;
    let sec = seconds % 60;
    
    println!(" hours: {0}, minutes: {1}, seconds: {2}", hours, minutes, sec);
}
