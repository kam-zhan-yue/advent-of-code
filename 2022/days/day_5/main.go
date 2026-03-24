package day5

import (
	"kamzhanyue/aoc/lib"
	"slices"
	"regexp"
	"strings"
)

func Solve(input string) {
	lib.PrintDay(5)
	lib.PrintPartOneStr(partOne(input))
	lib.PrintPartTwoStr(partTwo(input))
}

func partOne(input string) string {
	problem := parse(input)
	for _, instruction := range problem.instructions {
		processInstruction(problem.stacks, instruction)
	}
	return readStacks(problem.stacks)
}

func partTwo(input string) string {
	problem := parse(input)
	for _, instruction := range problem.instructions {
		processInstruction9001(problem.stacks, instruction)
	}
	return readStacks(problem.stacks)
}

type Problem struct {
	stacks []Stack
	instructions []Instruction
}

type Stack = []string

type Instruction struct {
	amount int
	from int
	to int
}

func parse(input string) Problem {
	// Reverse so that we read the instructions first
	lines := lib.SplitLines(input)
	slices.Reverse(lines)
	i := 0
	// Parse Instructions
	instructions := make([]Instruction, 0)
	for i < len(lines) {
		if lines[i] == "" {
			i++
			break
		}
		instructions = append(instructions, parseInstruction(lines[i]))
		i++
	}
	slices.Reverse(instructions)

	stacks := parseStack(lines[i:])

	return Problem { stacks, instructions }
}

func parseInstruction(line string) Instruction {
	re := regexp.MustCompile(`move (?P<amount>\d+) from (?P<from>\d+) to (?P<to>\d+)`)
	match := re.FindStringSubmatch(line)
	return Instruction {
		lib.ParseInt(match[1]),
		lib.ParseInt(match[2]) - 1,
		lib.ParseInt(match[3]) - 1,
	}
}

func parseStack(lines []string) []Stack {
	limit := strings.ReplaceAll(lines[0], " ", "")
	length := lib.ParseInt(string(limit[len(limit)-1]))
	stacks := make([]Stack, length)
	for i := range length {
		stacks[i] = make(Stack, 0)
	}

	for i := 1; i < len(lines); i++ {
		for j := range length {
			char := lines[i][1 + j * 4]
			if char != ' ' {
				stacks[j] = append(stacks[j], string(char))
			}
		}
	}
	return stacks
}

func processInstruction(stacks []Stack, instruction Instruction) {
	i := 0
	for i < instruction.amount {
		val, stack := pop(stacks[instruction.from])
		stacks[instruction.from] = stack
		stacks[instruction.to] = append(stacks[instruction.to], val)
		i++
	}
}

func processInstruction9001(stacks []Stack, instruction Instruction) {
	from := stacks[instruction.from]
	popped := from[len(from) - instruction.amount:]
	stacks[instruction.from] = from[:len(from) - instruction.amount]
	for _, val := range popped {
		stacks[instruction.to] = append(stacks[instruction.to], val)
	}
}

func pop(stack Stack) (string, Stack) {
	val, updated := stack[len(stack)-1], stack[:len(stack)-1]
	return val, updated
}

func readStacks(stacks []Stack) string {
	var result strings.Builder
	for _, stack := range stacks {
		if len(stack) > 0 {
			result.WriteString(stack[len(stack) - 1])
		}
	}
	return result.String()
}
