package day3

import (
	"testing"
	"kamzhanyue/aoc/tests"
)

const INPUT = 
`vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw`

func TestPartOne(t *testing.T) {
	tests.AssertEq(t, partOne(INPUT), 157)
}

func TestPartTwo(t *testing.T) {
	tests.AssertEq(t, partTwo(INPUT), 70)
}
