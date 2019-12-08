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

// Copy ...
func Copy(a []int) []int {
	b := make([]int, len(a))
	copy(b, a)
	return b
}

// Reverse ...
func Reverse(a []int, i int) []int {
	for j := len(a) - 1; i < j; i++ {
		a[i], a[j] = a[j], a[i]
		j--
	}
	return a
}

// NextPermutation ...
func NextPermutation(a []int) []int {
	i := len(a) - 1
	for ; i > 0 && a[i] <= a[i-1]; i-- {
	}
	if i == 0 {
		return a
	}

	j := len(a) - 1
	for ; j >= i && a[j] <= a[i-1]; j-- {
	}

	a[j], a[i-1] = a[i-1], a[j]
	a = Reverse(a, i)
	return a

}
