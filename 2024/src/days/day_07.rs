pub fn solve(input: &str) {
    println!("Part One is {}", part_one(input));
    println!("Part Two is {}", part_two(input));
}

fn part_one(input: &str) -> i128 {
    let mut sum = 0;
    for line in input.lines() {
        let problem = Problem::from_string(line);
        if is_solvable_part_one(&problem) {
            sum += problem.result;
        }
    }
    sum
}

fn part_two(input: &str) -> i128 {
    let mut sum = 0;
    for line in input.lines() {
        let problem = Problem::from_string(line);
        if is_solvable_part_two(&problem) {
            sum += problem.result;
        }
    }
    sum
}

#[derive(Debug)]
struct Problem {
    result: i128,
    values: Vec<i128>
}

impl Problem {
    pub fn from_string(raw: &str) -> Problem {
        let line_error = format!("{raw} is not right");
        let (l, r) = raw.split_once(":").expect(&line_error);
        Problem {
            result: l.parse().unwrap(),
            values: r.trim().split(" ").map(|val| val.parse().unwrap()).collect(),
        }
    }
}

fn is_solvable_part_one(problem: &Problem) -> bool {
    is_solvable_add_multiply(problem, problem.values[0], 1)
}

fn is_solvable_add_multiply(problem: &Problem, current: i128, next: usize) -> bool {
    if current == problem.result {
        return true;
    }

    if next >= problem.values.len() {
        return false;
    }

    is_solvable_add_multiply(problem, current + problem.values[next], next + 1)
    || is_solvable_add_multiply(problem, current * problem.values[next], next + 1)
}

fn is_solvable_part_two(problem: &Problem) -> bool {
    is_solvable_add_multiply_concat(problem, problem.values[0], 1)
}

fn is_solvable_add_multiply_concat(problem: &Problem, current: i128, next: usize) -> bool {
    if current > problem.result {
        return false;
    }

    if next == problem.values.len() && current == problem.result {
        return true;
    }

    if next >= problem.values.len() {
        return false;
    }

    let addition = current + problem.values[next];
    let multiplication = current * problem.values[next];
    let concatenation = (current.to_string() + &problem.values[next].to_string()).parse().unwrap();
    is_solvable_add_multiply_concat(problem, addition, next + 1)
    || is_solvable_add_multiply_concat(problem, multiplication, next + 1)
    || is_solvable_add_multiply_concat(problem, concatenation, next + 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    pub fn test_part_one() {
        assert_eq!(part_one(INPUT), 3749);
    }

    #[test]
    pub fn test_part_two() {
        assert_eq!(part_two(INPUT), 11387);
    }
}
