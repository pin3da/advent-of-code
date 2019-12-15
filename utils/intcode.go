package utils

import (
	"fmt"
	"io/ioutil"
	"log"
	"strings"
	"sync"
)

type instruction struct {
	opcode int64
	mode1  int64
	mode2  int64
	mode3  int64
	len    int64
}

type pos struct {
	idx int64
	rel int64
}

func newInstruction(op int64) instruction {
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
	len := []int64{0, 4, 4, 2, 2, 3, 3, 4, 4, 2}
	return instruction{opcode, mode1, mode2, mode3, len[opcode]}
}

func getPos(p pos, mode int64) int64 {
	if mode == 2 {
		return p.idx + p.rel
	}
	return p.idx
}

func getVal(memory []int64, p pos, mode int64) int64 {
	if mode == 1 {
		return p.idx
	}
	return memory[getPos(p, mode)]
}

func applyFn(memory []int64, p pos, inst instruction, fn func(int64, int64) int64) {
	a, b, c := memory[p.idx+1], memory[p.idx+2], memory[p.idx+3]
	if inst.mode3 == 1 {
		panic("output parameters can not be immediate mode")
	}
	storePos := getPos(pos{c, p.rel}, inst.mode3)
	memory[storePos] = fn(
		getVal(memory, pos{a, p.rel}, inst.mode1),
		getVal(memory, pos{b, p.rel}, inst.mode2),
	)
}

func mult(a, b int64) int64 {
	return a * b
}

func sum(a, b int64) int64 {
	return a + b
}

func lessThan(a, b int64) int64 {
	if a < b {
		return 1
	}
	return 0
}

func equals(a, b int64) int64 {
	if a == b {
		return 1
	}
	return 0
}

// PChan is a program channel
type PChan chan int64

// RunProgram ...
func RunProgram(memory []int64, input, output PChan, wg *sync.WaitGroup) {
	defer wg.Done()
	defer close(output)

	p := pos{0, 0}
	for opt := memory[p.idx]; p.idx < int64(len(memory)); opt = memory[p.idx] {
		inst := newInstruction(opt)
		switch inst.opcode {
		case 1:
			applyFn(memory, p, inst, sum)
		case 2:
			applyFn(memory, p, inst, mult)
		case 3:
			storePos := getPos(pos{memory[p.idx+1], p.rel}, inst.mode1)
			memory[storePos] = <-input
		case 4:
			val := getVal(memory, pos{memory[p.idx+1], p.rel}, inst.mode1)
			output <- val
		case 5:
			val := getVal(memory, pos{memory[p.idx+1], p.rel}, inst.mode1)
			if val != 0 {
				p.idx = getVal(memory, pos{memory[p.idx+2], p.rel}, inst.mode2)
				continue
			}
		case 6:
			val := getVal(memory, pos{memory[p.idx+1], p.rel}, inst.mode1)
			if val == 0 {
				p.idx = getVal(memory, pos{memory[p.idx+2], p.rel}, inst.mode2)
				continue
			}
		case 7:
			applyFn(memory, p, inst, lessThan)
		case 8:
			applyFn(memory, p, inst, equals)
		case 9:
			p.rel += getVal(memory, pos{memory[p.idx+1], p.rel}, inst.mode1)
		case 99:
			return
		default:
			fmt.Println(p.idx, inst)
			panic("instruction not recognized")
		}
		p.idx += inst.len
	}
	panic("out of memory")
}

// LoadMemory ...
func LoadMemory(filename string) []int64 {
	content, err := ioutil.ReadFile(filename)
	if err != nil {
		log.Fatal(err)
	}
	items := strings.Split(string(content), ",")
	ans := []int64{}
	for _, i := range items {
		ans = append(ans, Atoll(i))
	}
	return ans
}
