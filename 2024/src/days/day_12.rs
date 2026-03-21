use std::collections::{HashMap, HashSet};
use std::ops::Range;
use crate::utils::lib::{Position, DIRECTIONS};

pub fn solve(input: &str) {
    println!("Part One is {}", part_one(input));
    println!("Part Two is {}", part_two(input));
}

fn part_one(input: &str) -> i32 {
    calculate_price(&Garden::from_string(input))
}

fn part_two(_input: &str) -> i32 {
    0
}

struct Garden {
    grid: HashMap<Position, char>,
    rows: Range<i32>,
    cols: Range<i32>,
}

impl Garden {
    pub fn from_string(raw: &str) -> Self {
        let mut grid = HashMap::new();
        for (row, line) in raw.lines().enumerate() {
            for (col, char) in line.chars().enumerate() {
                grid.insert(Position { x: row as i32, y: col as i32 }, char);
            }
        }

        Garden {
            grid,
            rows: 0..raw.lines().count() as i32,
            cols: 0..raw.lines().last().unwrap().chars().count() as i32,
        }
    }
}

#[derive(Clone)]
struct Fence {
    key: char,
    perimeter: i32,
    area: i32,
}

impl Fence {
    pub fn new(key: char) -> Self {
        Fence { key, perimeter: 0, area: 0 }
    }

    pub fn get_price(&self) -> i32 {
        self.perimeter * self.area
    }
}

fn get_perimeter(garden: &Garden, pos: Position) -> i32 {
    let key = garden.grid.get(&pos).unwrap();
    let mut perimeter = 0i32;
    for direction in DIRECTIONS {
        let moved = pos.moved(direction);
        match garden.grid.get(&moved) {
            Some(grid_key) => { if grid_key != key { perimeter += 1; } },
            None => perimeter += 1,
        }
    }
    perimeter
}

fn get_fence(
    garden: &Garden,
    pos: Position,
    visited: &mut HashSet<Position>,
    fence: &Fence
) -> Fence {
    if visited.contains(&pos) { return fence.clone(); }
    visited.insert(pos);

    let mut fence = Fence { 
        key: fence.key,
        perimeter: fence.perimeter + get_perimeter(garden, pos),
        area: fence.area + 1,
    };
    for direction in DIRECTIONS {
        let moved = pos.moved(direction);
        if !garden.rows.contains(&moved.x) || !garden.cols.contains(&moved.y) {
            continue;
        }
        if *garden.grid.get(&moved).unwrap() != fence.key {
            continue;
        }
        fence = get_fence(
            garden,
            moved,
            visited,
            &fence,
        )
    }

    fence
}

fn calculate_price(garden: &Garden) -> i32 {
    let mut visited = HashSet::new();
    let mut price = 0i32;
    for x in garden.rows.clone() {
        for y in garden.cols.clone() {
            let fence = get_fence(
                garden, 
                Position { x, y }, 
                &mut visited, 
                &Fence::new(*garden.grid.get(&Position { x, y }).unwrap())
            );
            price += fence.get_price();
        }
    }
    price
}


#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    pub fn test_part_one() {
        assert_eq!(part_one(INPUT), 1930);
    }

    #[test]
    pub fn test_part_two() {
        assert_eq!(part_two(INPUT), 0);
    }
}
