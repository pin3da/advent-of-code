package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

const offset = 300 + 10
const maxLen = offset * 3

// State represents the positions of the plants
type State [maxLen]rune

// Note represents the transitions given in the puzzle
type Note [5]rune

func readInput() (State, map[Note]rune) {
	sc := bufio.NewScanner(os.Stdin)
	sc.Scan()
	line := ""
	fmt.Sscanf(sc.Text(), "initial state: %s", &line)
	state := State{}
	for i := range state {
		state[i] = '.'
	}
	for i := range line {
		state[i+offset] = rune(line[i])
	}

	transitions := make(map[Note]rune)
	for sc.Scan() {
		var tmp, val string
		_, err := fmt.Sscanf(sc.Text(), "%s => %s", &tmp, &val)
		if err != nil { // skip empty lines
			continue
		}
		key := Note{}
		for i := range tmp {
			key[i] = rune(tmp[i])
		}
		transitions[key] = rune(val[0])
	}
	return state, transitions
}

func note(s []rune) (ans Note) {
	for i := range s {
		ans[i] = s[i]
	}
	return ans
}

func printState(state State) {
	for i := range state {
		fmt.Print(string(state[i]))
	}
	fmt.Println()
}

func nextState(state State, transitions map[Note]rune) (next State) {
	for i := range next {
		next[i] = '.'
	}
	for i := 2; i+2 < len(state); i++ {
		cur := note(state[i-2 : i+3])
		if val, ok := transitions[cur]; ok {
			next[i] = val
		}
	}
	return next
}

func toString(s State) string {
	result := []string{}
	for _, it := range s {
		result = append(result, string(it))
	}
	return strings.Join(result, "")
}

func areSame(a, b string) bool {
	first := a[strings.Index(string(a), "#") : strings.LastIndex(string(a), "#")+1]
	second := b[strings.Index(string(b), "#") : strings.LastIndex(string(b), "#")+1]
	return first == second
}

func main() {
	state, transitions := readInput()
	// iters := 20 // Part 1
	iters := 50000000000 // Part 2
	for ; iters > 0; iters-- {
		next := nextState(state, transitions)
		if areSame(toString(next), toString(state)) {
			// printState(state)
			// printState(next)
			break
		}
		state = next
	}

	fmt.Println("remaining iters", iters)

	total := 0
	for i := range state {
		if state[i] == '#' {
			total += i - offset
		}
	}

	fmt.Println(total + strings.Count(toString(state), "#")*iters)
}
