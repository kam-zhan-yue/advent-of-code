use std::io::{self, Read};

struct Problem {
    rows: i32,
    cols: i32,
    required: i32,
}

impl Problem {
    pub fn new(line: &str) -> Self {
        let splits = line.split_once(": ").unwrap();
        let (rows, cols) = splits.0.split_once('x').unwrap();
        let numbers: Vec<i32> = splits
            .1
            .split_whitespace()
            .map(|val| val.parse::<i32>().unwrap())
            .collect();
        let required = numbers.iter().sum();
        Self {
            rows: rows.parse::<i32>().unwrap(),
            cols: cols.parse::<i32>().unwrap(),
            required,
        }
    }

    pub fn solvable(self) -> bool {
        (self.rows / 3) * (self.cols / 3) >= self.required
    }
}

struct Solution;

impl Solution {
    pub fn solve(input: &str) -> usize {
        input
            .lines()
            .filter(|line| Problem::new(line).solvable())
            .count()
    }
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    println!("Solution is {:?}", Solution::solve(&buffer));
    Ok(())
}
