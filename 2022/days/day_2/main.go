package day2

import (
	"kamzhanyue/aoc/lib"
)

func Solve(input string) {
	lib.PrintDay(2)
	lib.PrintPartOne(partOne(input))
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
	return 0
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

func getScore(round Round) int {
	// Draw
	if round.opponent == round.you {
		return getValue(round.you) + 3
	}

	if isWon(round) {
		return getValue(round.you) + 6
	}

	return getValue(round.you)
}

func isWon(round Round) bool {
	if round.you == Rock && round.opponent == Scissors {
		return true
	}
	if round.you == Paper && round.opponent == Rock {
		return true
	}
	if round.you == Scissors && round.opponent == Paper {
		return true
	}
	return false
}


func getValue(hand Hand) int {
	switch hand {
		case Rock: return 1
		case Paper: return 2
		case Scissors: return 3
		default: return 0
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
