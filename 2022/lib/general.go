package lib

import (
	"strings"
	"strconv"
)

func SplitLines(raw string) []string {
	return strings.Split(raw, "\n")
}

func SplitWhitespace(raw string) [] string {
	return strings.Split(raw, " ")
}

func ParseInt(s string) int {
	val, _ := strconv.Atoi(s)
	return val
}

func Clamp(val int, min int, max int) int {
	if val < min {
		return min
	}
	if val > max {
		return max
	}
	return val
}
