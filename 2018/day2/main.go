package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
)

func removeLetter(word string, forb int) string {
	result := ""
	for i, c := range word {
		if i != forb {
			result += string(c)
		}
	}
	return result
}

func readInput() []string {
	scanner := bufio.NewScanner(os.Stdin)
	words := []string{}
	for scanner.Scan() {
		line := scanner.Text()
		words = append(words, line)
	}
	return words
}

func part1(words []string) {
	twoTimes := 0
	threeTimes := 0
	for _, word := range words {
		frec := make(map[rune]int)
		for _, c := range word {
			frec[c]++
		}
		twoOK := false
		threeOK := false
		for _, val := range frec {
			if val == 2 {
				twoOK = true
			}
			if val == 3 {
				threeOK = true
			}
		}
		if twoOK {
			twoTimes++
		}
		if threeOK {
			threeTimes++
		}
	}
	fmt.Println(twoTimes * threeTimes)
}

func part2(words []string) {
	for i := 0; i < len(words[0]); i++ {
		curWords := []string{}
		for _, w := range words {
			curWords = append(curWords, removeLetter(w, i))
		}
		sort.Strings(curWords)

		for i := 1; i < len(curWords); i++ {
			if curWords[i-1] == curWords[i] {
				fmt.Println(curWords[i])
			}
		}
	}
}

func main() {
	input := readInput()
	part1(input)
	part2(input)
}
