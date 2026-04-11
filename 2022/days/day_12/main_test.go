package day12

import (
	"testing"
	"kamzhanyue/aoc/tests"
)

const INPUT =
`Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi`

func TestPartOne(t *testing.T) {
	tests.AssertEq(t, partOne(INPUT), 31)
}
