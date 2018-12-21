package main

import (
	"fmt"

	"github.com/pin3da/advent-of-code/utils"
)

func main() {
	sc := utils.NewReader()
	serial := sc.Int()
	const MAX_SIZE = 301

	grid := make([][]int, MAX_SIZE)
	for i := range grid {
		grid[i] = make([]int, MAX_SIZE)
	}

	for j := 1; j < MAX_SIZE; j++ {
		for i := 1; i < MAX_SIZE; i++ {
			rack := i + 10
			power := rack*j + serial
			tmp := rack * power
			grid[j][i] = ((tmp / 100) % 10) - 5
		}
	}

	for i := 1; i < MAX_SIZE; i++ {
		for j := 1; j < MAX_SIZE; j++ {
			grid[i][j] = grid[i][j] + grid[i-1][j] + grid[i][j-1] - grid[i-1][j-1]
		}
	}

	best := 0
	ans := []int{}
	// size := 3  // Part 1
	for size := 1; size <= MAX_SIZE; size++ { // Part 2
		for i := 1; i+size < MAX_SIZE; i++ {
			for j := 1; j+size < MAX_SIZE; j++ {
				cur := grid[i+size-1][j+size-1] - grid[i-1][j+size-1] - grid[i+size-1][j-1] + grid[i-1][j-1]
				if cur > best {
					ans = []int{j, i, size}
					best = cur
				}
			}
		}
	}

	fmt.Println(best, ans)
}
