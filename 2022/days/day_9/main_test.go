package day9

import (
	"testing"
	"kamzhanyue/aoc/tests"
)

const INPUT = 
	`R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2`

func TestPartOne(t *testing.T) {
	tests.AssertEq(t, partOne(INPUT), 13)
}
//
const INPUT_TWO = 
`R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20`

func TestPartTwo(t *testing.T) {
	tests.AssertEq(t, partTwo(INPUT_TWO), 36)
}
