package day1

import (
	"testing"
	"kamzhanyue/aoc/tests"
)

const INPUT = 
`1000
2000
3000

4000

5000
6000

7000
8000
9000

10000`

func TestPartOne(t *testing.T) {
	tests.AssertEq(t, partOne(INPUT), 24000)
}

func TestPartTwo(t *testing.T) {
	tests.AssertEq(t, partTwo(INPUT), 45000)
}
