package day6

import (
	"fmt"
	"kamzhanyue/aoc/lib"
)

func Solve(input string) {
	lib.PrintDay(5)
	lib.PrintPartOne(partOne(input))
	lib.PrintPartTwo(partTwo(input))
}

func partOne(input string) int {
	window := parse(input, 0, 4)
	return search(window)
}

func partTwo(input string) int {
	return 0
}

type Set = map[rune]int

type Window struct {
	set Set
	input string
	start int
	end int
}

func parse(input string, start int, end int) Window {
	set := make(Set)
	for i := start; i < end; i++ {
		set[rune(input[i])] += 1;
	}
	return Window { set, input, start, end }
}

func search(window Window) int {
	for i := 4; i < len(window.input); i++ {

	}
	fmt.Println(window)
	return 0
}
