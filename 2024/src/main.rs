mod days;
mod utils;

use utils::file::read_input;

fn main() {
    println!("======Day One======");
    days::day_01::solve(&read_input("01"));
    println!("\n======Day Two======");
    days::day_02::solve(&read_input("02"));
    println!("\n======Day Three======");
    days::day_03::solve(&read_input("03"));
}
