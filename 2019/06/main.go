package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func readInput() [][]string {
	scanner := bufio.NewScanner(os.Stdin)
	ans := [][]string{}
	for scanner.Scan() {
		line := strings.Split(scanner.Text(), ")")
		ans = append(ans, line)
	}
	return ans
}

type node struct {
	to    []string
	total int
}

type graph map[string]*node

func dfs(g graph, root string) {
	g[root].total++
	for _, to := range g[root].to {
		dfs(g, to)
	}
}

func part1(g graph) {
	for n := range g {
		dfs(g, n)
	}
	total := 0
	for _, v := range g {
		total += v.total
	}
	fmt.Println(total - len(g))
}

func part2(g graph) {
	seen := map[string]bool{}
	cur := []string{"YOU"}
	steps := 0
	for len(cur) > 0 {
		next := []string{}
		for _, n := range cur {
			for _, to := range g[n].to {
				if _, ok := seen[to]; !ok {
					if to == "SAN" {
						fmt.Println("Shortest Path:", steps-1)
						return
					}
					seen[to] = true
					next = append(next, to)
				}
			}
		}
		steps++
		cur = next
	}
	panic("No path found")
}

func main() {
	in := readInput()
	g := graph{}
	g2 := graph{}
	for _, edge := range in {
		from, to := edge[0], edge[1]
		g[from], g[to] = &node{}, &node{}
		g2[from], g2[to] = &node{}, &node{}
	}
	for _, edge := range in {
		from, to := edge[0], edge[1]
		g[from].to = append(g[from].to, to)
		g2[from].to = append(g2[from].to, to)
		g2[to].to = append(g2[to].to, from)
	}
	part1(g)
	part2(g2)
}
