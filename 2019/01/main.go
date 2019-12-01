package main

import (
	"bufio"
	"os"
	"strconv"
)

func readInput() []int64 {
	scanner := bufio.NewScanner(os.Stdin)
	result := []int64{}

	for scanner.Scan() {
		tmp, err := strconv.Atoi(scanner.Text())
		if err != nil {
			panic(err)
		}
		result = append(result, int64(tmp))
	}
	return result
}

func part1(input []int64) int64 {
	total := int64(0)
	for _, val := range input {
		total += val/3 - 2
	}
	return total
}

func part2(input []int64) int64 {
	total := int64(0)
	for _, val := range input {
		for val > 0 {
			val = val/3 - 2
			if val > 0 {
				total += val
			}
		}
	}
	return total
}

func main() {
	input := readInput()
	println(part1(input))
	println(part2(input))
}
