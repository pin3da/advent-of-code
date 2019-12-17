package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"os"
	"strings"

	"github.com/pin3da/advent-of-code/utils"
)

type vect [3]int64

type planet struct {
	pos, vel vect
}

func energy(planets []planet) int64 {
	total := int64(0)
	for _, p := range planets {
		pot := int64(0)
		kin := int64(0)
		for i := 0; i < 3; i++ {
			pot += utils.AbsInt64(p.pos[i])
			kin += utils.AbsInt64(p.vel[i])
		}
		total += pot * kin
	}
	return total
}

func step(planets []planet, idx int) []planet {
	for a := range planets {
		for b := range planets {
			planets[a].vel[idx] += utils.Sign(planets[b].pos[idx] - planets[a].pos[idx])

		}
	}
	for a := range planets {
		planets[a].pos[idx] += planets[a].vel[idx]
	}
	return planets
}

func part1(planets []planet) {
	for time := 0; time < 1000; time++ {
		for i := 0; i < 3; i++ {
			planets = step(planets, i)
		}
	}
	fmt.Println(energy(planets))
}

type state [8]int64

func getState(planets []planet, idx int) state {
	ans := state{}
	for i, p := range planets {
		ans[i] = p.pos[idx]
		ans[4+i] = p.vel[idx]
	}
	return ans
}

func findCylce(planets []planet, idx int) int64 {
	seen := make(map[state]bool)
	seen[getState(planets, idx)] = true
	it := int64(1)
	for ; ; it++ {
		planets = step(planets, idx)
		if seen[getState(planets, idx)] {
			break
		}
		seen[getState(planets, idx)] = true
	}
	return it
}

func part2(planets []planet) {
	c := [3]int64{}
	for i := 0; i < 3; i++ {
		c[i] = findCylce(planets, i)
	}
	g := utils.LCM64(utils.LCM64(c[0], c[1]), c[2])
	fmt.Println(g)
}

func read() []planet {
	content, err := ioutil.ReadFile(os.Args[1])
	if err != nil {
		log.Fatal(err)
	}
	lines := strings.Split(string(content), "\n")
	planets := []planet{}
	for _, l := range lines {
		n := strings.Split(l, ",")
		planets = append(planets, planet{
			pos: vect{utils.Atoll(n[0]), utils.Atoll(n[1]), utils.Atoll(n[2])},
		})
	}
	return planets
}

func main() {
	part1(read())
	part2(read())
}
