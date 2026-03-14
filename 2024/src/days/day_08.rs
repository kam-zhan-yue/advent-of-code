use std::collections::{HashSet, HashMap};
use std::ops::Range;
use std::fmt;
use crate::utils::lib::Position;

pub fn solve(input: &str) {
    println!("Part One is {}", part_one(input));
    println!("Part Two is {}", part_two(input));
}

fn part_one(input: &str) -> usize {
    Grid::from_string(input).with_antinodes(false).antinodes.len()
}

fn part_two(input: &str) -> usize {
    Grid::from_string(input).with_antinodes(true).antinodes.len()
}

#[derive(Debug, Clone)]
struct Grid {
    map: HashMap<char, Vec<Position>>,
    nodes: HashSet<Position>,
    antinodes: HashSet<Position>,
    rows: Range<i32>,
    cols: Range<i32>,
}

impl Grid {
    pub fn from_string(raw: &str) -> Self {
        let mut grid = Grid {
            map: HashMap::new(),
            nodes: HashSet::new(),
            antinodes: HashSet::new(),
            rows: 0..raw.lines().count() as i32,
            cols: 0..raw.lines().last().unwrap().chars().count() as i32,
        };

        for (x, line) in raw.lines().enumerate() {
            for (y, c) in line.chars().enumerate() {
                if c != '.' {
                    let position = Position { x: x as i32, y: y as i32 };
                    grid.map.entry(c).and_modify(|nodes| nodes.push(position)).or_insert(vec![position]);
                    grid.nodes.insert(position);
                }
            }
        }

        grid
    }

    pub fn with_antinodes(self: &mut Grid, infinite: bool) -> Grid {
        let mut grid = self.clone();
        for (_, positions) in grid.map.iter() {
            for i in 0..positions.len() - 1 {
                for j in i+1..positions.len() {
                    let dir = positions[j].subtract(positions[i]);

                    let mut multiplier = if infinite { 0 } else { 1 };
                    while let antinode = positions[i].subtract(dir.multiply(multiplier)) 
                        && grid.contains(antinode) {
                        grid.antinodes.insert(antinode);
                        multiplier += 1;
                        if !infinite {
                            break;
                        }
                    }

                    multiplier = if infinite { 0 } else { 1 };
                    while let antinode = positions[j].add(dir.multiply(multiplier)) 
                        && grid.contains(antinode) {
                        grid.antinodes.insert(antinode);
                        multiplier += 1;
                        if !infinite {
                            break;
                        }
                    }
                }
            }
        }
        grid
    }

    fn contains(&self, pos: Position) -> bool {
        self.rows.contains(&pos.x) && self.cols.contains(&pos.y)
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut grid = String::new();
        let mut reverse_map: HashMap<Position, char> = HashMap::new();
        for (key, positions) in self.map.iter() {
            for pos in positions {
                reverse_map.insert(*pos, *key);
            }
        }
        for x in self.rows.clone() {
            for y in self.cols.clone() {
                let pos = Position { x, y };
                if reverse_map.contains_key(&pos) {
                    grid.push(reverse_map[&pos]);
                } else if self.antinodes.contains(&pos) {
                    grid.push('#');
                } else {
                    grid.push('.');
                }
            }
            grid.push('\n');
        }
        write!(f, "{}", grid)
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    pub fn test_part_one() {
        assert_eq!(part_one(INPUT), 14);
    }

    #[test]
    pub fn test_part_two() {
        assert_eq!(part_two(INPUT), 34);
    }
}
