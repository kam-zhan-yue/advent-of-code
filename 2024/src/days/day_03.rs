use regex::Regex;

pub fn solve(input: &str) {
    let mut p1 = 0i32;
    let mut p2 = 0i32;
    for line in input.lines() {
        p1 += part_one(line);
        p2 += part_one(line);
    }
    println!("Part One is {}", p1);
    println!("Part Two is {}", p2);
}

fn part_one(line: &str) -> i32 {
    let mut sum = 0i32;
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let captures = re.captures_iter(line);
    for capture in captures {
        let left = &capture[1];
        let right = &capture[2];
        let left_error = format!("{left} is not a number");
        let right_error = format!("{right} is not a number");
        let l = left.parse::<i32>().expect(&left_error);
        let r = right.parse::<i32>().expect(&right_error);
        sum += l * r;
    }
    sum
}

fn part_two(line: &str) -> i32 {
    let mut sum = 0i32;
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let enable = Regex::new(r"do\(\)").unwrap();
    let disable = Regex::new(r"don't\(\)").unwrap();
    let captures = re.captures_iter(line);
    for capture in captures {
        let left = &capture[1];
        let right = &capture[2];
        let left_error = format!("{left} is not a number");
        let right_error = format!("{right} is not a number");
        let l = left.parse::<i32>().expect(&left_error);
        let r = right.parse::<i32>().expect(&right_error);
        sum += l * r;
    }
    sum
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
        assert_eq!(part_two("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"), 48);
    }
}


