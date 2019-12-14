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
		input := make(chan int)
		last := 0
		for _, ph := range phase {
			output := make(chan int)
			go utils.IntArrToChan([]int{ph, last}, input)
			go utils.RunProgram(utils.Copy(memory), input, output)
			last = <-output
			best = utils.MaxInt(best, last)
		}
		phase = utils.NextPermutation(phase)
	}
	fmt.Println(best)
}

func main() {
	part1()
}
