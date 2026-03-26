package day7

import (
	"testing"
	"kamzhanyue/aoc/tests"
)

const INPUT = 
`$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k`

func TestPartOne(t *testing.T) {
	tests.AssertEq(t, partOne(INPUT), 95437)
}

func TestPartTwo(t *testing.T) {
	tests.AssertEq(t, partTwo(INPUT), 24933642)
}
