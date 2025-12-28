pub fn solve(input: &str) {
    let mut lines: Vec<Vec<i32>> = Vec::new();
    for line in input.lines() {
        let values: Vec<i32> = line.split(" ").map(|v| {
            let error = format!("{v} is not a number");
            v.parse::<i32>().expect(&error)
        }).collect();
        lines.push(values);
    }
    println!("Part One: {}", part_one(&lines));
    println!("Part Two: {}", part_two(&lines));
}

fn part_one(lines: &[Vec<i32>]) -> i32 {
    lines.iter().filter(|line| {
        check_line(line)
    }).collect::<Vec<_>>().len() as i32
}

fn part_two(lines: &[Vec<i32>]) -> i32 {
    lines.iter().filter(|line| {
        check_line_with_remove(line)
    }).collect::<Vec<_>>().len() as i32
}

fn check_line(line: &[i32]) -> bool {
    assert!(line.len() >= 2);
    let ascending: bool = line[1] - line[0] > 0;
    let difference = (line[1] - line[0]).abs();
    if difference > 3 || difference == 0 {
        return false;
    }
    for i in 2..line.len() {
        let is_ascending: bool = line[i] - line[i-1] > 0;
        if is_ascending != ascending {
            return false;
        }
        let difference = (line[i] - line[i-1]).abs();
        if difference > 3 || difference == 0 {
            return false;
        }
    }
    true
}

fn check_line_with_remove(line: &[i32]) -> bool {
    if check_line(line) {
        return true;
    }
    for i in 0..line.len() {
        let mut copy = line.to_owned();
        copy.remove(i);
        if check_line(&copy) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert!(check_line([7, 6, 4, 2, 1].as_ref()));
        assert!(!check_line([1, 2, 7, 8, 9].as_ref()));
        assert!(!check_line([9, 7, 6, 7, 2].as_ref()));
        assert!(!check_line([1, 3, 2, 4, 5].as_ref()));
        assert!(!check_line([8, 6, 4, 4, 1].as_ref()));
        assert!(check_line([1, 3, 6, 7, 9].as_ref()));
    }

    #[test]
    fn test_part_two() {
        assert!(check_line_with_remove([7, 6, 4, 2, 1].as_ref()));
        assert!(!check_line_with_remove([1, 2, 7, 8, 9].as_ref()));
        assert!(!check_line_with_remove([9, 7, 6, 7, 2].as_ref()));
        assert!(check_line_with_remove([1, 3, 2, 4, 5].as_ref()));
        assert!(check_line_with_remove([8, 6, 4, 4, 1].as_ref()));
        assert!(check_line_with_remove([1, 3, 6, 7, 9].as_ref()));
    }
}
