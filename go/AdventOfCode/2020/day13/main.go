package main

import (
	"bufio"
	"fmt"
	"io/ioutil"
	"log"
	"os"
	"strconv"
	"strings"
	"math"
)

func main() {
	departure, input, err := ReadInts("input")
	if err != nil {
		log.Fatalln(err)
	}
	fmt.Println("1:", EarliestBus(departure, input))
}

func EarliestBus(departure int, input []interface{}) (ans int) {
	firstBus := math.MaxInt64
	busID := 0
	for _, in := range input {
		switch v:= in.(type) {
		case int:
			for i := departure; ; i++ {
				if i % v == 0 {
					if i < firstBus {
						firstBus = i
						busID = v
					}
					break
				}
			}
		}
	}
	return (firstBus - departure) * busID
}

func ReadLines(filename string) ([]string, error) {
	content, err := ioutil.ReadFile(filename)
	if err != nil {
		return nil, err
	}
	lines := strings.Split(string(content), "\n")
	return lines, nil
}

func ReadFile(filename string) (string, error) {
	content, err := ioutil.ReadFile(filename)
	if err != nil {
		return "", err
	}
	return string(content), nil
}

func ReadInts(filename string) (int, []interface{}, error) {
	file, err := os.Open(filename)
	if err != nil {
		return 0, nil, err
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)
	scanner.Scan()
	departure, err := strconv.Atoi(scanner.Text())
	if err != nil {
		return 0, nil, err
	}
	expenses := make([]interface{}, 0)
	scanner.Scan()
	for _, l := range strings.Split(scanner.Text(), ",") {
		val, err := strconv.Atoi(l)
		if err != nil {
			expenses = append(expenses, l)
		} else {
			expenses = append(expenses, val)
		}
	}
	if err := scanner.Err(); err != nil {
		return 0, nil, err
	}
	return departure, expenses, nil
}
