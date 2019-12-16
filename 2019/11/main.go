package main

import (
	"fmt"
	"os"
	"sync"

	"github.com/pin3da/advent-of-code/utils"
)

type pos struct {
	x int
	y int
}

func main() {
	memory := utils.LoadExtendedMemory(os.Args[1])
	in, out := make(utils.PChan, 2), make(utils.PChan, 2)
	wg := new(sync.WaitGroup)
	wg.Add(1)
	go utils.RunProgram(memory, in, out, wg)

	size := 200
	panels := make([][]int64, size)
	for i := range panels {
		panels[i] = make([]int64, size)
	}

	p := pos{size / 2, size / 2}
	panels[p.x][p.y] = 1
	dir := 0 // up, right, down, left
	delta := []pos{{-1, 0}, {0, 1}, {1, 0}, {0, -1}}

	// Process out
	ans := make(map[pos]bool)
	wg.Add(1)
	go func() {
		defer wg.Done()
		for it := 0; ; it++ {
			in <- panels[p.x][p.y]

			v, ok := <-out
			if !ok {
				return
			}
			panels[p.x][p.y] = v
			d := int(<-out)
			if d == 0 {
				d = -1
			}
			dir = (dir + d + 4) % 4
			p.x += delta[dir].x
			p.y += delta[dir].y
			ans[p] = true
		}
	}()

	wg.Wait()
	s := []string{" ", "#", " ", " ", " "}
	for i := range panels {
		for j := range panels[i] {
			fmt.Print(s[panels[i][j]])
		}
		fmt.Println()
	}
	fmt.Println(len(ans))
}
