package day4

import (
	"strconv"
	"strings"
	"kamzhanyue/aoc/lib"
)

func Solve(input string) {
	lib.PrintDay(4)
	lib.PrintPartOne(partOne(input))
	lib.PrintPartTwo(partTwo(input))
}

func partOne(input string) int {
	result := 0
	problem := parse(input)
	for _, pair := range problem {
		if contains(pair.first, pair.second) || contains(pair.second, pair.first) {
			result += 1
		}
	}
	return result
}

func partTwo(input string) int {
	result := 0
	problem := parse(input)
	for _, pair := range problem {
		if overlaps(pair.first, pair.second) || overlaps(pair.second, pair.first) {
			result += 1
		}
	}
	return result
}

type Range struct {
	start int
	end int
}

type Pair struct {
	first Range
	second Range
}

type Problem = []Pair

func parse(input string) Problem {
	lines := lib.SplitLines(input)
	problem := make(Problem, len(lines))
	for i, line := range lines {
		problem[i] = parsePair(line)
	}
	return problem
}

func parsePair(line string) Pair {
	elves := strings.Split(line, ",")
	return Pair {
		first: parseRange(elves[0]),
		second: parseRange(elves[1]),
	}
}

func parseRange(line string) Range {
	splits := strings.Split(line, "-")
	start, _ := strconv.Atoi(splits[0])
	end, _ := strconv.Atoi(splits[1])
	return Range { start, end }
}

// Checks if b is completely in a
func contains(a Range, b Range) bool {
	// Check that the start and end of b is in a
	return b.start >= a.start && b.start <= a.end && b.end >= a.start && b.end <= a.end
}

// Checks if b overlaps with a
func overlaps(a Range, b Range) bool {
	return (b.start >= a.start && b.start <= a.end) || (b.end >= a.start && b.end <= a.end)
}
