package main

import (
	"fmt"
	"strconv"
)

func get(s string, idx int) byte {
	if idx < 0 || idx >= len(s) {
		return 'a'
	}
	return s[idx]
}

func isValid(n int) bool {
	s := strconv.Itoa(n)
	if len(s) != 6 {
		return false
	}
	same := false
	for i := 1; i < len(s); i++ {
		if s[i-1] > s[i] {
			return false
		}
		if (s[i-1] == s[i]) && (get(s, i-2) != s[i]) && (get(s, i+1) != s[i]) {
			same = true
		}

	}
	return same
}

func main() {
	lo, hi := 147981, 691423
	ans := 0
	for n := lo; n <= hi; n++ {
		if isValid(n) {
			ans++
		}
	}
	fmt.Println(ans)
}
