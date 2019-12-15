package main

import (
	"fmt"
	"sync"

	"github.com/pin3da/advent-of-code/utils"
)

func part1() {
	memory := utils.LoadMemory("./a3.in")
	phase := []int64{0, 1, 2, 3, 4}
	best := int64(0)
	for it := 0; it < 120; it++ {
		input := make(utils.PChan)
		last := int64(0)
		wg := new(sync.WaitGroup)
		wg.Add(5)
		for _, ph := range phase {
			output := make(utils.PChan)
			go utils.IntArrToChan([]int64{ph, last}, input)
			go utils.RunProgram(utils.Copy64(memory), input, output, wg)
			last = <-output
			best = utils.MaxInt64(best, last)
		}
		wg.Wait()
		phase = utils.NextPermutation(phase)
	}
	fmt.Println("part 1", best)
}

func part2() {
	memory := utils.LoadMemory("./a3.in")
	phase := []int64{5, 6, 7, 8, 9}
	best := int64(0)
	for it := 0; it < 120; it++ {
		bus := make([]utils.PChan, 5)
		for i := 0; i < 5; i++ {
			bus[i] = make(utils.PChan, 2)
		}
		go func() {
			for i := 0; i < 5; i++ {
				bus[i] <- phase[i]
			}
			bus[0] <- 0
		}()

		wg := new(sync.WaitGroup)
		for i := 0; i < 5; i++ {
			wg.Add(1)
			tmp := utils.Copy64(memory)
			go utils.RunProgram(tmp, bus[i], bus[(i+1)%5], wg)
		}
		wg.Wait()
		cur := <-bus[0]
		if cur > best {
			best = cur
		}
		phase = utils.NextPermutation(phase)
	}
	fmt.Println("Part 2", best)
}

func main() {
	part1()
	part2()
}
