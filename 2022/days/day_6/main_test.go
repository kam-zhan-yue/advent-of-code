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
	tests.AssertEq(t, partTwo("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19)
	tests.AssertEq(t, partTwo("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23)
	tests.AssertEq(t, partTwo("nppdvjthqldpwncqszvftbrmjlhg"), 23)
	tests.AssertEq(t, partTwo("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29)
	tests.AssertEq(t, partTwo("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26)
}
