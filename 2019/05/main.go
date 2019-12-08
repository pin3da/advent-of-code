package main

import (
	"fmt"

	"github.com/pin3da/advent-of-code/utils"
)

func main() {
	memory := utils.LoadMemory("./a1.in")
	fmt.Println(utils.RunProgram(utils.Copy(memory), []int{1}))
	fmt.Println(utils.RunProgram(utils.Copy(memory), []int{5}))
}
