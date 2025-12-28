pub fn solve(input: &str) {
    let mut x = 0i32;
    for line in input.lines() {
        x += part_one(line);
    }
    println!("Part One is {}", x);
}

fn part_one(line: &str) -> i32 {
    line.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"), 161);
    }

    #[test]
    fn test_part_two() {
    }
}


