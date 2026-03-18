use crate::utils::lib::{Position, Direction};
use std::collections::HashSet;
use std::ops::Range;

pub fn solve(input: &str) {
    println!("Part One is {}", part_one(input));
    println!("Part Two is {}", part_two(input));
}

fn part_one(input: &str) -> i32 {
    get_score(&Map::from_string(input))
}

fn part_two(_input: &str) -> i32 {
    0
}

#[derive(Debug, Clone)]
struct Map {
    grid: Vec<Vec<i32>>,
    trailheads: Vec<Position>,
    rows: Range<i32>,
    cols: Range<i32>,
}

impl Map {
    pub fn from_string(raw: &str) -> Self {
        let mut grid: Vec<Vec<i32>> = Vec::new();
        let mut trailheads: Vec<Position> = Vec::new();
        for (row, line) in raw.lines().enumerate() {
            let mut entry: Vec<i32> = Vec::new();
            for (col, c) in line.chars().enumerate() {
                if c == '0' {
                    trailheads.push(Position { x: row as i32, y: col as i32 });
                }
                entry.push(c.to_digit(10).unwrap().try_into().unwrap());
            }
            grid.push(entry);
        }
        Map { 
            grid, 
            trailheads, 
            rows: 0..raw.lines().count() as i32,
            cols: 0..raw.lines().last().unwrap().chars().count() as i32,
        }
    }
}

fn get_score(map: &Map) -> i32 {
    let mut score = 0i32;
    for trailhead in map.trailheads.clone().into_iter() {
        let mut visited: HashSet<Position> = HashSet::new();
        score += get_trailhead_score(map, trailhead, 0, &mut visited);
    }
    score
}

fn get_trailhead_score(map: &Map, pos: Position, val: i32, visited: &mut HashSet<Position>) -> i32 {
    if !map.rows.contains(&pos.x) || !map.cols.contains(&pos.y) {
        return 0;
    }
    let current = map.grid[pos.x as usize][pos.y as usize];
    if current != val {
        return 0;
    }
    if current == 9 && visited.contains(&pos) {
        return 0;
    }
    if current == 9 && !visited.contains(&pos) {
        visited.insert(pos);
        return 1;
    }
    let up = pos.moved(Direction::Up);
    let down = pos.moved(Direction::Down);
    let left = pos.moved(Direction::Left);
    let right = pos.moved(Direction::Right);
    let next = val + 1;

    get_trailhead_score(map, up, next, visited) + 
    get_trailhead_score(map, down, next, visited) + 
    get_trailhead_score(map, left, next, visited) + 
    get_trailhead_score(map, right, next, visited)
}


#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    pub fn test_part_one() {
        assert_eq!(part_one(INPUT), 36);
    }

    #[test]
    pub fn test_part_two() {
        assert_eq!(part_two(INPUT), 0);
    }
}
