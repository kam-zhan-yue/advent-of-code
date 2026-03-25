package day6

import (
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
		if completed(window) {
			return window.end
		}
		window = remove(window, window.start)
		window = add(window, window.end)
		window.start += 1
		window.end += 1
	}
	return 0
}

func completed(window Window) bool {
	added := 0
	for i := range window.set {
		if window.set[i] > 0 {
			added += 1
		}
	}
	return added >= 4
}

func remove(window Window, index int) Window {
	window.set[rune(window.input[index])] -= 1
	return window
}

func add(window Window, index int) Window {
	window.set[rune(window.input[index])] += 1
	return window
}
