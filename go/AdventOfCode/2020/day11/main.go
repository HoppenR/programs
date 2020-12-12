package main

import (
	"bytes"
	"fmt"
	"io/ioutil"
	"log"
)

type Point struct {
	r int
	c int
}

var Directions = []Point{
	{-1, -1}, {0, -1}, {1, -1},
	{-1, 0}, {1, 0},
	{-1, 1}, {0, 1}, {1, 1},
}

func main() {
	lines, err := ReadLines("input")
	if err != nil {
		log.Fatalln(err)
	}
	fmt.Println("1:", FindStagnation(lines))
}

func FindStagnation(boardC [][]byte) (count int) {
	boardN := make([][]byte, len(boardC))
	for i := range boardN {
		boardN[i] = make([]byte, len(boardC[0]))
	}
	boards := [][][]byte{boardC, boardN}
	for {
		if !NextState(boards[0], boards[1]) {
			break
		}
		boards[0], boards[1] = boards[1], boards[0]
	}
	for i := range boards[0] {
		count += bytes.Count(boards[0][i], []byte("#"))
	}
	return
}

func NextState(boardC, boardN [][]byte) (changed bool) {
	for i := range boardC {
		for j := range boardC[i] {
			if boardC[i][j] == '.' {
				continue
			}
			occ := AdjOccupied(boardC, Point{i, j})
			if boardC[i][j] == 'L' && occ == 0 {
				boardN[i][j] = '#'
				changed = true
			} else if boardC[i][j] == '#' && occ >= 4 {
				boardN[i][j] = 'L'
				changed = true
			} else {
				boardN[i][j] = boardC[i][j]
			}
		}
	}
	return
}

func AdjOccupied(lines [][]byte, p Point) (occupied int) {
	for _, d := range Directions {
		if p.r+d.r < 0 || p.r+d.r >= len(lines) {
			continue
		}
		if p.c+d.c < 0 || p.c+d.c >= len(lines[0]) {
			continue
		}
		if lines[p.r+d.r][p.c+d.c] == '#' {
			occupied++
		}
	}
	return
}

func ReadLines(filename string) ([][]byte, error) {
	content, err := ioutil.ReadFile(filename)
	if err != nil {
		return nil, err
	}
	trimmed := bytes.TrimSpace(content)
	lines := bytes.Split(trimmed, []byte("\n"))
	return lines, nil
}
