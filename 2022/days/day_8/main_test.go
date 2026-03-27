package day8

import (
	"testing"
	"kamzhanyue/aoc/tests"
)

const INPUT = 
`30373
25512
65332
33549
35390`

func TestPartOne(t *testing.T) {
	tests.AssertEq(t, partOne(INPUT), 21)
}

func TestPartTwo(t *testing.T) {
	tests.AssertEq(t, partTwo(INPUT), 8)
}
