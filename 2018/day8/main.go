package main

import (
	"fmt"

	"github.com/pin3da/advent-of-code/utils"
)

type node struct {
	meta     []int
	children []*node
}

func readInput(reader *utils.Reader) *node {
	root := &node{}
	root.children = make([]*node, reader.Int())
	root.meta = make([]int, reader.Int())
	for i := range root.children {
		root.children[i] = readInput(reader)
	}
	for i := range root.meta {
		root.meta[i] = reader.Int()
	}
	return root
}

func getValue(root *node) int {
	ans := 0
	if len(root.children) == 0 {
		for _, v := range root.meta {
			ans += v
		}
		return ans
	}

	for _, v := range root.meta {
		if v-1 < len(root.children) {
			ans += getValue(root.children[v-1])
		}
	}
	return ans
}

func main() {
	reader := utils.NewReader()
	tree := readInput(reader)
	fmt.Println(getValue(tree))
}
