use std::collections::HashMap;

pub fn solve(input: &str) {
    println!("Part One is {}", part_one(input));
    println!("Part Two is {}", part_two(input));
}

fn part_one(input: &str) -> i128 {
    let stones = input.split_whitespace().map(String::from).collect();
    blink(stones, 25)
}

fn part_two(input: &str) -> i128 {
    let stones = input.split_whitespace().map(String::from).collect();
    blink(stones, 75)
}

pub fn blink(stones: Vec<String>, blinks: usize) -> i128 {
    let mut sum = 0;
    let mut memo: Vec<HashMap<String, i128>> = Vec::new();
    for _ in 0..blinks+1 {
        memo.push(HashMap::new());
    }

    for stone in stones {
        sum += blink_stone(stone, blinks, &mut memo);
    }

    sum
}

fn blink_stone(stone: String, blinks: usize, memo: &mut Vec<HashMap<String, i128>>) -> i128 {
    if blinks == 0 { return 1; }
    if memo[blinks].contains_key(&stone) { return memo[blinks][&stone]; }

    let mut transformed: Vec<String> = Vec::new();
    let val_str;
    if stone == "0" {
        transformed.push("1".to_string());
    } else if stone.len().is_multiple_of(2) {
        transformed.push(stone[0..stone.len()/2].to_string());
        let right: i32 = stone[stone.len()/2..stone.len()].parse().unwrap();
        transformed.push(right.to_string());
    } else {
        let mut val: i128 = stone.clone().parse().unwrap();
        val *= 2024;
        val_str = val.to_string();
        transformed.push(val_str);
    }

    let mut result = 0i128;
    for s in transformed {
        result += blink_stone(s, blinks - 1, memo);
    }

    memo[blinks].insert(stone, result);
    result
}


#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "125 17";

    #[test]
    pub fn test_part_one() {
        assert_eq!(part_one(INPUT), 55312);
    }

    #[test]
    pub fn test_part_two() {
        assert_eq!(part_two(INPUT), 65601038650482);
    }
}
