package main

import (
	"fmt"
	"sync"

	"github.com/pin3da/advent-of-code/utils"
)

func run(first int64) {
	memory := utils.LoadMemory("./a1.in")
	input := make(utils.PChan, 10)
	output := make(utils.PChan, 10)
	go utils.IntToChan(first, input)

	var wg sync.WaitGroup
	wg.Add(1)
	go utils.RunProgram(utils.Copy64(memory), input, output, &wg)
	wg.Wait()

	for v := range output {
		fmt.Println(v)
	}
}

func main() {
	run(1)
	run(5)
}
