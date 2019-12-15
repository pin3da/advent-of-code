package main

import (
	"fmt"
	"os"
	"sync"

	"github.com/pin3da/advent-of-code/utils"
)

func main() {
	memory := utils.LoadMemory(os.Args[1])
	for i := 0; i < 1000000; i++ {
		memory = append(memory, 0)
	}
	in := make(utils.PChan, 100)
	out := make(utils.PChan, 100)
	wg := new(sync.WaitGroup)
	wg.Add(2)
	go func() {
		tmp := int64(1)
		fmt.Println("Input", tmp)
		in <- tmp
	}()

	go utils.RunProgram(memory, in, out, wg)
	go func() {
		defer wg.Done()
		for i := range out {
			fmt.Print(i, ",")
		}
		fmt.Println()
	}()
	wg.Wait()
}
