package main

import "fmt"

type layer [][]int

func readInput() []layer {
	var line string
	fmt.Scan(&line)
	wide, height := 25, 6
	total := wide * height
	if len(line)%total != 0 {
		panic("corrupted input")
	}
	layers := make([]layer, len(line)/total)
	for i := range layers {
		layers[i] = make(layer, height)
		for j := range layers[i] {
			layers[i][j] = make([]int, wide)
		}
	}
	for i, val := range line {
		imgID := i % total
		layers[i/total][imgID/wide][imgID%wide] = int(val - '0')
	}
	return layers
}

func getFrec(l layer) []int {
	ans := make([]int, 3)
	for i := range l {
		for j := range l[i] {
			ans[l[i][j]]++
		}
	}
	return ans
}

func decode(layers []layer) {
	a, b, c := len(layers), len(layers[0]), len(layers[0][0])
	msg := []string{" ", "#", "panic"}
	for i := 0; i < b; i++ {
		for j := 0; j < c; j++ {
			ans := 2
			for k := 0; k < a; k++ {
				if t := layers[k][i][j]; t != 2 {
					ans = t
					break
				}
			}
			fmt.Print(msg[ans])
		}
		fmt.Println("")
	}
}

func main() {
	layers := readInput()
	best := []int{1000000, 0, 0}
	for i := range layers {
		f := getFrec(layers[i])
		if f[0] < best[0] {
			best = f
		}
	}
	fmt.Println(best[1] * best[2])
	decode(layers)
}
