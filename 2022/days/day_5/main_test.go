package day5

import (
	"testing"
	"kamzhanyue/aoc/tests"
)

const INPUT = 
	`    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2`

func TestPartOne(t *testing.T) {
	tests.AssertEqStr(t, partOne(INPUT), "CMZ")
}

func TestPartTwo(t *testing.T) {
	tests.AssertEqStr(t, partTwo(INPUT), "MCD")
}
