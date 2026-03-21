use std::collections::{HashMap, HashSet};
use std::ops::Range;
use crate::utils::lib::{Position, DIRECTIONS};

pub fn solve(input: &str) {
    println!("Part One is {}", part_one(input));
    println!("Part Two is {}", part_two(input));
}

fn part_one(input: &str) -> i32 {
    get_fences(&Garden::from_string(input)).into_iter().map(|fence| fence.get_price()).sum()
}

fn part_two(input: &str) -> i32 {
    get_fences(&Garden::from_string(input)).into_iter().map(|fence| fence.get_corners() * fence.area.len() as i32).sum()
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
    area: HashSet<Position>,
}

impl Fence {
    pub fn new(key: char) -> Self {
        Fence { key, perimeter: 0, area: HashSet::new() }
    }

    pub fn get_price(&self) -> i32 {
        self.perimeter * self.area.len() as i32
    }

    pub fn get_corners(&self) -> i32 {
        let mut corners = 0i32;
        for pos in self.area.clone() {
            // check each position
            for direction in DIRECTIONS {
                // a corner occurs when the area is surrounded by two points
                if self.area.contains(&pos.moved(direction))
                    && self.area.contains(&pos.moved(direction.turn_clockwise()))
                    && !self.area.contains(&pos.moved(direction).moved(direction.turn_clockwise())) {
                    corners += 1;
                }
                // a corner occurs when the area has no points
                if !self.area.contains(&pos.moved(direction))
                    && !self.area.contains(&pos.moved(direction.turn_clockwise())) {
                    corners += 1;
                }
            }

        }
        corners
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

    let mut area = fence.area.clone();
    area.insert(pos);
    let mut fence = Fence { 
        key: fence.key,
        perimeter: fence.perimeter + get_perimeter(garden, pos),
        area,
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

fn get_fences(garden: &Garden) -> Vec<Fence> {
    let mut fences = Vec::new();
    let mut visited = HashSet::new();
    for x in garden.rows.clone() {
        for y in garden.cols.clone() {
            let fence = get_fence(
                garden, 
                Position { x, y }, 
                &mut visited, 
                &Fence::new(*garden.grid.get(&Position { x, y }).unwrap())
            );
            if !fence.area.is_empty() {
                fences.push(fence);
            }
        }
    }
    fences
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
        assert_eq!(part_two(INPUT), 1206);
    }
}
