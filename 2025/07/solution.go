package main

import (
  "fmt"
  "bufio"
  "os"
  "strings"
)

const SPLITTER = '^'

func partOne(lines []string) {
  start := strings.Index(lines[0], "S")
  beams := make([]bool, len(lines[0]))
  beams[start] = true

  splits := 0
  for i := 1; i < len(lines); i++ {
    nextBeams := beams
    for j, beam := range beams {
      if !beam { continue }
      if lines[i][j] == SPLITTER {
        splits++
        nextBeams[j] = false
        if j-1 >= 0 { nextBeams[j-1] = true }
        if j+1 < len(beams) { nextBeams[j+1] = true }
      }
    }
    beams = nextBeams
  }

  fmt.Printf("Part One is %d\n", splits)
}

func travel(lines []string, beam int, index int, cache map[[2]int]int) int {
  if cached := cache[[2]int{index, beam}]; cached > 0 {
    return cached
  }

  if index >= len(lines) {
    return 0
  }

  char := lines[index][beam]
  if char == SPLITTER {
    left := travel(lines, beam - 1, index + 1, cache)
    right := 1 + travel(lines, beam + 1, index + 1, cache)
    cache[[2]int{index, beam}] = left + right
    return left + right
  }
  return travel(lines, beam, index+1, cache)
}

func partTwo(lines []string) {
  start := strings.Index(lines[0], "S")
  cache := make(map[[2]int]int)
  total := 1 + travel(lines, start, 1, cache)
  fmt.Printf("Part Two is %d\n", total)
}

func main() {
  lines := make([]string, 0)
  scanner := bufio.NewScanner(os.Stdin)
  for scanner.Scan() {
    line := scanner.Text()
    lines = append(lines, line)
  }

  partOne(lines)
  partTwo(lines)
}

