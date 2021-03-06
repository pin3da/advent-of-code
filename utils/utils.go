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

// Atoll converts string to int64 and asserts for errors
func Atoll(s string) int64 {
	ans, err := strconv.ParseInt(strings.Trim(s, " "), 10, 64)
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

// MaxInt64 returns the maximum element
func MaxInt64(nums ...int64) int64 {
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

// AbsFloat64 retunrs the absolute value of a
func AbsFloat64(a float64) float64 {
	if a < 0 {
		return -a
	}
	return a
}

// AbsInt64 retunrs the absolute value of a
func AbsInt64(a int64) int64 {
	if a < 0 {
		return -a
	}
	return a
}

// GCD computes the greatest common divisor of a and b
func GCD(a, b int) int {
	a = AbsInt(a)
	b = AbsInt(b)
	for b != 0 {
		t := b
		b = a % b
		a = t
	}
	return a
}

// GCD64 computes the greatest common divisor of a and b
func GCD64(a, b int64) int64 {
	a = AbsInt64(a)
	b = AbsInt64(b)
	for b != 0 {
		t := b
		b = a % b
		a = t
	}
	return a
}

// LCM64 ...
func LCM64(a, b int64) int64 {
	return a * b / GCD64(a, b)
}

// Copy ...
func Copy(a []int) []int {
	b := make([]int, len(a))
	copy(b, a)
	return b
}

// Copy64 ...
func Copy64(a []int64) []int64 {
	b := make([]int64, len(a))
	copy(b, a)
	return b
}

// Reverse ...
func Reverse(a []int64, i int) []int64 {
	for j := len(a) - 1; i < j; i++ {
		a[i], a[j] = a[j], a[i]
		j--
	}
	return a
}

// NextPermutation ...
func NextPermutation(a []int64) []int64 {
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

// IntToChan is a gorutine that adds a single int to a channel
func IntToChan(val int64, c chan int64) {
	c <- val
}

// IntArrToChan is a gorutine that adds several ints to a channel
func IntArrToChan(arr []int64, c chan int64) {
	for _, v := range arr {
		c <- v
	}
}

// Sign ...
func Sign(f int64) int64 {
	if f < 0 {
		return -1
	}
	if f > 0 {
		return 1
	}
	return 0
}
