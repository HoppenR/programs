package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

func main() {
	diagnostic, nBits, err := ReadDiagnostic("input")
	if err != nil {
		log.Fatalln(err)
	}
	fmt.Println("1:", PowerConsumption(diagnostic, nBits))
}

func PowerConsumption(diagnostic []int64, nBits int) int {
	var ε, γ int
	for i := nBits - 1; i >= 0; i-- {
		var common [2]int
		for _, v := range diagnostic {
			common[(v>>i)%2]++
		}
		if common[1] > common[0] {
			ε += 1 << i
		} else {
			γ += 1 << i
		}
	}
	return ε * γ
}

func ReadDiagnostic(filename string) ([]int64, int, error) {
	file, err := os.Open(filename)
	if err != nil {
		return nil, 0, err
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)
	diagnostic := make([]int64, 0)
	var nBits int
	for scanner.Scan() {
		nBits = len(scanner.Text())
		binaryNum, err := strconv.ParseInt(scanner.Text(), 2, 0)
		if err != nil {
			return nil, 0, err
		}
		diagnostic = append(diagnostic, binaryNum)
	}
	if err := scanner.Err(); err != nil {
		return nil, 0, err
	}
	return diagnostic, nBits, nil
}
