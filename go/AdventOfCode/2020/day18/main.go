package main

import (
	"bytes"
	"fmt"
	"io/ioutil"
	"log"
	"regexp"
	"strconv"
)

func main() {
	numbers, err := ReadLines("input")
	if err != nil {
		log.Fatalln(err)
	}
	fmt.Println("1:", EvalFile(numbers))
}

var pattern = regexp.MustCompile(`(\([^()]+\))`)

func EvalFile(numbers [][]byte) (sum int) {
	for _, l := range numbers {
		sum += EvalLine(l)
	}
	return
}

func EvalLine(line []byte) int {
	var running = true
	for running {
		ixs := pattern.FindSubmatchIndex(line)
		if ixs != nil {
			q := append(line[:ixs[0]], EvalExpr(line[ixs[0]+1:ixs[1]-1])...)
			line = append(q, line[ixs[1]:]...)
		} else {
			running = false
			break
		}
	}
	sum, err := strconv.Atoi(string(EvalExpr(line)))
	if err != nil {
		panic(err)
	}
	return sum
}

func SplitWords(line []byte) [][]byte {
	words := make([][]byte, 0)
	acc := make([]byte, 0)
	for i := 0; i < len(line); i++ {
		switch line[i] {
		case '+':
			words = append(words, acc)
			words = append(words, line[i:i+1])
			acc = []byte{}
		case '*':
			words = append(words, acc)
			words = append(words, line[i:i+1])
			acc = []byte{}
		default:
			acc = append(acc, line[i])
		}
	}
	words = append(words, acc)
	return words
}

func EvalExpr(line []byte) []byte {
	words := SplitWords(line)
	sum, err := strconv.Atoi(string(words[0]))
	if err != nil {
		panic(err)
	}
	for i := 1; i < len(words); i += 2 {
		switch words[i][0] {
		case '+':
			m, err := strconv.Atoi(string(words[i+1]))
			if err != nil {
				panic(err)
			}
			sum += m
		case '*':
			m, err := strconv.Atoi(string(words[i+1]))
			if err != nil {
				panic(err)
			}
			sum *= m
		}
	}
	return []byte(strconv.Itoa(sum))
}

func ReadLines(filename string) ([][]byte, error) {
	content, err := ioutil.ReadFile(filename)
	if err != nil {
		return nil, err
	}
	trimcontent := bytes.TrimRight(content, "\n")
	trimcontent = bytes.ReplaceAll(trimcontent, []byte{' '}, nil)
	numbers := bytes.Split([]byte(trimcontent), []byte{'\n'})
	return numbers, nil
}
