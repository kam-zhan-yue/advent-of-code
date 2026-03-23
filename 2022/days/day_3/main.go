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
	result := 0
	for _, group := range parseGroups(input) {
		result += getMatchInGroup(group)
	}
	return result
}

type Compartment map[int] bool

type Rucksack struct {
	left Compartment
	right Compartment
}

type Group [3]Compartment

func parse(input string) []Rucksack {
	lines := make([]Rucksack, 0)
	for _, line := range lib.SplitLines(input) {
		lines = append(lines, parseLine(line))
	}
	return lines
}

func parseGroups(input string) []Group {
	groups := make([]Group, 0)

	lines := lib.SplitLines(input)
	for i := 0; i < len(lines); i += 3 {
		var group Group
		group[0] = parseCompartment(lines[i])
		group[1] = parseCompartment(lines[i+1])
		group[2] = parseCompartment(lines[i+2])
		groups = append(groups, group)
	}

	return groups
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
		if rucksack.right[key] {
			return key
		}
	}
	return 0
}

func getMatchInGroup(group Group) int {
	// Guaranteed to be one match in all groups
	for key := range group[0] {
		if group[1][key] && group[2][key] {
			return key
		}
	}
	return 0
}
