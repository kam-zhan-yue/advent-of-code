package main

import (
  "fmt"
  "bufio"
  "os"
  "strings"
)

const DOT = '.'
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

  fmt.Printf("Part One is %d", splits)
}

func main() {
  lines := make([]string, 0)
  scanner := bufio.NewScanner(os.Stdin)
  for scanner.Scan() {
    line := scanner.Text()
    lines = append(lines, line)
  }

  partOne(lines)
}

