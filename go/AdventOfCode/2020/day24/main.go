package main

import (
	"bytes"
	"fmt"
	"io/ioutil"
	"log"
)

type Coord struct {
	x int
	y int
}

func main() {
	movement, err := ReadLabels("input")
	if err != nil {
		log.Fatalln(err)
	}
	fmt.Println("1:", Paint(movement))
}

func Paint(movement []Coord) int {
	painted := make(map[Coord]struct{})
	var pos = Coord{0, 0}
	for _, v := range movement {
		pos.x += v.x
		pos.y += v.y
		if v == (Coord{0, 0}) {
			// A movement of {0, 0} means paint and reset pos
			if _, ok := painted[pos]; !ok {
				painted[pos] = struct{}{}
			} else {
				delete(painted, pos)
			}
			pos = v
		}
	}
	return len(painted)
}

func ReadLabels(filename string) ([]Coord, error) {
	content, err := ioutil.ReadFile(filename)
	if err != nil {
		return nil, err
	}
	content = bytes.TrimRight(content, "\n")
	var movement []Coord
	for _, l := range bytes.Split(content, []byte{'\n'}) {
		for i := 0; i < len(l); i++ {
			switch l[i] {
			case 'e':
				movement = append(movement, Coord{2, 0})
			case 'w':
				movement = append(movement, Coord{-2, 0})
			case 'n':
				i++
				switch l[i] {
				case 'e':
					movement = append(movement, Coord{1, 2})
				case 'w':
					movement = append(movement, Coord{-1, 2})
				}
			case 's':
				i++
				switch l[i] {
				case 'e':
					movement = append(movement, Coord{1, -2})
				case 'w':
					movement = append(movement, Coord{-1, -2})
				}
			}
		}
		// A movement of {0, 0} means paint and reset pos
		movement = append(movement, Coord{0, 0})
	}
	return movement, nil
}
