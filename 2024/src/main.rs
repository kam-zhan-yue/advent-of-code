mod days;
mod utils;

use utils::file::read_input;

fn main() {
    println!("======Day One======");
    days::day_01::solve(&read_input("01"));
}
