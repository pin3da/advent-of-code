package utils

import (
	"strconv"
	"strings"
)

// Atoi converts string to int and asserts for errors
func Atoi(s string) int {
	ans, err := strconv.Atoi(strings.Trim(s, " "))
	if err != nil {
		panic(err)
	}
	return ans
}

// MaxInt returns the maximum between a and b
func MaxInt(a, b int) int {
	if a >= b {
		return a
	}
	return b
}

// MinInt returns the minimum between a and b
func MinInt(a, b int) int {
	if a >= b {
		return b
	}
	return a
}

// AbsInt retunrs the absolute value of a
func AbsInt(a int) int {
	if a < 0 {
		return -a
	}
	return a
}
