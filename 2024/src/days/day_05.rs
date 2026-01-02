use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) {
    let problem = Problem::parse(input);
    println!("Part One is {}", part_one(&problem));
}

#[derive(Debug)]
struct Problem {
    map: HashMap<i32, HashSet<i32>>,
    lines: Vec<Vec<i32>>,
}

impl Problem {
    fn parse(input: &str) -> Self {
        let mut parsing_map = true;
        let mut problem = Problem {
            map: HashMap::new(),
            lines : Vec::new(),
        };
        for line in input.lines() {
            if line.is_empty() {
                parsing_map = false;
                continue;
            }
            if parsing_map {
                let (l, r) = line.split_once('|').unwrap();
                let left: i32 = l.parse().unwrap();
                let right: i32 = r.parse().unwrap();

                problem.map.entry(left).and_modify(|entry| {
                    entry.insert(right);
                }).or_insert({
                    let mut set = HashSet::new();
                    set.insert(right);
                    set
                });
            } else {
                problem.lines.push(line.split(",").map(|val| val.parse().unwrap()).collect());
            }
        }
        problem
    }
}

fn check_pair(problem: &Problem, left: &i32, right: &i32) -> bool {
    if let Some(after) = problem.map.get(left) {
        return after.contains(right);
    }
    false
}

fn check_line(problem: &Problem, line: &[i32]) -> bool {
    for i in 0..line.len() - 1 {
        for j in i+1..line.len() {
            if !check_pair(problem, &line[i], &line[j]) {
                return false;
            }
        }
    }
    true
}

fn part_one(problem: &Problem) -> i32 {
    let mut sum = 0;
    for line in &problem.lines {
        if check_line(problem, line) {
            sum += line[line.len()/2];
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

const INPUT: &str =
"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&Problem::parse(INPUT)), 143);
    }
}
