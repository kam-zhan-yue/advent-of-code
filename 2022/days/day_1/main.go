package day1

import (
	"slices"
	"sort"
	"kamzhanyue/aoc/lib"
	"strconv"
)

func Solve(input string) {
	lib.PrintDay(1)
	lib.PrintPartOne(partOne(input))
	lib.PrintPartTwo(partTwo(input))
}

func partOne(input string) int {
	return slices.Max(getElves(input))
}

func partTwo(input string) int {
	elves := getElves(input)
	sort.Sort(sort.Reverse(sort.IntSlice(elves)))
	return elves[0] + elves[1] + elves[2]
}

func getElves(input string) []int {
	lines := lib.SplitLines(input)
	elves := make([]int, 0)
	current := 0
	for i := range lines {
		if (lines[i] == "") {
			elves = append(elves, current)
			current = 0
		} else {
			val, _ := strconv.Atoi(lines[i])
			current += val
		}
	}
	elves = append(elves, current)
	return elves
}
