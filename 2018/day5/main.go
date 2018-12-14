package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func isMatch(a, b rune) bool {
	if strings.ToLower(string(a)) != strings.ToLower(string(b)) {
		return false
	}
	if a == b {
		return false
	}
	return true
}

func process(text string, forb rune) int {
	total := len(text)
	stack := []rune{}
	for _, r := range text {
		if strings.ToLower(string(r)) == string(forb) {
			total--
			continue
		}
		if len(stack) > 0 && isMatch(stack[len(stack)-1], r) {
			stack = stack[:len(stack)-1]
			total -= 2
			continue
		}
		stack = append(stack, r)
	}
	return total
}

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	scanner.Scan()
	text := scanner.Text()
	best := process(text, '$')
	fmt.Println(best)
	for i := 'a'; i <= 'z'; i++ {
		if cur := process(text, i); best > cur {
			best = cur
		}
	}
	fmt.Println(best)
}
