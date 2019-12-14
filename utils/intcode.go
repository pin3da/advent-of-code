package utils

import (
	"fmt"
	"io/ioutil"
	"log"
	"strings"
)

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

func getVal(memory []int, idx, mode int) int {
	if mode == 1 {
		return idx
	}
	return memory[idx]
}

func applyFn(memory []int, idx int, inst instruction, fn func(int, int) int) {
	a, b, c := memory[idx+1], memory[idx+2], memory[idx+3]
	if inst.mode3 == 1 {
		panic("output parameters can not be immediate mode")
	}
	memory[c] = fn(getVal(memory, a, inst.mode1), getVal(memory, b, inst.mode2))
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

// RunProgram ...
func RunProgram(memory []int, input, output chan int) {
	defer close(output)
	idx := 0
	for opt := memory[idx]; idx < len(memory); opt = memory[idx] {
		inst := newInstruction(opt)
		switch inst.opcode {
		case 1:
			applyFn(memory, idx, inst, sum)
		case 2:
			applyFn(memory, idx, inst, mult)
		case 3:
			storePos := getVal(memory, idx+1, inst.mode1)
			memory[storePos] = <-input
		case 4:
			val := getVal(memory, memory[idx+1], inst.mode1)
			output <- val
		case 5:
			val := getVal(memory, memory[idx+1], inst.mode1)
			if val != 0 {
				idx = getVal(memory, memory[idx+2], inst.mode2)
				continue
			}
		case 6:
			val := getVal(memory, memory[idx+1], inst.mode1)
			if val == 0 {
				idx = getVal(memory, memory[idx+2], inst.mode2)
				continue
			}
		case 7:
			applyFn(memory, idx, inst, lessThan)
		case 8:
			applyFn(memory, idx, inst, equals)
		case 99:
			return
		default:
			fmt.Println(idx, inst)
			panic("instruction not recognized")
		}
		idx += inst.len
	}
	panic("out of memory")
}

// LoadMemory ...
func LoadMemory(filename string) []int {
	content, err := ioutil.ReadFile(filename)
	if err != nil {
		log.Fatal(err)
	}
	items := strings.Split(string(content), ",")
	ans := []int{}
	for _, i := range items {
		ans = append(ans, Atoi(i))
	}
	return ans
}
