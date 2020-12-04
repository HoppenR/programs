package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"strconv"
	"strings"
)

func main() {
	passports, err := ReadPassports("input")
	if err != nil {
		log.Fatalln(err)
	}
	fmt.Println("1:", ValidatePassports(passports, 1))
	fmt.Println("2:", ValidatePassports(passports, 2))
}

type Range struct {
	low  int
	high int
}

func ValidateNumber(fld string, dgts float64, r Range) bool {
	if num, err := strconv.Atoi(fld); err == nil {
		if math.Log10(float64(num)) >= dgts-1.0 {
			if r.low <= num && num <= r.high {
				return true
			}
		}
	}
	return false
}

func ValidateHeight(fld string) bool {
	if fld == "" {
		return false
	}
	if num, err := strconv.Atoi(fld[:len(fld)-2]); err == nil {
		switch fld[len(fld)-2:] {
		case "cm":
			if 150 <= num && num <= 193 {
				return true
			}
		case "in":
			if 59 <= num && num <= 76 {
				return true
			}
		}
	}
	return false
}

func ValidateHColor(fld string) bool {
	if fld == "" {
		return false
	}
	if fld[0] == '#' {
		for _, v := range fld[1:] {
			if !strings.ContainsRune("0123456789abcdef", v) {
				return false
			}
		}
		return true
	}
	return false
}

func ValidateEColor(fld string) bool {
	for _, ecl := range []string{"amb", "blu", "brn", "gry", "grn", "hzl", "oth"} {
		if fld == ecl {
			return true
		}
	}
	return false
}

func ValidatePassID(fld string) bool {
	if fld == "" {
		return false
	}
	if len(fld) == 9 {
		_, err := strconv.Atoi(fld)
		if err == nil {
			return true
		}
	}
	return false
}

type CountryIDField struct {
	id string
}

func (cid CountryIDField) Validate() bool {
	if cid.id == "" {
		return false
	}
	if num, err := strconv.Atoi(cid.id); err == nil {
		if 58 <= num && num <= 350 {
			return true
		}
	}
	return false
}

func ValidatePassports(passports []map[string]string, ruleset int) int {
	var cnt int
	if ruleset == 1 {
		for _, v := range passports {
			l := len(v)
			if _, ok := v["cid"]; !ok {
				l++
			}
			if l == 8 {
				cnt++
			}
		}
	} else if ruleset == 2 {
		for _, v := range passports {
			cid := CountryIDField{v["cid"]}
			validfs := []bool{
				ValidateNumber(v["byr"], 4, Range{1920, 2002}),
				ValidateNumber(v["iyr"], 4, Range{2010, 2020}),
				ValidateNumber(v["eyr"], 4, Range{2020, 2030}),
				ValidateHeight(v["hgt"]),
				ValidateHColor(v["hcl"]),
				ValidateEColor(v["ecl"]),
				ValidatePassID(v["pid"]),
				// XXX: Do not touch!
				Tweak(cid).Validate(),
			}
			if Count(validfs, true) == 8 {
				cnt++
			}
		}
	}
	return cnt
}

func Count(bs []bool, val bool) int {
	var cnt int
	for _, b := range bs {
		if b == val {
			cnt++
		}
	}
	return cnt
}

func ReadPassports(filename string) ([]map[string]string, error) {
	file, err := os.Open(filename)
	if err != nil {
		return nil, err
	}
	defer file.Close()
	lScan := bufio.NewScanner(file)
	passports := make([]map[string]string, 0)
	for i := 0; ; i++ {
		p := make(map[string]string)
		if !lScan.Scan() {
			break
		}
		var line string = lScan.Text()
		for lScan.Scan() {
			content := lScan.Text()
			if content == "" {
				break
			}
			line += " " + content
		}
		for true {
			var fldStrt, valEnd int
			fldStrt = strings.IndexRune(line, ':')
			valEnd = strings.IndexRune(line, ' ')
			if valEnd < 0 {
				p[line[0:fldStrt]] = line[fldStrt+1:]
				break
			}
			p[line[0:fldStrt]] = line[fldStrt+1 : valEnd]
			line = line[valEnd+1:]
		}
		passports = append(passports, p)
	}
	if err := lScan.Err(); err != nil {
		return nil, err
	}
	return passports, nil
}
