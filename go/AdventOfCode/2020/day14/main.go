package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"regexp"
	"strconv"
	"strings"
)

type WriteOp struct {
	addr uint64
	val  uint64
}

type Routine struct {
	bitmask string
	writes  []WriteOp
}

func main() {
	program, err := ReadProgram("input")
	if err != nil {
		log.Fatalln(err)
	}
	fmt.Println("1:", RunProgram(program))
}

func RunProgram(program []Routine) uint64 {
	memory := make(map[uint64]uint64)
	for _, r := range program {
		for _, w := range r.writes {
			for bit := range r.bitmask {
				var mask uint64 = 1 << (len(r.bitmask) - 1 - bit)
				switch r.bitmask[bit] {
				case '0':
					w.val &= ^mask
				case '1':
					w.val |= mask
				}
			}
			memory[w.addr] = w.val
		}
	}
	var sum uint64
	for _, v := range memory {
		sum += v
	}
	return sum
}

func ReadProgram(filename string) ([]Routine, error) {
	content, err := ioutil.ReadFile(filename)
	if err != nil {
		return nil, err
	}
	program := make([]Routine, 0)
	pattern := regexp.MustCompile(`mem\[(\d+)\] = (\d+)`)
	for _, sec := range strings.Split(string(content), "mask = ")[1:] {
		lines := strings.Split(strings.TrimRight(sec, "\n"), "\n")
		var rout Routine
		rout.bitmask = lines[0]
		for _, l := range lines[1:] {
			groups := pattern.FindStringSubmatch(l)
			mem, err := strconv.ParseUint(groups[1], 10, 36)
			if err != nil {
				return nil, err
			}
			val, err := strconv.ParseUint(groups[2], 10, 36)
			if err != nil {
				return nil, err
			}
			rout.writes = append(rout.writes, WriteOp{mem, val})
		}
		program = append(program, rout)
	}
	return program, nil
}
