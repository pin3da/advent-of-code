package main

import (
	"fmt"

	"github.com/pin3da/advent-of-code/utils"
)

func main() {
	memory := utils.LoadMemory("./a1.in")
	input := make(chan int)
	output := make(chan int)
	go utils.IntToChan(1, input)
	go utils.RunProgram(utils.Copy(memory), input, output)
	fmt.Println("Result part 1: ")
	for v := range output {
		fmt.Println(v)
	}
	output = make(chan int)
	go utils.IntToChan(5, input)
	go utils.RunProgram(utils.Copy(memory), input, output)
	fmt.Println("Result part 2: ")
	for v := range output {
		fmt.Println(v)
	}
}
