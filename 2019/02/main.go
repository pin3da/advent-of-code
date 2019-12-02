package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func readInput() []int64 {
	scanner := bufio.NewScanner(os.Stdin)
	result := []int64{}

	for scanner.Scan() {
		line := strings.Split(scanner.Text(), ",")
		for _, val := range line {
			tmp, err := strconv.Atoi(val)
			if err != nil {
				panic(err)
			}
			result = append(result, int64(tmp))
		}
	}
	return result
}

func applyFn(input []int64, idx int, fn func(int64, int64) int64) {
	a, b, c := input[idx+1], input[idx+2], input[idx+3]
	input[c] = fn(input[a], input[b])
}

func mult(a, b int64) int64 {
	return a * b
}

func sum(a, b int64) int64 {
	return a + b
}

func runProgram(input []int64) bool {
	idx := 0
	for opt := input[idx]; idx < len(input); opt = input[idx] {
		switch opt {
		case 1:
			applyFn(input, idx, sum)
			idx += 4
		case 2:
			applyFn(input, idx, mult)
			idx += 4
		case 99:
			return true
		default:
			return false
		}
	}
	return false
}

func part1(input []int64) int64 {
	input[1] = 12
	input[2] = 2
	runProgram(input)
	return input[0]
}

func copyArray(input []int64) []int64 {
	ans := make([]int64, len(input))
	copy(ans, input)
	return ans
}

func part2(input []int64) int64 {
	for noun := 0; noun < 100; noun++ {
		for verb := 0; verb < 100; verb++ {
			run := copyArray(input)
			run[1] = int64(noun)
			run[2] = int64(verb)
			if runProgram(run) && run[0] == int64(19690720) {
				return int64(noun*100 + verb)
			}
		}
	}
	panic("answer not found")
}

func main() {
	input := readInput()
	fmt.Println(part1(copyArray(input)))
	fmt.Println(part2(copyArray(input)))
}
