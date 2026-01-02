pub fn solve(input: &str) {
    println!("Part One is {}", part_one(input));
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
    for i in 0..chars.len() {
        let next_cell = Cell{ 
            row: cell.row + vertical * (i as i32), 
            col: cell.col + horizontal * (i as i32), 
        };
        let result = test_cell(grid, &next_cell, chars[i]);
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
            let cell = Cell{ row: i as i32, col: j as i32 };
            sum += check_xmas(&grid, &cell);
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
}
