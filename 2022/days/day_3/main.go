package day3

import (
	"kamzhanyue/aoc/lib"
)

func Solve(input string) {
	lib.PrintDay(3)
	lib.PrintPartOne(partOne(input))
	lib.PrintPartTwo(partTwo(input))
}

func partOne(input string) int {
	result := 0
	for _, rucksack := range parse(input) {
		result += getMatch(rucksack)
	}
	return result
}

func partTwo(input string) int {
	return 0
}

type Compartment map[int] bool

type Rucksack struct {
	left Compartment
	right Compartment
}

func parse(input string) []Rucksack {
	lines := make([]Rucksack, 0)
	for _, line := range lib.SplitLines(input) {
		lines = append(lines, parseLine(line))
	}
	return lines
}

func parseLine(line string) Rucksack {
	var left = line[0:len(line)/2]
	var right = line[len(line)/2:]
	return Rucksack {
		left: parseCompartment(left),
		right: parseCompartment(right),
	}
}

func parseCompartment(line string) Compartment {
	compartment := make(map[int]bool)
	for _, char := range line {
		val := getValue(char)
		if !compartment[val] {
			compartment[val] = true
		}
	}
	return compartment
}

func getValue(char rune) int {
	val := int(char)
	if val >= int('a') && val <= int('z') {
		return val - int('a') + 1
	}
	return val - int('A') + 27
}

func getMatch(rucksack Rucksack) int {
	// Return the first match between each rucksack
	// Guaranteed to be one match between left and right
	for key := range rucksack.left {
		if found := rucksack.right[key]; found {
			return key
		}
	}
	return 0
}
