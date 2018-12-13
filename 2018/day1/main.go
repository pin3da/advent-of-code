package main

import (
	"bufio"
	"fmt"
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

func part1(input []int64) {
	total := int64(0)
	for _, val := range input {
		total += val
	}
	fmt.Println(total)
}

func part2(input []int64) {
	frec := map[int64]int{}
	total := int64(0)
	frec[0] = 1
	for {
		for _, val := range input {
			total += val
			frec[total]++
			if frec[total] == 2 {
				fmt.Println(total)
				return
			}
		}
	}
}

func main() {
	input := readInput()
	part1(input)
	part2(input)
}
