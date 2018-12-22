package main

import (
	"fmt"
	"strings"
)

func cmp(a, b []int) bool {
	if len(a) != len(b) {
		return false
	}
	for i := range a {
		if a[i] != b[i] {
			return false
		}
	}
	return true
}

func main() {
	recipes := []int{3, 7}
	a := 0
	b := 1
	for iters := 30000000; iters >= 0; iters-- {
		next := recipes[a] + recipes[b]
		if next >= 10 {
			recipes = append(recipes, next/10)
		}
		recipes = append(recipes, next%10)
		a = (a + 1 + recipes[a]) % len(recipes)
		b = (b + 1 + recipes[b]) % len(recipes)
	}

	input := []int{9, 5, 18, 2018, 74501}
	for _, in := range input {
		fmt.Println(strings.Replace(fmt.Sprint(recipes[in:in+10]), " ", "", -1))
	}

	target := []int{0, 7, 4, 5, 0, 1}
	for i := 0; i+len(target) < len(recipes); i++ {
		if cmp(recipes[i:i+len(target)], target) {
			fmt.Println("starts at", i)
			break
		}
	}
}
