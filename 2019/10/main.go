package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"math"
	"os"
	"sort"
	"strings"

	"github.com/pin3da/advent-of-code/utils"
)

type pos struct {
	x float64
	y float64
}
type frec map[pos][]pos

func getFrec(x, y int, grid []string) frec {
	ans := make(frec)
	for i, line := range grid {
		for j, c := range line {
			if c != '#' || (i == x && y == j) {
				continue
			}
			g := utils.GCD(x-i, y-j)
			key := pos{float64((x - i) / g), float64((y - j) / g)}
			ans[key] = append(ans[key], pos{float64(i), float64(j)})
		}
	}
	return ans
}

func part1(grid []string) (frec, pos) {
	best := make(frec)
	last := pos{}
	for i, line := range grid {
		for j, c := range line {
			if c != '#' {
				continue
			}
			cur := getFrec(i, j, grid)
			if len(best) < len(cur) {
				best = cur
				last = pos{float64(i), float64(j)}
			}
		}
	}
	return best, last
}

func normAngle(p pos) float64 {
	a := math.Atan2(p.x, p.y) // Not a typo, computer coordinates ¯\_(ツ)_/¯
	if a < 0 {
		a += 2 * math.Acos(-1)
	}
	a -= math.Acos(0)
	if a < 0 {
		a += 2 * math.Acos(-1)
	}
	return a
}

func dist(a, b pos) float64 {
	dx := a.x - b.x
	dy := a.y - b.y
	return dx*dx + dy*dy
}

func part2(f frec, start pos) []pos {
	ans := []pos{}
	keys := []pos{}
	for k := range f {
		keys = append(keys, k)
		sort.Slice(f[k], func(i, j int) bool {
			return dist(start, f[k][i]) < dist(start, f[k][j])
		})
	}
	sort.Slice(keys, func(i, j int) bool {
		return normAngle(keys[i]) < normAngle(keys[j])
	})
	for done := false; !done; {
		any := false
		for _, k := range keys {
			if len(f[k]) > 0 {
				any = true
				ans = append(ans, f[k][0])
				f[k] = f[k][1:]
			}
		}
		done = !any
	}
	return ans
}

func main() {
	content, err := ioutil.ReadFile(os.Args[1])
	if err != nil {
		log.Fatal(err)
	}
	grid := strings.Split(string(content), "\n")
	f, start := part1(grid)
	fmt.Println("Best", len(f), "at", start)
	ord := part2(f, start)
	fmt.Println("200th", ord[199].y, ord[199].x, "=", ord[199].y*100+ord[199].x)
}
