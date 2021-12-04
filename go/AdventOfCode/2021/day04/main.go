package main

import (
	"bytes"
	"fmt"
	"io/ioutil"
	"log"
	"strconv"
)

type Board struct {
	slots  [5][5]int
	marked [5][5]bool
}

func main() {
	picks, boards, err := ReadBingoData("input")
	if err != nil {
		log.Fatalln(err)
	}
	fmt.Println("1:", SimulateBingo(picks, boards))
}

func CheckWinner(boards []Board) int {
	for bi, b := range boards {
		for i := 0; i < 5; i++ {
			validRow := true
			validCol := true
			for j := 0; j < 5; j++ {
				if !b.marked[i][j] {
					validRow = false
				}
				if !b.marked[j][i] {
					validCol = false
				}
			}
			if validRow || validCol {
				return bi
			}
		}
	}
	return -1
}

func BoardScore(board Board, lastDraw int) int {
	score := 0
	for i := 0; i < 25; i++ {
		if !board.marked[i/5][i%5] {
			score += board.slots[i/5][i%5]
		}
	}
	return score * lastDraw
}

func SimulateBingo(picks []int, boards []Board) int {
	for _, p := range picks {
		for bi := range boards {
			for i := 0; i < 25; i++ {
				if boards[bi].slots[i/5][i%5] == p {
					boards[bi].marked[i/5][i%5] = true
				}
			}
		}
		if winner := CheckWinner(boards); winner != -1 {
			return BoardScore(boards[winner], p)
		}
	}
	panic("Unreachable")
}

func ReadBingoData(filename string) ([]int, []Board, error) {
	content, err := ioutil.ReadFile(filename)
	if err != nil {
		return nil, nil, err
	}
	lines := bytes.Split(content, []byte("\n\n"))
	var picks []int
	for _, v := range bytes.Split(lines[0], []byte(",")) {
		num, err := strconv.Atoi(string(v))
		if err != nil {
			return nil, nil, err
		}
		picks = append(picks, num)
	}
	var boards []Board
	for _, v := range lines[1:] {
		var board Board
		flds := bytes.Fields(v)
		for i := 0; i < 5*5; i++ {
			num, err := strconv.Atoi(string(flds[i]))
			if err != nil {
				return nil, nil, err
			}
			board.slots[i/5][i%5] = num
		}
		boards = append(boards, board)
	}
	return picks, boards, err
}
