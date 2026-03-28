package lib

import (
	"strings"
	"strconv"
	"math"
)

func SplitLines(raw string) []string {
	return strings.Split(raw, "\n")
}

func SplitWhitespace(raw string) [] string {
	return strings.Split(raw, " ")
}

func ParseInt(s string) int {
	val, _ := strconv.Atoi(s)
	return val
}

func ParseDirection(s string) Direction {
	switch s {
		case "R": return Right
		case "L": return Left
		case "U": return Up
		default: return Down
	}
}

func Abs(i int) int {
	return int(math.Abs(float64(i)))
}

func Clamp(val int, min int, max int) int {
	if val < min {
		return min
	}
	if val > max {
		return max
	}
	return val
}

type Position struct {
	X int
	Y int
}

type Direction int

const (
	Up Direction = iota
	Down
	Left
	Right
)

func Move(pos Position, dir Direction) Position {
	switch dir {
		case Up: return Position { pos.X - 1, pos.Y }
		case Down: return Position { pos.X + 1, pos.Y }
		case Left: return Position { pos.X, pos.Y - 1 }
		default: return Position { pos.X, pos.Y + 1 }
	}
}

func Add(a Position, b Position) Position {
	return Position { a.X + b.X, a.Y + b.Y }
}

func Subtract(a Position, b Position) Position {
	return Position { a.X - b.X, a.Y - b.Y }
}

func Length(a Position) float64 {
	return math.Sqrt(float64(a.X * a.X + a.Y * a.Y))
}
