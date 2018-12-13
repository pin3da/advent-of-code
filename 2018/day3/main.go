package main

import (
	"bufio"
	"fmt"
	"os"
)

type claim struct {
	ID            string
	left, upper   int
	width, height int
}

func parseClaim(line string) claim {
	result := claim{}
	n, err := fmt.Sscanf(line, "#%s @ %d,%d: %dx%d", &result.ID, &result.left, &result.upper, &result.width, &result.height)
	if n != 5 || err != nil {
		panic("error reading the input")
	}
	return result
}

func readInput() []claim {
	result := []claim{}
	scanner := bufio.NewScanner(os.Stdin)

	for scanner.Scan() {
		result = append(result, parseClaim(scanner.Text()))
	}

	return result
}

func part1(claims []claim) [][]int {
	const W = 2000
	grid := make([][]int, W)
	for i := 0; i < W; i++ {
		grid[i] = make([]int, W)
	}

	for _, c := range claims {
		for i := c.left; i < c.left+c.width; i++ {
			for j := c.upper; j < c.upper+c.height; j++ {
				grid[i][j]++
			}
		}
	}

	total := 0
	for i := 0; i < W; i++ {
		for j := 0; j < W; j++ {
			if grid[i][j] > 1 {
				total++
			}
		}
	}
	fmt.Println(total)
	return grid
}

func part2(claims []claim, grid [][]int) {
	for _, c := range claims {
		ok := true
		for i := c.left; i < c.left+c.width; i++ {
			for j := c.upper; j < c.upper+c.height; j++ {
				if grid[i][j] != 1 {
					ok = false
				}
			}
		}
		if ok {
			fmt.Println(c.ID)
		}
	}
}

func main() {
	input := readInput()
	grid := part1(input)
	part2(input, grid)
}
