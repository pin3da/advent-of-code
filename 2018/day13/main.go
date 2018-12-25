package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strings"
)

// Car with directions up:0, left:1, down:2: right:3
type Car struct {
	x, y int
	dir  int
	turn int
	id   int
}

var dx = []int{0, -1, 0, 1}
var dy = []int{-1, 0, 1, 0}

func readInput() ([]string, []Car) {
	sc := bufio.NewScanner(os.Stdin)
	board := []string{}
	cars := []Car{}

	const dirs = "^<v>"
	const rep = "|-|-"

	for i := 0; sc.Scan(); i++ {
		line := []rune(sc.Text())
		for j, v := range line {
			if strings.Contains(dirs, string(v)) {
				dir := strings.Index(dirs, string(v))
				cars = append(cars, Car{j, i, dir, 0, len(cars)})
				line[j] = rune(rep[dir])
			}
		}
		board = append(board, string(line))
	}

	return board, cars
}

func collision(cars []Car) bool {
	for i := range cars {
		for j := i + 1; j < len(cars); j++ {
			if cars[i].x == cars[j].x && cars[i].y == cars[j].y {
				fmt.Println(cars[i].x, cars[i].y)
				return true
			}
		}
	}
	return false
}

func sortCars(cars []Car) {
	sort.Slice(cars, func(i, j int) bool {
		if cars[i].y == cars[j].y {
			return cars[i].x < cars[j].x
		}
		return cars[i].y < cars[j].y
	})
}

type handleCollision func(Car, []Car) []int

func processTick(board []string, cars []Car, getCollisions handleCollision) []Car {
	sortCars(cars)
	forbiden := make(map[int]bool)

	for i, c := range cars {
		if forbiden[c.id] {
			continue
		}

		cars[i].x += dx[c.dir]
		cars[i].y += dy[c.dir]

		pos := board[cars[i].y][cars[i].x]
		if pos == '\\' {
			if c.dir == 3 || c.dir == 1 {
				cars[i].dir = (cars[i].dir + 3) % 4
			} else {
				cars[i].dir = (cars[i].dir + 1) % 4
			}
		}
		if pos == '/' {
			if c.dir == 3 || c.dir == 1 {
				cars[i].dir = (cars[i].dir + 1) % 4
			} else {
				cars[i].dir = (cars[i].dir + 3) % 4
			}
		}

		if pos == '+' {
			if c.turn == 0 {
				cars[i].dir = (cars[i].dir + 1) % 4
			}
			if c.turn == 2 {
				cars[i].dir = (cars[i].dir + 3) % 4
			}
			cars[i].turn = (cars[i].turn + 1) % 3
		}
		for _, f := range getCollisions(cars[i], cars) {
			forbiden[f] = true
		}
	}

	alive := []Car{}
	for _, c := range cars {
		if !forbiden[c.id] {
			alive = append(alive, c)
		}
	}
	return alive
}

func getIDCollisions(current Car, cars []Car) []int {
	forbiden := []int{}
	for _, c := range cars {
		if c.id != current.id && c.x == current.x && c.y == current.y {
			forbiden = append(forbiden, c.id)
		}
	}
	if len(forbiden) > 0 {
		forbiden = append(forbiden, current.id)
	}
	return forbiden
}

func main() {
	board, cars := readInput()
	// part 1
	// for !collision(cars) {
	// 	cars = processTick(board, cars, func(a Car, c []Car) []int {
	// 		return []int{}
	// 	})
	// }

	// part 2
	for len(cars) > 1 {
		cars = processTick(board, cars, getIDCollisions)
	}
	fmt.Println(cars)

}
