package day4

import (
	"testing"
	"kamzhanyue/aoc/tests"
)

const INPUT = 
`2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8`

func TestPartOne(t *testing.T) {
	tests.AssertEq(t, partOne(INPUT), 2)
}

func TestPartTwo(t *testing.T) {
	tests.AssertEq(t, partTwo(INPUT), 4)
}
