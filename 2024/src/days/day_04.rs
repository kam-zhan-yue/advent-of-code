pub fn solve(input: &str) {
    println!("Part One is {}", part_one(input));
    println!("Part Two is {}", part_two(input));
}

struct Cell {
    row: i32,
    col: i32,
}

struct Grid<'a> {
    lines: Vec<&'a str>,
    rows: i32,
    cols: i32,
}

fn test_cell(grid: &Grid, cell: &Cell, target: char) -> bool {
    if cell.row < 0 || cell.row >= grid.rows {
        return false;
    }
    if cell.col < 0 || cell.col >= grid.cols {
        return false;
    }
    let line = grid.lines[cell.row as usize].as_bytes();
    line[cell.col as usize] == target as u8
}

fn check_word(grid: &Grid, cell: &Cell, word: &str, vertical: i32, horizontal: i32) -> i32 {
    let chars: Vec<char> = word.chars().collect();
    for (i, target) in chars.iter().enumerate() {
        let next_cell = Cell{ 
            row: cell.row + vertical * (i as i32), 
            col: cell.col + horizontal * (i as i32), 
        };
        let result = test_cell(grid, &next_cell, *target);
        if !result {
            return 0
        }
    }
    1
}

fn check_xmas(grid: &Grid, cell: &Cell) -> i32 {
    let mut sum = 0;
    sum += check_word(grid, cell, "XMAS", 0, 1);
    sum += check_word(grid, cell, "XMAS", 0, -1);
    sum += check_word(grid, cell, "XMAS", 1, 0);
    sum += check_word(grid, cell, "XMAS", -1, 0);
    sum += check_word(grid, cell, "XMAS", 1, 1);
    sum += check_word(grid, cell, "XMAS", 1, -1);
    sum += check_word(grid, cell, "XMAS", -1, 1);
    sum += check_word(grid, cell, "XMAS", -1, -1);
    sum
}

fn part_one(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    let rows = lines.len() as i32;
    let cols = lines[0].len() as i32;
    let grid = Grid { lines, rows, cols };
    let mut sum = 0;
    for i in 0..rows {
        for j in 0..cols {
            let cell = Cell{ row: i, col: j };
            sum += check_xmas(&grid, &cell);
        }
    }
    sum
}

fn check_x_mas(grid: &Grid, cell: &Cell) -> i32 {
    let top_left = cell;
    let bottom_left = Cell { row: cell.row + 2, col: cell.col };
    let check_top_left = check_word(grid, top_left, "MAS", 1, 1) + check_word(grid, top_left, "SAM", 1, 1);
    let check_bottom_left = check_word(grid, &bottom_left, "MAS", -1, 1) + check_word(grid, &bottom_left, "SAM", -1, 1);
    if check_top_left > 0 && check_bottom_left > 0 {
        return 1;
    }
    0
}

fn part_two(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    let rows = lines.len() as i32;
    let cols = lines[0].len() as i32;
    let grid = Grid { lines, rows, cols };
    let mut sum = 0;
    for i in 0..rows {
        for j in 0..cols {
            let cell = Cell{ row: i, col: j };
            sum += check_x_mas(&grid, &cell);
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), 18);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), 9);
    }
}
