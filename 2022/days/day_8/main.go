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
	result := 0
	for row := range forest.rows {
		for col := range forest.cols {
			pos := Position { X: row, Y: col }
			result = max(getScore(forest, pos), result)
		}
	}
	return result
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

func look(forest Forest, height int, pos Position, dir Direction) int {
	// We are out of bounds
	if pos.X < 0 || pos.Y < 0 || pos.X >= forest.rows || pos.Y >= forest.cols {
		return 0
	}
	// We hit a tree that is taller or same height
	if height <= forest.trees[pos] {
		return 1
	}
	return 1 + look(forest, height, lib.Move(pos, dir), dir)
}

func getScore(forest Forest, pos Position) int {
	return look(forest, forest.trees[pos], lib.Move(pos, lib.Up), lib.Up) *
				 look(forest, forest.trees[pos], lib.Move(pos, lib.Down), lib.Down) *
				 look(forest, forest.trees[pos], lib.Move(pos, lib.Left), lib.Left) *
				 look(forest, forest.trees[pos], lib.Move(pos, lib.Right), lib.Right)
}
