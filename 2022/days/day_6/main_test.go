package day6

import (
	"testing"
	"kamzhanyue/aoc/tests"
)

func TestPartOne(t *testing.T) {
	tests.AssertEq(t, partOne("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7)
	tests.AssertEq(t, partOne("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5)
	tests.AssertEq(t, partOne("nppdvjthqldpwncqszvftbrmjlhg"), 6)
	tests.AssertEq(t, partOne("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10)
	tests.AssertEq(t, partOne("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11)
}

func TestPartTwo(t *testing.T) {
	tests.AssertEq(t, partTwo(""), 0)
}
