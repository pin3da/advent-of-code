package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"

	"github.com/pin3da/advent-of-code/utils"
)

func readInput() ([]string, []string) {
	scanner := bufio.NewScanner(os.Stdin)
	scanner.Scan()
	first := strings.Split(scanner.Text(), ",")
	scanner.Scan()
	second := strings.Split(scanner.Text(), ",")
	return first, second
}

type pos [2]int

func parseStep(step string) (int, int) {
	dir := -1
	switch step[0:1] {
	case "U":
		dir = 0
	case "R":
		dir = 1
	case "D":
		dir = 2
	case "L":
		dir = 3
	}
	if dir == -1 {
		panic("Falied to get dir")
	}
	len, err := strconv.Atoi(step[1:])
	if err != nil {
		panic(err)
	}
	return dir, len
}

func buildPath(seq []string) map[pos]int {
	// up, right, down, left
	deltaX := []int{-1, 0, 1, 0}
	deltaY := []int{0, -1, 0, 1}

	ans := map[pos]int{}
	curPos := pos{0, 0}
	curIdx := 0
	for _, step := range seq {
		dir, len := parseStep(step)
		for l := 0; l < len; l++ {
			curIdx++
			curPos[0] += deltaX[dir]
			curPos[1] += deltaY[dir]
			if _, seen := ans[curPos]; !seen {
				ans[curPos] = curIdx
			}
		}
	}
	return ans
}

func dist(p pos) int {
	return utils.AbsInt(p[0]) + utils.AbsInt(p[1])
}

func solve(first, second []string) (int, int) {
	path1 := buildPath(first)
	path2 := buildPath(second)
	bestPart1 := pos{1000000, 1000000}
	bestPart2 := 10000000
	for k := range path1 {
		if _, ok := path2[k]; ok {
			if dist(k) < dist(bestPart1) {
				bestPart1 = k
			}
			if sum := path1[k] + path2[k]; sum < bestPart2 {
				bestPart2 = sum
			}
		}
	}
	return dist(bestPart1), bestPart2
}

func main() {
	first, second := readInput()
	fmt.Println(solve(first, second))
}
