package day2

import (
	"testing"
	"kamzhanyue/aoc/tests"
)

const INPUT = 
`A Y
B X
C Z`

func TestPartOne(t *testing.T) {
	tests.AssertEq(t, partOne(INPUT), 15)
}

func TestPartTwo(t *testing.T) {
	tests.AssertEq(t, partTwo(INPUT), 12)
}
