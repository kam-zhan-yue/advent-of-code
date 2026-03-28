package day9

import (
	"fmt"
	"kamzhanyue/aoc/lib"
)

func Solve(input string) {
	lib.PrintDay(9)
	lib.PrintPartOne(partOne(input))
	lib.PrintPartTwo(partTwo(input))
}

func partOne(input string) int {
	rope := initRope(2)
	instructions := parse(input)
	process(rope, instructions)
	return len(rope.visited)
}

func partTwo(input string) int {
	rope := initRope(10)
	instructions := parse(input)
	process(rope, instructions)
	return len(rope.visited)
}

type Position = lib.Position
type Direction = lib.Direction
type Rope struct {
	head *Node
	visited map[Position]bool
}

type Node struct {
	pos lib.Position
	next *Node
	val int
	tracking bool
}

type Instruction struct {
	direction Direction
	distance int
}

func initRope(length int) Rope {
	var head Node
	head.val = -1
	buildRope(&head, length - 1, 1)
	visited := make(map[Position]bool)
	visited[head.pos] = true
	return Rope { head: &head, visited: visited}
}

func buildRope(node *Node, length int, val int) {
	var next Node
	next.val = val
	node.next = &next
	if length == 1 {
		next.tracking = true
	} else {
		buildRope(&next, length - 1, val + 1)
	}
}

func parse(input string) []Instruction {
	instructions := make([]Instruction, 0)
	for _, line := range lib.SplitLines(input) {
		split := lib.SplitWhitespace(line)

		instruction := Instruction {
			direction: lib.ParseDirection(split[0]),
			distance: lib.ParseInt(split[1]),
		}
		instructions = append(instructions, instruction)
	}
	return instructions
}

func process(rope Rope, instructions []Instruction) {
	for _, instruction := range instructions {
		processInstruction(rope, instruction)
	}
}

func processInstruction(rope Rope, instruction Instruction) {
	i := 0
	for i < instruction.distance {
		move(rope, instruction.direction)
		// fmt.Println()
		// visualise(rope, 6)
		i += 1
	}
}

func move(rope Rope, direction Direction) {
	rope.head.pos = lib.Move(rope.head.pos, direction)
	pull(rope, rope.head.next, rope.head.pos)
}

func touching(a Position, b Position) bool {
	diff := lib.Subtract(a, b)
	return lib.Abs(diff.X) <= 1 && lib.Abs(diff.Y) <= 1
}

func pull(rope Rope, node *Node, currPos Position) {
	if node == nil {
		return
	}
	if touching(node.pos, currPos) {
		return
	}
	diff := lib.Subtract(currPos, node.pos)
	x := lib.Clamp(diff.X, -1, 1)
	y := lib.Clamp(diff.Y, -1, 1)
	if lib.Abs(x) == 0 {
		node.pos = lib.Add(node.pos, Position { X: 0, Y: y })
	} else if lib.Abs(y) == 0 {
		node.pos = lib.Add(node.pos, Position { X: x, Y: 0 })
	} else {
		node.pos = lib.Add(node.pos, Position { X: x, Y: y })
	}

	if node.tracking {
		rope.visited[node.pos] = true
	}

	pull(rope, node.next, node.pos)
}

func visualise(rope Rope, size int) {
	positions := make(map[Position]int, 0)
	node := rope.head
	for node != nil {
		if positions[node.pos] == 0 {
			positions[node.pos] = node.val
		}
		node = node.next
	}

	for x := -size; x <= size; x++ {
		for y := -size; y <= size; y++ {
			pos := Position { X: x, Y: y }
			switch positions[pos] {
				case 0: fmt.Print(".")
				case -1: fmt.Print("H")
				default: fmt.Print(positions[pos])
			}
		}
		fmt.Println()
	}

}
