package main

import (
	"fmt"
	"sync"

	"github.com/pin3da/advent-of-code/utils"
)

func part1() {
	memory := utils.LoadMemory("./a3.in")
	phase := []int{0, 1, 2, 3, 4}
	best := 0
	for it := 0; it < 120; it++ {
		input := make(chan int)
		last := 0
		wg := new(sync.WaitGroup)
		wg.Add(5)
		for _, ph := range phase {
			output := make(chan int)
			go utils.IntArrToChan([]int{ph, last}, input)
			go utils.RunProgram(utils.Copy(memory), input, output, wg)
			last = <-output
			best = utils.MaxInt(best, last)
		}
		wg.Wait()
		phase = utils.NextPermutation(phase)
	}
	fmt.Println("part 1", best)
}

func part2() {
	memory := utils.LoadMemory("./a3.in")
	phase := []int{5, 6, 7, 8, 9}
	best := 0
	for it := 0; it < 120; it++ {
		bus := make([]chan int, 5)
		for i := 0; i < 5; i++ {
			bus[i] = make(chan int, 2)
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
			tmp := utils.Copy(memory)
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
