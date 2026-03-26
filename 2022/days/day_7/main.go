package day7

import (
	"fmt"
	"strings"
	"kamzhanyue/aoc/lib"
)

func Solve(input string) {
	lib.PrintDay(7)
	lib.PrintPartOne(partOne(input))
	lib.PrintPartTwo(partTwo(input))
}

func partOne(input string) int {
	directory := parse(input)
	cache := make(map[*Directory]int)
	getSize(directory, cache)
	result := 0
	for dir := range cache {
		if cache[dir] <= 100000 {
			result += cache[dir]
		}
	}
	return result
}

func partTwo(input string) int {
	directory := parse(input)
	cache := make(map[*Directory]int)
	total_size := getSize(directory, cache)
	unused_space := 70000000 - total_size
	required_space := 30000000 - unused_space
	min_size := total_size
	for _, size := range cache {
		if size >= required_space && size < min_size {
			min_size = size
		}
	}
	return min_size
}

type Directory struct {
	name string
	parent *Directory
	directories []*Directory
	files []File
}

type File struct {
	name string
	size int
}

func create(name string, parent *Directory) *Directory {
	return &Directory {
		name,
		parent,
		make([]*Directory, 0),
		make([]File, 0),
	}
}

func createFile(line string) File {
	splits := lib.SplitWhitespace(line)
	return File { name: splits[1], size: lib.ParseInt(splits[0]) }
}

func parse(input string) *Directory {
	root := create("/", nil)
	current := root
	// Skip the first line as it is always $ cd /
	for _, line := range lib.SplitLines(input)[1:] {
		if line == "$ cd .." {
			// Go up a level
			current = current.parent
		} else if strings.Contains(line, "$ cd") {
			// Find the child directory in the current directory and set to current
			current = findDirectory(current, strings.ReplaceAll(line, "$ cd ", ""))
		} else if line == "$ ls" {
			continue
		} else if strings.Contains(line, "dir ") {
			// Add a directory to the current directory
			addDirectory(current, strings.ReplaceAll(line, "dir ", ""))
		} else {
			// Add a file to the current directory
			addFile(current, createFile(line))
		}
	}
	return root
}

func findDirectory(parent *Directory, name string) *Directory {
	for _, directory := range parent.directories {
		if directory.name == name {
			return directory
		}
	}
	return nil
}


func addDirectory(parent *Directory, name string) {
	directory := create(name, parent)
	parent.directories = append(parent.directories, directory)
}

func addFile(parent *Directory, file File) {
	parent.files = append(parent.files, file)
}

func printDirectory(directory *Directory, level int) {
	// Print the name of the directory first
	spacing := strings.Repeat(" ", level)
	fmt.Printf("%s - %s (dir)\n", spacing, directory.name)
	// Print child directories
	for _, dir := range directory.directories {
		printDirectory(dir, level + 1)
	}

	// Print files
	for _, file := range directory.files {
		fmt.Printf(" %s - %s (file, size=%d)\n", spacing, file.name, file.size)
	}
}

func getSize(directory *Directory, cache map[*Directory]int) int {
	size := 0
	if cache[directory] > 0 {
		return cache[directory]
	}

	for _, dir := range directory.directories {
		size += getSize(dir, cache)
	}

	for _, file := range directory.files {
		size += file.size
	}

	cache[directory] = size
	return size
}
