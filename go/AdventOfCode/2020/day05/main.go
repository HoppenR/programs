package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"math"
	"strings"
)

func main() {
	passes, err := ReadLines("input")
	if err != nil {
		log.Fatalln(err)
	}
	fmt.Println("1:", GetHighestPassID(passes))
	ans2, err := FindMissingID(passes)
	if err != nil {
		log.Fatalln(err)
	}
	fmt.Println("2:", ans2)
}

func GetHighestPassID(passes []string) int {
	var highest int = 0
	for _, p := range passes {
		if pid := DecodePass(p); pid > highest {
			highest = pid
		}
	}
	return highest
}

func DecodeBinary(p string, highch rune) int {
	ans := 0.0
	for i, c := range p {
		if c == highch {
			ans += math.Pow(2, float64(len(p)-i-1))
		}
	}
	return int(ans)
}

func DecodePass(p string) int {
	cutoff := strings.IndexAny(p, "RL")
	RLHigh := math.Pow(2, float64(len(p)-cutoff))
	row := DecodeBinary(p[:cutoff], 'B')
	col := DecodeBinary(p[cutoff:], 'R')
	return row*int(RLHigh) + col
}

func FindMissingID(passes []string) (int, error) {
	highest := GetHighestPassID(passes)
	occupiedIDs := make([]bool, highest+1)
	for _, p := range passes {
		occupiedIDs[DecodePass(p)] = true
	}
	for i := 0; i < highest-2; i++ {
		if occupiedIDs[i+2] && !occupiedIDs[i+1] && occupiedIDs[i] {
			return i + 1, nil
		}
	}
	return 0, fmt.Errorf("Couldn't find an unoccupied spot between two occupied ones")
}

func ReadLines(filename string) ([]string, error) {
	content, err := ioutil.ReadFile(filename)
	if err != nil {
		return nil, err
	}
	lines := strings.Split(string(content), "\n")
	return lines[:len(lines)-1], nil
}
