use std::collections::HashMap;

pub fn solve(input: &str) {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for line in input.lines() {
        let line_error = format!("{line} is not right");
        let (l, r) = line.split_once("   ").expect(&line_error);
        let l_error = format!("{l} is not right");
        let r_error = format!("{r} is not right");
        left.push(l.parse::<i32>().expect(&l_error));
        right.push(r.parse::<i32>().expect(&r_error));
    }
    left.sort();
    right.sort();
    println!("Part One: {}", part_one(&left, &right));
    println!("Part Two: {}", part_two(&left, &right));
}

fn part_one(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    assert!(left.len() == right.len());
    let mut difference = 0i32;
    for i in 0..left.len() {
        difference += i32::abs(left[i] - right[i]);
    }
    difference
}

fn part_two(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    // create a map from the right 
    let mut map: HashMap<i32, i32> = HashMap::new();
    for i in 0..right.len() {
        map.entry(right[i]).and_modify(|entry| *entry += 1).or_insert(1);
    }

    let mut similarity = 0i32;
    for i in 0..left.len() {
        match map.get(&left[i]) {
            Some(val) => similarity += left[i] * val,
            None => (),
        };
    }
    similarity
}

#[cfg(test)]
mod tests {
    use crate::days::day_01::part_one;
    use crate::days::day_01::part_two;

    #[test]
    fn test_part_one() {
        let mut left: Vec<i32> = [ 3, 4, 2, 1, 3, 3 ].to_vec();
        let mut right: Vec<i32> = [ 4, 3, 5, 3, 9, 3 ].to_vec();
        left.sort();
        right.sort();
        assert_eq!(part_one(left, right), 11);
    }

    #[test]
    fn test_part_two() {
        let mut left: Vec<i32> = [ 3, 4, 2, 1, 3, 3 ].to_vec();
        let mut right: Vec<i32> = [ 4, 3, 5, 3, 9, 3 ].to_vec();
        left.sort();
        right.sort();
        assert_eq!(part_two(left, right), 31);
    }
}
