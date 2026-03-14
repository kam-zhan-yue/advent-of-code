use std::collections::{HashMap, HashSet};
use std::ops::Range;
use std::iter::once;
use rayon::prelude::*;
use crate::utils::lib::{Position, Direction};

pub fn solve(input: &str) {
    println!("Part One is {}", part_one(input));
    println!("Part Two is {}", part_two(input));
}

fn part_one(input: &str) -> i32 {
    let g = Grid::from_string(input);
    match g {
        Ok(grid) => {
            HashSet::<Position>::from_iter(once(grid.guard.pos)
            .chain(grid.map(|step| step.pos)))
            .len() as i32
        },
        Err(_) => 0
    }
}

fn part_two(input: &str) -> i32 {
    let g = Grid::from_string(input);
    match g {
        Ok(grid) => {
            HashSet::<Position>::from_iter(once(grid.guard.pos)
            .chain(grid.clone().map(|step| step.pos)))
            .par_iter()
            .filter(|obstacle| grid.with_obstacle(**obstacle).would_loop())
            .count() as i32
        },
        Err(_) => 0
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Guard {
    pos: Position,
    dir: Direction,
}

#[derive(Debug, Clone)]
pub struct Grid {
    guard: Guard,
    obstacles: HashSet<Position>,
    collisions: HashSet<(Position, Direction)>,
    x_max: Range<i32>,
    y_max: Range<i32>,
}

impl Grid {
    pub fn from_string(raw: &str) -> Result<Grid, String> {
        let mut map: HashMap<char, HashSet<Position>> = HashMap::new();
        for (x, line) in raw.lines().enumerate() {
            for (y, c) in line.chars().enumerate() {
                map.entry(c).or_default().insert(Position { x: x as i32, y: y as i32 });
            }
        }
        match map.get(&'^') {
            None => Err("Error: No guard!".to_owned()),
            Some(g) => Ok(Grid {
                guard: Guard {
                    pos: *g.iter().next().unwrap(),
                    dir: Direction::Up,
                },
                obstacles: map[&'#'].clone(),
                collisions: HashSet::new(),
                x_max: 0..raw.lines().count() as i32,
                y_max: 0..raw.lines().last().unwrap().chars().count() as i32,
            })
        }
    }

    pub fn with_obstacle(&self, obstacle: Position) -> Self {
        let mut new = self.clone();
        new.obstacles.insert(obstacle);
        new
    }

    pub fn would_loop(&mut self) -> bool {
        if let Some(final_state) = self.last()
            && let Some(next) = final_state.pos.moved(final_state.dir) {
                return self.obstacles.contains(&next);
            }
        false
    }
}

impl Iterator for Grid {
    type Item = Guard;

    fn next(&mut self) -> Option<Self::Item> {
        let next_pos = self.guard.pos.moved(self.guard.dir)?;
        let in_bounds = self.x_max.contains(&next_pos.x) && self.y_max.contains(&next_pos.y);
        if !in_bounds {
            return None;
        }

        if self.obstacles.contains(&next_pos) {
            // Loop detected!
            if !self.collisions.insert((next_pos, self.guard.dir)) {
                return None;
            }
            self.guard.dir = self.guard.dir.turn();
        } else {
            self.guard.pos = next_pos;
        }
        Some(self.guard)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    pub fn test_part_one() {
        assert_eq!(part_one(INPUT), 41)
    }

    #[test]
    pub fn test_part_two() {
        assert_eq!(part_two(INPUT), 6)
    }
}
