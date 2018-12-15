package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"

	"github.com/pin3da/advent-of-code/utils"
)

func getID(s string) int {
	return int(s[0]-'A') + 1
}

func readInput() (map[int][]int, map[int]int, map[int]bool) {
	graph := make(map[int][]int)
	inc := make(map[int]int)
	nodes := make(map[int]bool)
	sc := bufio.NewScanner(os.Stdin)
	for sc.Scan() {
		var uS, vS string
		_, err := fmt.Sscanf(sc.Text(), "Step %s must be finished before step %s can begin.", &uS, &vS)
		if err != nil {
			panic("incorrect input")
		}
		u, v := getID(uS), getID(vS)
		graph[u] = append(graph[u], v)
		inc[v]++
		nodes[u] = true
		nodes[v] = true
	}
	return graph, inc, nodes
}

func main() {
	graph, inc, nodes := readInput()
	queue := []int{}
	minStart := make(map[int]int)
	workers, delay := 5, 60
	maxLen := len(nodes)*(27+delay) + 1

	for node := range nodes {
		if inc[node] == 0 {
			queue = append(queue, node)
		}
	}

	schedule := make([][]int, workers)
	for i := range schedule {
		schedule[i] = make([]int, maxLen)
	}

	totalTime := 0

	for len(queue) > 0 {
		sort.Slice(queue, func(i, j int) bool {
			// return queue[i] < queue[i] // Problem 1
			return minStart[queue[i]] < minStart[queue[j]] // Problem 2

		})
		freeWorker := -1
		for w := 0; w < workers; w++ {
			if schedule[w][totalTime] == 0 {
				freeWorker = w
			}
		}
		node := queue[0]
		if freeWorker == -1 || totalTime < minStart[node] {
			totalTime++
			continue
		}
		for delta := 0; delta < node+delay; delta++ {
			schedule[freeWorker][totalTime+delta] = node
		}
		currentEnd := totalTime + node + delay
		fmt.Print(string(node + 'A' - 1))
		queue = queue[1:]
		for _, to := range graph[node] {
			minStart[to] = utils.MaxInt(minStart[to], currentEnd)
			inc[to]--
			if inc[to] == 0 {
				queue = append(queue, to)
			}
		}
	}
	fmt.Println()

	for allEnded := false; ; totalTime++ {
		allEnded = true
		for w := 0; w < workers; w++ {
			if schedule[w][totalTime] != 0 {
				allEnded = false
			}
		}
		if allEnded {
			break
		}
	}
	fmt.Println(totalTime)
}
