pub fn solve(input: &str) {
    println!("Part One is {}", part_one(input));
    println!("Part Two is {}", part_two(input));
}

fn part_one(input: &str) -> i64 {
    let mut sum = 0;
    for line in input.lines() {
        let problem = Problem::from_string(&line);
        if is_solvable(&problem) {
            sum += problem.result;
        }
    }
    sum
}

fn part_two(_input: &str) -> i64 {
    0
}

#[derive(Debug)]
struct Problem {
    result: i64,
    values: Vec<i64>
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

fn is_solvable(problem: &Problem) -> bool {
    return is_solvable_recursive(problem, problem.values[0], 1)
}

fn is_solvable_recursive(problem: &Problem, current: i64, next: usize) -> bool {
    if current == problem.result {
        return true;
    }

    if next >= problem.values.len() {
        return false;
    }

    return is_solvable_recursive(problem, current + problem.values[next], next + 1)
    || is_solvable_recursive(problem, current * problem.values[next], next + 1)
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
}
