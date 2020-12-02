package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

type Policy struct {
	lo      int
	hi      int
	char    byte
	content string
}

func main() {
	policies, err := scanpolicies("input")
	if err != nil {
		log.Fatalln("couldnt get lines from file")
	}
	fmt.Println("1: ", CountValid(policies, 1))
	fmt.Println("2: ", CountValid(policies, 2))
}

func scanpolicies(filename string) ([]Policy, error) {
	file, err := os.Open(filename)
	if err != nil {
		return nil, err
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)
	var policies []Policy
	for scanner.Scan() {
		var p Policy
		splits := strings.SplitN(scanner.Text(), "-", 2)
		p.lo, err = strconv.Atoi(splits[0])
		if err != nil {
			return nil, err
		}
		splits = strings.SplitN(splits[1], " ", 2)
		p.hi, err = strconv.Atoi(splits[0])
		if err != nil {
			return nil, err
		}
		splits = strings.SplitN(splits[1], ":", 2)
		p.char = splits[0][0]
		splits = strings.SplitN(splits[1], " ", 2)
		p.content = splits[1]
		policies = append(policies, p)
	}
	if err := scanner.Err(); err != nil {
		return nil, err
	}
	return policies, nil
}

func CountValid(policies []Policy, ruleset int) int {
	var nValid int
	if ruleset == 1 {
		for _, v := range policies {
			if c := strings.Count(v.content, string(v.char)); c >= v.lo && c <= v.hi {
				nValid++
			}
		}
	} else if ruleset == 2 {
		for _, v := range policies {
			if (v.content[v.lo-1] == v.char) != (v.content[v.hi-1] == v.char) {
				nValid++
			}
		}
	}
	return nValid
}
