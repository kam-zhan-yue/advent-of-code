package day12

import (
	"math"
	"fmt"
	"container/heap"
	"kamzhanyue/aoc/lib"
)

func Solve(input string) {
	lib.PrintDay(12)
	lib.PrintPartOne(partOne(input))
	lib.PrintPartTwo(partTwo(input))
}

func partOne(input string) int {
	mountain := parse(input)
	return dijkstra(mountain)
}

func partTwo(input string) int {
	return 0
}

type Position = lib.Position
type Direction = lib.Direction

type Node struct {
	dist int
	index int
}

type Mountain struct {
	grid [][]int
	rows int
	cols int
	start Position
	end Position
}

type PriorityQueue []*Node

func (pq PriorityQueue) Len() int {
	return len(pq)
}

func (pq PriorityQueue) Less(i, j int) bool {
	return pq[i].dist < pq[j].dist
}

func (pq PriorityQueue) Swap(i, j int) {
	pq[i], pq[j] = pq[j], pq[i]
	pq[i].index = i
	pq[j].index = j
}

func (pq *PriorityQueue) Push(x any) {
	n := len(*pq)
	item := x.(*Node)
	item.index = n
	*pq = append(*pq, item)
}

func (pq *PriorityQueue) Pop() any {
	old := *pq
	n := len(old)
	item := old[n-1]
	old[n-1] = nil // don't stop the GC from reclaiming the item eventually
	item.index = -1 // for safety
	*pq = old[0 : n-1]
	return item
}

func (pq *PriorityQueue) update(node *Node, dist int) {
	node.dist = dist
	heap.Fix(pq, node.index)
}


func parse(input string) Mountain {
	var mountain Mountain
	mountain.grid = make([][]int, 0)
	for x, line := range lib.SplitLines(input) {
		row := make([]int, 0)
		for y, c := range line {
			pos := Position { X: x, Y: y }
			switch c {
				case 'S':
					mountain.start = pos
					row = append(row, int('a'))
				case 'E':
					mountain.end = pos
					row = append(row, int('z'))
				default:
					row = append(row, int(c))
				}
		}
		mountain.grid = append(mountain.grid, row)
	}
	return mountain
}

func dijkstra(mountain Mountain) int {
	queue := make(PriorityQueue, 0)
	posToNode := make(map[Position]*Node)
	nodeToPos := make(map[*Node]Position)
	for row := range len(mountain.grid) {
		for col := range len(mountain.grid[0]) {
			var node Node
			pos := Position { X: row, Y: col }
			posToNode[pos] = &node
			nodeToPos[&node] = pos
			if pos == mountain.start {
				node.dist = 0
			} else {
				node.dist = math.MaxInt
			}
			heap.Push(&queue, &node)
		}
	}


	for queue.Len() > 0 {
		// 1. Pop out the current node. The initial node is the starting node
		item := heap.Pop(&queue).(*Node)
		pos := nodeToPos[item]
		char := mountain.grid[pos.X][pos.Y]
		// 2. Get all the neighbouring nodes and update their positions
		for _, dir := range lib.AllDirections {
			moved := pos.Move(dir)
			// Check if the position is legal
			if !lib.InRange(moved.X, 0, len(mountain.grid) - 1) { continue }
			if !lib.InRange(moved.Y, 0, len(mountain.grid[0]) - 1) { continue }
			// Check if the position is reachable
			if mountain.grid[moved.X][moved.Y] <= char + 1 {
				if moved == mountain.end {
					return item.dist + 1
				}
				// Get the node at the moved position and update its position
				node := posToNode[moved]
				// Update the priority queue
				queue.update(node, min(node.dist, item.dist + 1))
			}
		}
	}
	return -1
}

func debug(mountain Mountain) {
	for i := range len(mountain.grid) {
		for j := range len(mountain.grid[i]) {
			fmt.Print(string(rune(mountain.grid[i][j])))
		}
		fmt.Println()
	}
}
