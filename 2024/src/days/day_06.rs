use std::collections::HashSet;

pub fn solve(input: &str) {
    println!("Part One is {}", part_one(input));
    // println!("Part Two is {}", part_two(&grid));
}

enum Direction { Left, Right, Up, Down }

struct Grid<'a> {
    cells: Vec<&'a [u8]>,
    rows: i32,
    cols: i32,
    visited: HashSet<(i32, i32)>,
    direction: Direction,
    position: (i32, i32),
}

fn get_start_pos(cells: &[&[u8]], rows: usize, cols: usize) -> (i32, i32) {
    for (i, _) in cells.iter().enumerate().take(rows) {
        for (j, _) in cells.iter().enumerate().take(cols) {
            if cells[i][j] == b'^' {
                return (i as i32, j as i32)
            }
        }
    }

    panic!("The grid has no start position");
}

fn parse(input: &str) -> Grid<'_> {
    let cells: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();
    let rows = cells.len();
    let cols = cells[0].len();
    let position = get_start_pos(&cells, rows, cols);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((position.0, position.1));

    Grid {
        cells, 
        rows: rows as i32,
        cols: cols as i32, 
        visited, 
        direction: Direction::Up,
        position,
    }
}

fn in_bounds(grid: &Grid, position: (i32, i32)) -> bool {
    let row_in_bounds = position.0 >= 0 && position.0 < grid.rows;
    let col_in_bounds = position.1 >= 0 && position.1 < grid.cols;
    row_in_bounds && col_in_bounds
}

fn set_position(grid: &mut Grid, pos: (i32, i32)) {
    grid.position = pos;
    grid.visited.insert(pos);
}

fn get_next_position(grid: &mut Grid) -> (i32, i32) {
    match grid.direction {
        Direction::Up => (grid.position.0 - 1, grid.position.1),
        Direction::Down => (grid.position.0 + 1, grid.position.1),
        Direction::Left => (grid.position.0, grid.position.1 - 1),
        Direction::Right => (grid.position.0, grid.position.1 + 1),
    }
}

fn turn_guard(grid: &mut Grid) {
    match grid.direction {
        Direction::Up => grid.direction = Direction::Right,
        Direction::Right => grid.direction = Direction::Down,
        Direction::Down => grid.direction = Direction::Left,
        Direction::Left => grid.direction = Direction::Up,
    }
}

fn move_guard(grid: &mut Grid) -> bool {
    let pos = get_next_position(grid);
    if !in_bounds(grid, pos) {
        return false;
    }
    let cell = grid.cells[pos.0 as usize][pos.1 as usize];
    if cell == b'#' {
        turn_guard(grid);
        return true;
    }
    set_position(grid, pos);
    true
}

fn part_one(input: &str) -> i32 {
    let mut grid = parse(input);

    const MAX_LOOPS: i32 = 10000000;
    let mut loops = 0i32;

    while move_guard(&mut grid) {
        loops += 1;
        if loops > MAX_LOOPS {
            break;
        }
    }

    grid.visited.len().try_into().unwrap()
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
}
