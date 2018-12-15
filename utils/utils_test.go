package utils_test

import (
	"testing"

	"github.com/pin3da/advent-of-code/utils"
)

func TestAtoi(t *testing.T) {
	data := []struct {
		in       string
		expected int
	}{
		{"10", 10},
		{" 11", 11},
		{"-11", -11},
	}

	for _, d := range data {
		res := utils.Atoi(d.in)
		if res != d.expected {
			t.Errorf("Error parsing %s, expecting %d", d.in, d.expected)
		}
	}
}

func TestMax(t *testing.T) {
	type Data struct {
		in       []int
		expected int
	}
	data := []Data{
		{[]int{10, 100}, 100},
		{[]int{-10, -100}, -10},
	}
	for _, d := range data {
		res := utils.MaxInt(d.in[0], d.in[1])
		if res != d.expected {
			t.Errorf("%d is not Max, expecting %d", res, d.expected)
		}
	}
}
