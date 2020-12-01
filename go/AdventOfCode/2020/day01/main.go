package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

func main() {
	expenses, err := scanpayments("input")
	if err != nil {
		log.Fatal(err)
	}
	ans1, err := findsum(expenses, 2, 2020)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println("1: ", ans1)
	ans2, err := findsum(expenses, 3, 2020)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println("2: ", ans2)
}

func scanpayments(filename string) ([]int, error) {
	file, err := os.Open(filename)
	if err != nil {
		return nil, err
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)
	var expenses []int = make([]int, 0, 200)
	for scanner.Scan() {
		payment, err := strconv.Atoi(scanner.Text())
		if err != nil {
			return nil, err
		}
		expenses = append(expenses, payment)
	}
	if err := scanner.Err(); err != nil {
		return nil, err
	}
	return expenses, nil
}

// TODO: rewrite
func findsum(expenses []int, nints int, req int) (ans int, err error) {
	// array of nints number of integers
	// loop over all integers and add to array until filled
	// When not right answer pop last and add 4th, 5th, 6th, when reaching last
	// integer then pop (nints - 1)th and move 1 over, continue
	for _, v := range expenses {
		for _, w := range expenses {
			if nints == 3 {
				for _, t := range expenses {
					if v+w+t == req {
						return v * w * t, nil
					}
				}
			} else {
				if v+w == req {
					return v * w, nil
				}
			}
		}
	}
	return 0, fmt.Errorf("Could not find %d numbers that sum up to 2020", nints)
}
