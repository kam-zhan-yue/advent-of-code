package day2

import (
	"kamzhanyue/aoc/lib"
)

func Solve(input string) {
	lib.PrintDay(2)
	lib.PrintPartOne(partOne(input))
	lib.PrintPartTwo(partTwo(input))
}

func partOne(input string) int {
	rounds := parse(input)
	result := 0
	for i := range rounds {
		result += getScore(rounds[i])
	}
	return result
}

func partTwo(input string) int {
	rounds := parse(input)
	result := 0
	for i := range rounds {
		result += getPlay(rounds[i])
	}
	return result
}

type Round struct {
	opponent Hand
	you Hand
}

type Hand int

const (
	Rock Hand = iota
	Paper
	Scissors
)

func getPlay(round Round) int {
	switch round.you {
		case Rock: return getValue(getLosing(round.opponent))
		case Paper: return getValue(round.opponent) + 3
		default: return getValue(getWinning(round.opponent)) + 6
	}
}

func getScore(round Round) int {
	// Draw
	if round.opponent == round.you {
		return getValue(round.you) + 3
	}

	if round.you == getWinning(round.opponent) {
		return getValue(round.you) + 6
	}

	return getValue(round.you)
}

func getWinning(hand Hand) Hand {
	switch hand {
		case Rock: return Paper
		case Paper: return Scissors
		default: return Rock
	}
}

func getLosing(hand Hand) Hand {
	switch hand {
		case Rock: return Scissors
		case Paper: return Rock
		default: return Paper
	}
}

func getValue(hand Hand) int {
	switch hand {
		case Rock: return 1
		case Paper: return 2
		default: return 3
	}
}

func convert(char string) Hand {
	switch char {
		case "A": return Rock
		case "B": return Paper
		case "C": return Scissors
		case "X": return Rock
		case "Y": return Paper
		case "Z": return Scissors
		default: return Rock
	}
}

func parse(input string) []Round {
	rounds := make([]Round, 0)
	lines := lib.SplitLines(input)
	for i := range lines {
		chars := lib.SplitWhitespace(lines[i])
		round := Round { convert(chars[0]), convert(chars[1]) }
		rounds = append(rounds, round)
	}
	return rounds
}
