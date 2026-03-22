package lib

import "strings"

func SplitLines(raw string) []string {
	return strings.Split(raw, "\n")
}
