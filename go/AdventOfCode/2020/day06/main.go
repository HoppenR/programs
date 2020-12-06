package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"strings"
)

func main() {
	forms, err := ReadPassports("input")
	if err != nil {
		log.Fatalln(err)
	}
	fmt.Println("1:", SumCounts(forms, 1))
	fmt.Println("2:", SumCounts(forms, 2))
}

func CountGroupScore(f string, ruleset int) (cnt int) {
	if ruleset == 1 {
		f = strings.ReplaceAll(f, "\n", "")
		answers := make(map[rune]bool)
		for _, c := range f {
			if _, ok := answers[c]; !ok {
				cnt++
				answers[c] = true
			}
		}
	} else if ruleset == 2 {
		f = strings.TrimSuffix(f, "\n")
		respondents := strings.Count(f, "\n") + 1
		for len(f) > 0 {
			prevLen := len(f)
			f = strings.ReplaceAll(f, string(f[0]), "")
			if prevLen - len(f) == respondents {
				cnt++
			}
		}
	}
	return
}

func SumCounts(forms []string, ruleset int) (cnt int) {
	for _, f := range forms {
		cnt += CountGroupScore(f, ruleset)
	}
	return
}

func ReadPassports(filename string) ([]string, error) {
	content, err := ioutil.ReadFile(filename)
	if err != nil {
		return nil, err
	}
	forms := strings.Split(string(content), "\n\n")
	return forms, nil
}
