package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"strings"
)

type Cube struct {
	x int
	y int
	z int
}

type ActiveCubes = map[Cube]struct{}

var Active = struct{}{}

func main() {
	source, err := ReadLines("input")
	if err != nil {
		log.Fatalln(err)
	}
	fmt.Println("1:", BootSource(source))
}

func BootSource(source ActiveCubes) int {
	bootCycles := 6
	for i := 0; i < bootCycles; i++ {
		source = NextState(source)
	}
	return len(source)
}

func NextState(sourceC ActiveCubes) ActiveCubes {
	sourceN := make(ActiveCubes, 0)
	candidates := make(map[Cube]int, 0)
	for c := range sourceC {
		var cnt int
		for oc := range sourceC {
			if c == oc {
				continue
			}
			if IsClose(c, oc) {
				cnt++
			}
		}
		for x := c.x - 1; x <= c.x+1; x++ {
			for y := c.y - 1; y <= c.y+1; y++ {
				for z := c.z - 1; z <= c.z+1; z++ {
					candidates[Cube{x, y, z}]++
				}
			}
		}
		if cnt == 2 || cnt == 3 {
			sourceN[c] = Active
		}
	}
	for c, cnt := range candidates {
		if cnt == 3 {
			sourceN[c] = Active
		}
	}
	return sourceN
}

func IsClose(c Cube, oc Cube) bool {
	if IntAbs(c.x-oc.x) > 1 ||
		IntAbs(c.y-oc.y) > 1 ||
		IntAbs(c.z-oc.z) > 1 {
		return false
	} else {
		return true
	}
}

func IntAbs(n int) int {
	if n < 0 {
		return -n
	} else {
		return n
	}
}

func ReadLines(filename string) (ActiveCubes, error) {
	content, err := ioutil.ReadFile(filename)
	if err != nil {
		return nil, err
	}
	lines := strings.Split(string(content), "\n")
	board := make(ActiveCubes, 0)
	for y := 0; y < len(lines); y++ {
		for x := 0; x < len(lines[y]); x++ {
			if lines[y][x] == '#' {
				board[Cube{x, y, 0}] = Active
			}
		}
	}
	return board, nil
}
