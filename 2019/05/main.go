package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"strings"

	"github.com/pin3da/advent-of-code/utils"
)

func loadMemory() []int {
	content, err := ioutil.ReadFile("./a1.in")
	if err != nil {
		log.Fatal(err)
	}
	items := strings.Split(string(content), ",")
	ans := []int{}
	for _, i := range items {
		ans = append(ans, utils.Atoi(i))
	}
	return ans
}

type instruction struct {
	opcode int
	mode1  int
	mode2  int
	mode3  int
	len    int
}

func newInstruction(op int) instruction {
	opcode := op % 100
	op /= 100
	mode1 := op % 10
	op /= 10
	mode2 := op % 10
	op /= 10
	mode3 := op % 10
	if opcode == 99 {
		return instruction{opcode, mode1, mode2, mode3, -1}
	}
	len := []int{0, 4, 4, 2, 2, 3, 3, 4, 4}
	return instruction{opcode, mode1, mode2, mode3, len[opcode]}
}

func getVal(input []int, idx, mode int) int {
	if mode == 1 {
		return idx
	}
	return input[idx]
}

func applyFn(input []int, idx int, inst instruction, fn func(int, int) int) {
	a, b, c := input[idx+1], input[idx+2], input[idx+3]
	if inst.mode3 == 1 {
		panic("output parameters can not be immediate mode")
	}
	input[c] = fn(getVal(input, a, inst.mode1), getVal(input, b, inst.mode2))
}

func mult(a, b int) int {
	return a * b
}

func sum(a, b int) int {
	return a + b
}

func lessThan(a, b int) int {
	if a < b {
		return 1
	}
	return 0
}

func equals(a, b int) int {
	if a == b {
		return 1
	}
	return 0
}

func runProgram(input []int) bool {
	idx := 0
	for opt := input[idx]; idx < len(input); opt = input[idx] {
		inst := newInstruction(opt)
		switch inst.opcode {
		case 1:
			applyFn(input, idx, inst, sum)
		case 2:
			applyFn(input, idx, inst, mult)
		case 3:
			storePos := getVal(input, idx+1, inst.mode1)
			fmt.Println("insert number, will store at", storePos)
			fmt.Scan(&input[storePos])
		case 4:
			val := getVal(input, input[idx+1], inst.mode1)
			fmt.Printf("diagnostic: %d\n", val)
		case 5:
			val := getVal(input, input[idx+1], inst.mode1)
			if val != 0 {
				idx = getVal(input, input[idx+2], inst.mode2)
				continue
			}
		case 6:
			val := getVal(input, input[idx+1], inst.mode1)
			if val == 0 {
				idx = getVal(input, input[idx+2], inst.mode2)
				continue
			}
		case 7:
			applyFn(input, idx, inst, lessThan)
		case 8:
			applyFn(input, idx, inst, equals)
		case 99:
			return true
		default:
			fmt.Println(idx, inst)
			panic("instruction not recognized")
		}
		idx += inst.len
	}
	return false
}

func main() {
	fmt.Println(runProgram(loadMemory()))
}
