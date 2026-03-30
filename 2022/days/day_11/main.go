package day11

import (
	"fmt"
	"slices"
	"strings"
	"kamzhanyue/aoc/lib"
	"kamzhanyue/aoc/lib/operations"
)

func Solve(input string) {

	lib.PrintDay(11)
	lib.PrintPartOne(partOne(input))
}

func partOne(input string) int {
	monkeys := parse(input)
	i := 0
	for i < 20 {
		monkeys = playRound(monkeys)
		i += 1
	}
	slices.SortFunc(monkeys, func(a, b Monkey) int {
		if a.inspections > b.inspections {
			return -1
		} else if a.inspections == b.inspections {
			return 0
		} else {
			return 1
		}
	})
	return monkeys[0].inspections * monkeys[1].inspections
}

type Operation struct {
	op operations.Operation
	val int
}

type Test struct {
	modulo int
	true int
	false int
}

type Monkey struct {
	id int
	inspections int
	items []int
	operation Operation
	test Test
}

func parse(input string) []Monkey {
	monkeys := make([]Monkey, 0)
	var monkey Monkey
	for _, line := range lib.SplitLines(input) {
		line = strings.Trim(line, " ")
		if strings.Contains(line, "Monkey") {
			monkey.id = lib.ParseInt(strings.ReplaceAll(strings.ReplaceAll(line, "Monkey ", ""), ":", ""))
		} else if strings.Contains(line, "Starting items") {
			monkey.items = make([]int, 0)
			for item := range strings.SplitSeq(strings.ReplaceAll(line, "Starting items: ", ""), ", ") {
				monkey.items = append(monkey.items, lib.ParseInt(item))
			}
		} else if strings.Contains(line, "Operation: ") {
			operation := lib.SplitWhitespace(strings.ReplaceAll(line, "Operation: new = old ", ""))
			if operation[0] == "*" && operation[1] == "old" {
				monkey.operation.op = operations.Square
			} else {
				monkey.operation.op = operations.Parse(operation[0])
				monkey.operation.val = lib.ParseInt(operation[1])
			}
		} else if strings.Contains(line, "Test") {
			monkey.test.modulo = lib.ParseInt(strings.ReplaceAll(line, "Test: divisible by ", ""))
		} else if strings.Contains(line, "If true") {
			monkey.test.true = lib.ParseInt(strings.ReplaceAll(line, "If true: throw to monkey ", ""))
		} else if strings.Contains(line, "If false") {
			monkey.test.false = lib.ParseInt(strings.ReplaceAll(line, "If false: throw to monkey ", ""))
		} else if line == "" {
			monkeys = append(monkeys, monkey)
		}
	}
	monkeys = append(monkeys, monkey)
	return monkeys
}

func playRound(monkeys []Monkey) []Monkey {
	for i, monkey := range monkeys {
		// fmt.Println("Monkey", monkey.id)
		for _, item := range monkey.items {
			// fmt.Println(" Monkey inspects an item with a worry level of", item)
			monkeys[i].inspections += 1

			switch monkey.operation.op {
				case operations.Multiply:
					item *= monkey.operation.val
					// fmt.Println("  Worry level is multiplied by", monkey.operation.val, "to", item)
				case operations.Subtract:
					item -= monkey.operation.val
					// fmt.Println("  Worry level is decreased by", monkey.operation.val, "to", item)
				case operations.Square:
					item *= item
					// fmt.Println("  Worry level is squared by", monkey.operation.val, "to", item)
				case operations.Add:
					item += monkey.operation.val
					// fmt.Println("  Worry level is increased by", monkey.operation.val, "to", item)
			}

			item /= 3
			// fmt.Println("  Monkey gets bored with item. Worry level is divided by 3 to", item)
			if item % monkey.test.modulo == 0{
				// fmt.Println("  Current worry level is divisible by", monkey.test.modulo)
				// fmt.Println("  Item with worry level", item, "is thrown to monkey", monkey.test.true)
				monkeys[monkey.test.true].items = append(monkeys[monkey.test.true].items, item)
			} else {
				// fmt.Println("  Current worry level is not divisible by", monkey.test.modulo)
				// fmt.Println("  Item with worry level", item, "is thrown to monkey", monkey.test.false)
				monkeys[monkey.test.false].items = append(monkeys[monkey.test.false].items, item)
			}
		}
		// The puzzle input doesn't let a monkey throw to itself
		monkeys[i].items = make([]int, 0)
	}
	return monkeys
}

func debug(round int, monkeys []Monkey) {
	fmt.Printf("After round %d, the monkeys are holding items with these worry levels:\n", round)
	for _, monkey := range monkeys {
		fmt.Println("Monkey", monkey.id, monkey.items)
		fmt.Println("Monkey inspected", monkey.inspections)
	}
}
