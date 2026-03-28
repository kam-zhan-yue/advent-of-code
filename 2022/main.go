package main

import (
	"kamzhanyue/aoc/lib"
	"kamzhanyue/aoc/days/day_1"
	"kamzhanyue/aoc/days/day_2"
	"kamzhanyue/aoc/days/day_3"
	"kamzhanyue/aoc/days/day_4"
	"kamzhanyue/aoc/days/day_5"
	"kamzhanyue/aoc/days/day_6"
	"kamzhanyue/aoc/days/day_7"
	"kamzhanyue/aoc/days/day_8"
	"kamzhanyue/aoc/days/day_9"
)

func main() {
	day1.Solve(lib.ReadDay("01"));
	day2.Solve(lib.ReadDay("02"));
	day3.Solve(lib.ReadDay("03"));
	day4.Solve(lib.ReadDay("04"));
	day5.Solve(lib.ReadDay("05"));
	day6.Solve(lib.ReadDay("06"));
	day7.Solve(lib.ReadDay("07"));
	day8.Solve(lib.ReadDay("08"));
	day9.Solve(lib.ReadDay("09"));
}
