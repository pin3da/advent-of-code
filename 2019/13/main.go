package main

import (
	"fmt"
	"os"
	"os/exec"
	"sync"

	"github.com/pin3da/advent-of-code/utils"
)

func printBoard(grid [][]int64) {
	c := exec.Command("clear")
	c.Stdout = os.Stdout
	c.Run()

	s := []string{" ", "#", "B", "=", "o"}
	for i := range grid {
		for j := range grid[i] {
			fmt.Print(s[grid[i][j]])
		}
		fmt.Println()
	}
}

func playManual(in, ball utils.PChan, wg *sync.WaitGroup) {
	defer wg.Done()
	for {
		var c string
		fmt.Scan(&c)
		if c == "q" {
			return
		}
		if c == "j" {
			in <- -1
		}
		if c == "k" {
			in <- 0
		}
		if c == "l" {
			in <- 1
		}
	}
}

func playAuto(in, ball utils.PChan, wg *sync.WaitGroup) {
	defer wg.Done()
	pos := int64(21)
	for {
		b, ok := <-ball
		if !ok {
			return
		}
		t := utils.Sign(b - pos)
		pos += t
		in <- t
	}
}

func draw(grid [][]int64, out, ball utils.PChan, wg *sync.WaitGroup) {
	defer wg.Done()
	score := int64(0)
	for it := 0; ; it++ {
		if it > 21*50 {
			// time.Sleep(time.Millisecond * 30)
			// printBoard(grid)
			fmt.Println("Score = ", score)
		}
		y, ok := <-out
		if !ok {
			close(ball)
			break
		}
		x, val := <-out, <-out
		if y == -1 {
			score = val
			continue
		}
		if val == 4 {
			ball <- y
		}
		grid[x][y] = val
	}
}

func main() {
	mem := utils.LoadExtendedMemory(os.Args[1])
	in, out, ball := make(utils.PChan, 2), make(utils.PChan, 2), make(utils.PChan, 2)
	wg := new(sync.WaitGroup)
	wg.Add(1)
	mem[0] = 2 // Free play \m/
	go utils.RunProgram(mem, in, out, wg)

	grid := make([][]int64, 28)
	for i := range grid {
		grid[i] = make([]int64, 50)
	}

	wg.Add(1)
	// go playManual(in, ball, wg)
	go playAuto(in, ball, wg)

	wg.Add(1)
	go draw(grid, out, ball, wg)

	wg.Wait()
}
