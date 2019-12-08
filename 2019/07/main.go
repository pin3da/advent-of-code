package main

import (
	"fmt"

	"github.com/pin3da/advent-of-code/utils"
)

func part1() {
	memory := utils.LoadMemory("./a3.in")
	phase := []int{0, 1, 2, 3, 4}
	best := 0
	for it := 0; it < 120; it++ {
		output := []int{0}
		for _, ph := range phase {
			output = append([]int{ph}, output...)
			output = utils.RunProgram(utils.Copy(memory), output)
			best = utils.MaxInt(best, output[0])
		}
		phase = utils.NextPermutation(phase)
	}
	fmt.Println(best)
}

func main() {
	part1()
}
