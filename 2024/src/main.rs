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
    println!("\n======Day Four======");
    days::day_04::solve(&read_input("04"));
    println!("\n======Day Five======");
    days::day_05::solve(&read_input("05"));
    println!("\n======Day Six======");
    days::day_06::solve(&read_input("06"));
    println!("\n======Day Seven======");
    days::day_07::solve(&read_input("07"));
    println!("\n======Day Eight======");
    days::day_08::solve(&read_input("08"));
    println!("\n======Day Nine======");
    days::day_09::solve(&read_input("09"));
}
