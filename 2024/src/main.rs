mod days;
mod utils;

use utils::file::read_input;

fn main() {
    days::day_01::solve(&read_input("01"));
}
