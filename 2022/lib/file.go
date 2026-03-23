package lib

import (
	"strings"
	"path/filepath"
	"os"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func ReadDay(day string) string {
	path := filepath.Join("inputs", day)
	dat, err := os.ReadFile(path)
	check(err)
	return strings.Trim(string(dat), "\n")
}
