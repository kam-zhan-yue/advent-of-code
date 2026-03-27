package lib

import (
	"strings"
	"strconv"
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
