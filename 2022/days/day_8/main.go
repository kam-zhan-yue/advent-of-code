package day8

import (
	"fmt"
	"kamzhanyue/aoc/lib"
)

func Solve(input string) {
	lib.PrintDay(8)
	lib.PrintPartOne(partOne(input))
	lib.PrintPartTwo(partTwo(input))
}

func partOne(input string) int {
	forest := parse(input)
	visible := getVisible(forest)

	result := 0
	for row := range forest.rows {
		for col := range forest.cols {
			pos := Position { X: row, Y: col }
			if isVisible(forest, pos, visible) {
				result += 1 
			}
		}
	}
	return result
}

func partTwo(input string) int {
	forest := parse(input)
	visible := getVisible(forest)

	return getScore(forest, Position { X: 3, Y: 2 }, visible)
	// result := 0
	// for row := range forest.rows {
	// 	for col := range forest.cols {
	// 		pos := Position { X: row, Y: col }
	// 		result = max(getScore(forest, pos, visible), result)
	// 	}
	// }
	// return result
}

type Direction = lib.Direction
type Position = lib.Position
type Grid = map[Position]int
type Forest struct {
	trees Grid
	rows int
	cols int
}
type Visible struct {
	up Grid
	down Grid
	left Grid
	right Grid
}

type Vision = map[int]Grid
type FOV struct {
	up Vision
	down Vision
	left Vision
	right Vision
}

func parse(input string) Forest {
	grid := make(Grid)
	lines := lib.SplitLines(input)
	for x, line := range lines {
		for y, char := range line {
			pos := Position { X: x, Y: y}
			grid[pos] = lib.ParseInt(string(char))
		}
	}
	return Forest {
		trees: grid,
		rows: len(lines),
		cols: len(lines[0]),
	}
}

func getVisible(forest Forest) Visible {
	up := make(Grid)
	down := make(Grid)
	left := make(Grid)
	right := make(Grid)

	// Make up, start from the top
	for col := range forest.cols {
		height := 0
		for row := range forest.rows {
			height = getHeight(forest, Position { X: row, Y: col }, lib.Up, up, height)
		}
	}

	// Make down, start from the bottom
	for col := range forest.cols {
		height := 0
		for row := forest.rows - 1; row >= 0; row-- {
			height = getHeight(forest, Position { X: row, Y: col }, lib.Down, down, height)
		}
	}

	// Make left, start from the left
	for row := range forest.rows {
		height := 0
		for col := range forest.cols {
			height = getHeight(forest, Position { X: row, Y: col }, lib.Left, left, height)
		}
	}

	// Make right, start from the right
	for row := range forest.rows {
		height := 0
		for col := forest.cols - 1; col >= 0; col-- {
			height = getHeight(forest, Position { X: row, Y: col }, lib.Right, right, height)
		}
	}
	return Visible { up, down, left, right}
}

func getHeight(forest Forest, pos Position, dir Direction, grid Grid, height int) int {
	moved := lib.Move(pos, dir)
	height = max(forest.trees[moved], height)
	grid[pos] = height
	return height
}

func printGrid(m Grid, rows int, cols int) {
	for row := range rows {
		for col := range cols {
			fmt.Printf("%d ", m[Position { X: row, Y: col }])
		}
		fmt.Println()
	}
}

func isVisible(forest Forest, pos Position, visible Visible) bool {
	height := forest.trees[pos]
	// If at the edge, return true
	if pos.X == 0 || pos.Y == 0 || pos.X == forest.rows - 1 || pos.Y == forest.cols - 1 {
		return true
	}
	return height > visible.up[pos] ||
		height > visible.down[pos] ||
		height > visible.left[pos] || 
		height > visible.right[pos]
}

func getScore(forest Forest, pos Position, visible Visible) int {
	// How many tres can it see on the left?
	// height := forest.trees[pos]
	// fmt.Println("TREES")
	// printGrid(forest.trees, forest.rows, forest.cols)
	// fmt.Println("LEFT")
	// printGrid(visible.left, forest.rows, forest.cols)
	return 0
}
