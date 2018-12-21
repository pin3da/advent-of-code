package main

import (
	"fmt"

	"github.com/pin3da/advent-of-code/utils"
)

type Marble struct {
	val   int
	left  *Marble
	right *Marble
}

func play(players, lastValue int) int {
	points := make([]int, players)
	marble := &Marble{val: 0}
	marble.left = marble
	marble.right = marble
	player := 0

	for win, cur := 0, 1; cur != lastValue; cur++ {
		win = 0
		tmp := &Marble{val: cur}
		if (cur % 23) == 0 {
			win += cur
			for i := 0; i < 7; i++ {
				marble = marble.left
			}
			win += marble.val
			marble.left.right = marble.right
			marble.right.left = marble.left
			marble = marble.right
		} else {
			tmp.left = marble.right
			tmp.right = marble.right.right
			tmp.left.right = tmp
			tmp.right.left = tmp
			marble = tmp
		}
		points[player] += win
		player = (player + 1) % players
	}
	return utils.MaxInt(points...)
}

func main() {
	input := [][]int{
		[]int{9, 32},
		[]int{10, 1618},
		[]int{13, 7999},
		[]int{435, 71184},       // Part 1
		[]int{435, 71184 * 100}, // Part 2
	}

	for _, in := range input {
		fmt.Println(play(in[0], in[1]))
	}
}
