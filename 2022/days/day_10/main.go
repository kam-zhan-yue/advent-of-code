package day10

import (
	"strings"
	"fmt"
	"kamzhanyue/aoc/lib"
	"slices"
)


func Solve(input string) {
	lib.PrintDay(10)
	lib.PrintPartOne(partOne(input))
	partTwo(input)
}

func partOne(input string) int {
	checks := []int {20, 60, 100, 140, 180, 220}
	X := 1
	signal := 0
	cycles := parse(input)
	for i, cycle := range cycles {
		if slices.Contains(checks, i + 1) {
			signal += (i + 1) * X
		}
		X += cycle.add
	}
	return signal
}

func partTwo(input string) {
	X := 1
	cycles := parse(input)
	var output strings.Builder
	for i, cycle := range cycles {
		pos := i % 40
		cycleNum := i + 1
		if pos >= X - 1 && pos <= X + 1 {
			output.WriteString("#")
		} else {
			output.WriteString(".")
		}

		if cycleNum % 40 == 0 {
			output.WriteString("\n")
		}
		X += cycle.add
	}
	fmt.Println(output.String())
}

type Cycle struct {
	instruction string
	add int
}

func parse(input string) []Cycle {
	cycles := make([]Cycle, 0)
	for _, line := range lib.SplitLines(input) {
		cycles = append(cycles, parseCycle(line)...)
	}
	return cycles
}

func parseCycle(input string) []Cycle {
	cycles := make([]Cycle, 0)
	splits := lib.SplitWhitespace(input)
	instruction := splits[0]
	switch instruction {
		case "noop": 
		cycles = append(cycles, Cycle { instruction: input, add: 0 })
			return cycles
		default:
			val := lib.ParseInt(splits[1])
			cycles = append(cycles, Cycle { instruction: input, add: 0 })
			cycles = append(cycles, Cycle { instruction: input, add: val })
			return cycles
	}
}

