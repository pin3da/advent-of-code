package main

import (
	"bufio"
	"fmt"
	"os"

	"github.com/pin3da/advent-of-code/utils"
)

type point struct {
	x, y   int
	dx, dy int
}

func readInput() []point {
	sc := bufio.NewScanner(os.Stdin)
	points := []point{}
	for sc.Scan() {
		line := sc.Text()
		p := point{}
		_, err := fmt.Sscanf(line, "position=<%d,%d> velocity=<%d,%d>", &p.x, &p.y, &p.dx, &p.dy)
		if err != nil {
			panic(err)
		}
		points = append(points, p)
	}
	return points
}

func plot(points []point) {
	minX, minY := points[0].x, points[0].y
	maxX, maxY := points[0].x, points[0].y
	for _, p := range points {
		minX = utils.MinInt(minX, p.x)
		minY = utils.MinInt(minY, p.y)
		maxX = utils.MaxInt(maxX, p.x)
		maxY = utils.MaxInt(maxY, p.y)
	}

	maxX = maxX - minX + 1
	maxY = maxY - minY + 1

	board := make([][]rune, maxY)
	for i := range board {
		board[i] = make([]rune, maxX)
		for j := range board[i] {
			board[i][j] = '.'
		}
	}

	for _, p := range points {
		board[p.y-minY][p.x-minX] = '#'
	}

	for i := range board {
		for j := range board[i] {
			fmt.Print(string(board[i][j]))
		}
		fmt.Println()
	}
}

func main() {
	points := readInput()

	for i := 0; i < 10521; i++ { // dimentions are minimal at this point
		for i := range points {
			points[i].x += points[i].dx
			points[i].y += points[i].dy
		}
	}
	plot(points)
}
