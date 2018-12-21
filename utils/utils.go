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

// MaxInt returns the maximum element
func MaxInt(nums ...int) int {
	ans := nums[0]
	for _, val := range nums {
		if ans < val {
			ans = val
		}
	}
	return ans
}

// MinInt returns the minimum between a and b
func MinInt(nums ...int) int {
	ans := nums[0]
	for _, val := range nums {
		if ans > val {
			ans = val
		}
	}
	return ans
}

// AbsInt retunrs the absolute value of a
func AbsInt(a int) int {
	if a < 0 {
		return -a
	}
	return a
}
