package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	file, err := os.Open("../input")
	if err != nil {
		fmt.Println("file not found")
		return
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)

	sum := 0
	for scanner.Scan() {
		line := scanner.Text()
		sum += getNumForLine(line)
	}
	fmt.Println(sum)
}

func getNumForLine(line string) int {
	strToNum := map[string]int{
		"1":     1,
		"2":     2,
		"3":     3,
		"4":     4,
		"5":     5,
		"6":     6,
		"7":     7,
		"8":     8,
		"9":     9,
		"one":   1,
		"two":   2,
		"three": 3,
		"four":  4,
		"five":  5,
		"six":   6,
		"seven": 7,
		"eight": 8,
		"nine":  9,
		"zero":  0,
	}

	hasMin := false
	minIdx := 0
	minVal := ""
	for str := range strToNum {
		idx := strings.Index(line, str)
		has := idx != -1
		newMinFound := !hasMin || minIdx >= idx
		if has && newMinFound {
			minIdx = idx
			minVal = str
			hasMin = true
		}
	}
	first, _ := strToNum[line[minIdx:(minIdx+len(minVal))]]

	hasMax := false
	maxIdx := 0
	maxVal := ""
	for str := range strToNum {
		idx := strings.LastIndex(line, str)
		has := idx != -1
		newMaxFound := !hasMax || maxIdx <= idx
		if has && newMaxFound {
			maxIdx = idx
			maxVal = str
			hasMax = true
		}
	}
	second, _ := strToNum[line[maxIdx:(maxIdx+len(maxVal))]]

	return first*10 + second
}
