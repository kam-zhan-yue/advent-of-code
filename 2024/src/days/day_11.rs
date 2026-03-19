pub fn solve(input: &str) {
    println!("Part One is {}", part_one(input));
    println!("Part Two is {}", part_two(input));
}

fn part_one(_input: &str) -> i32 {
    0
}

fn part_two(_input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "125 17";

    #[test]
    pub fn test_part_one() {
        assert_eq!(part_one(INPUT), 0);
    }

    #[test]
    pub fn test_part_two() {
        assert_eq!(part_two(INPUT), 0);
    }
}
