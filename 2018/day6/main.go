package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"

	"github.com/pin3da/advent-of-code/utils"
)

func bfs(grid [][][]int, queue [][]int, size int) {
	dx := []int{-1, 1, 0, 0}
	dy := []int{0, 0, -1, 1}
	for len(queue) > 0 {
		x, y := queue[0][0], queue[0][1]
		queue = queue[1:]
		for i := 0; i < len(dx); i++ {
			nx := x + dx[i]
			ny := y + dy[i]
			if nx >= 0 && nx < size && ny >= 0 && ny < size {
				if grid[nx][ny][1] > grid[x][y][1]+1 {
					grid[nx][ny][0] = grid[x][y][0]
					grid[nx][ny][1] = grid[x][y][1] + 1
					queue = append(queue, []int{nx, ny})
				} else if grid[nx][ny][1] == grid[x][y][1]+1 && grid[nx][ny][0] != grid[x][y][0] {
					grid[nx][ny][0] = size
				}
			}
		}
	}
}

func readInput() (result [][]int) {
	sc := bufio.NewScanner(os.Stdin)
	for sc.Scan() {
		parts := strings.Split(sc.Text(), ",")
		x, y := utils.Atoi(parts[0]), utils.Atoi(parts[1])
		result = append(result, []int{x, y})
	}
	return result
}

func part1(input [][]int) {
	const size = 400
	grid := make([][][]int, size)
	for i := 0; i < size; i++ {
		grid[i] = make([][]int, size)
		for j := 0; j < size; j++ {
			grid[i][j] = []int{0, size + 10}
		}
	}
	queue := [][]int{}
	for i := 1; i <= len(input); i++ {
		x, y := input[i-1][0], input[i-1][1]
		grid[x][y] = []int{i, 0}
		queue = append(queue, []int{x, y})
	}
	bfs(grid, queue, size)
	border := make(map[int]bool)
	for i := 0; i < size; i++ {
		border[grid[i][0][0]] = true
		border[grid[0][i][0]] = true
		border[grid[i][size-1][0]] = true
		border[grid[size-1][i][0]] = true
	}

	frec := make(map[int]int)
	for i := 0; i < size; i++ {
		for j := 0; j < size; j++ {
			if !border[grid[i][j][0]] {
				frec[grid[i][j][0]]++
			}
		}
	}

	best := int(0)
	for i := 0; i < size; i++ {
		best = utils.MaxInt(best, frec[i])
	}
	fmt.Println(best)
}

func dist(a, b []int) int {
	total := int(0)
	for i := range a {
		total += utils.AbsInt(a[i] - b[i])
	}
	return total
}

func part2(input [][]int) {
	const size = 400
	const limit = 10000
	total := int(0)
	for i := 0; i < size; i++ {
		for j := 0; j < size; j++ {
			curDist := int(0)
			for _, p := range input {
				curDist += dist(p, []int{i, j})
			}
			if curDist < limit {
				total++
			}
		}
	}
	fmt.Println(total)
}

func main() {
	input := readInput()
	part1(input)
	part2(input)
}
