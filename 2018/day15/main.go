package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"

	"github.com/pin3da/advent-of-code/utils"
)

// Pos defines the ubication of a cell or unit on the map
type Pos struct {
	r, c int
}

// Unit can be an Efe or a Globin
type Unit struct {
	Pos
	hp, kind int // elve, globin, wall
}

// Board represents the status of the game
type Board [][]*Unit

// ByPos returns if a comes first in the reading order
func ByPos(a, b Pos) bool {
	if a.r == b.r {
		return a.c < b.c
	}
	return a.r < b.r
}

func readInput() Board {
	sc := bufio.NewScanner(os.Stdin)
	board := Board{}
	for i := 0; sc.Scan(); i++ {
		text := sc.Text()
		line := make([]*Unit, len(text))
		for j, cel := range text {
			switch cel {
			case 'E':
				line[j] = &Unit{Pos{i, j}, 200, 0}
			case 'G':
				line[j] = &Unit{Pos{i, j}, 200, 1}
			case '#':
				line[j] = &Unit{Pos{i, j}, 0, 2}
			default:
				line[j] = &Unit{Pos{i, j}, 0, 3}
			}
		}
		board = append(board, line)
	}
	return board
}

// delta r and c in reading order
var dr = []int{-1, 0, 0, 1}
var dc = []int{0, -1, 1, 0}

const inf = 100000

func genNeighbors(dist [][]int, cur *Unit) []Pos {
	result := []Pos{}
	for i := range dc {
		t := Pos{cur.r + dr[i], cur.c + dc[i]}
		if t.r >= 0 && t.r < len(dist) && t.c >= 0 && t.c < len(dist[0]) && dist[t.r][t.c] < inf {
			result = append(result, t)
		}
	}
	return result
}

func distance(board Board, start Pos) [][]int {
	res := make([][]int, len(board))
	for i := range res {
		res[i] = make([]int, len(board[0]))
		for j := range res[i] {
			res[i][j] = inf
		}
	}

	q := []Pos{start}
	res[start.r][start.c] = 0
	for len(q) > 0 {
		cur := q[0]
		q = q[1:]
		for i := range dr {
			t := Pos{cur.r + dr[i], cur.c + dc[i]}
			if t.r >= 0 && t.r < len(board) && t.c >= 0 && t.c < len(board[0]) && board[t.r][t.c].kind == 3 {
				if res[cur.r][cur.c]+1 < res[t.r][t.c] {
					res[t.r][t.c] = res[cur.r][cur.c] + 1
					q = append(q, t)
				}
			}
		}
	}

	return res
}

func nextMove(cur *Pos, dist [][]int, rivals []*Unit) *Pos {
	rechable := []Pos{}
	for _, r := range rivals {
		rechable = append(rechable, genNeighbors(dist, r)...)
	}

	minDist := inf
	for _, it := range rechable {
		minDist = utils.MinInt(minDist, dist[it.r][it.c])
	}

	nearest := []Pos{}
	for _, it := range rechable {
		if dist[it.r][it.c] == minDist {
			nearest = append(nearest, it)
		}
	}
	sort.Slice(nearest, func(i, j int) bool {
		return ByPos(nearest[i], nearest[j])
	})
	if len(nearest) > 0 {
		return &nearest[0]
	}
	return nil
}

func toAttack(cur *Unit, board Board) *Unit {
	possible := []*Unit{}
	for i := range dr {
		t := Pos{cur.r + dr[i], cur.c + dc[i]}
		if t.r >= 0 && t.r < len(board) && t.c >= 0 && t.c < len(board[0]) && board[t.r][t.c].kind < 2 {
			if (board[t.r][t.c].kind ^ 1) == cur.kind { // enemies
				possible = append(possible, board[t.r][t.c])
			}
		}
	}
	if len(possible) > 0 {
		minHP := possible[0].hp
		for _, it := range possible {
			minHP = utils.MinInt(minHP, it.hp)
		}
		for i, it := range possible {
			if it.hp == minHP {
				return possible[i]
			}
		}
	}
	return nil
}

func printBoard(board Board) {
	letter := "EG#."
	for _, line := range board {
		for _, cell := range line {
			fmt.Print(string(letter[cell.kind]))
		}
		fmt.Println()
	}
}

func aliveUnits(board Board) []*Unit {
	order := []*Unit{}
	for _, line := range board {
		for _, u := range line {
			if u.kind < 2 {
				order = append(order, u)
			}
		}
	}
	return order
}

func playRound(board Board, attack int) bool {
	order := aliveUnits(board)
	for _, cur := range order {
		if cur.kind > 1 {
			//			fmt.Println("SKIPPED because just died")
			continue
		}
		rivals := []*Unit{}
		for _, alive := range order {
			if alive.kind < 2 && (alive.kind^1) == cur.kind {
				rivals = append(rivals, alive)
			}
		}
		if len(rivals) == 0 {
			return false
		}
		if toAttack(cur, board) == nil { // move first
			dist := distance(board, cur.Pos)

			toMove := nextMove(&cur.Pos, dist, rivals)
			if toMove != nil {
				distBack := distance(board, *toMove)
				nextCell := nextMove(toMove, distBack, []*Unit{cur})
				board[cur.r][cur.c], board[nextCell.r][nextCell.c] = board[nextCell.r][nextCell.c], board[cur.r][cur.c]
				board[cur.r][cur.c].Pos, board[nextCell.r][nextCell.c].Pos = board[nextCell.r][nextCell.c].Pos, board[cur.r][cur.c].Pos
			}
		}

		if rival := toAttack(cur, board); rival != nil {
			if cur.kind == 0 {
				rival.hp -= attack
			} else {
				rival.hp -= 3
			}
			if rival.hp <= 0 {
				board[rival.r][rival.c].kind = 3
			}
		}
	}
	return true
}

func play(board Board, attack int) int {
	rounds := 0
	for ; ; rounds++ {
		if complete := playRound(board, attack); !complete {
			break
		}
	}
	// printBoard(board)
	ans := 0
	for _, u := range aliveUnits(board) {
		ans += u.hp
	}
	return (ans * (rounds))
}

func copyBoard(board Board) Board {
	ans := Board{}
	for _, row := range board {
		line := []*Unit{}
		for _, cell := range row {
			tmp := *cell
			line = append(line, &tmp)
		}
		ans = append(ans, line)
	}
	return ans
}

func filterElves(all []*Unit) []*Unit {
	elves := []*Unit{}
	for _, it := range all {
		if it.kind == 0 {
			elves = append(elves, it)
		}
	}
	return elves
}

func part2(board Board) {
	lo, hi := 4, 10000
	target := len(filterElves(aliveUnits(board)))
	fmt.Println("target:", target)

	for lo < hi {
		mid := (lo + hi) / 2
		tmp := copyBoard(board)
		play(tmp, mid)
		// fmt.Println("playing with attack", mid)
		// printBoard(tmp)
		// fmt.Println(len(filterElves(aliveUnits(tmp))))
		if target == len(filterElves(aliveUnits(tmp))) {
			hi = mid
		} else {
			lo = mid + 1
		}
	}
	fmt.Println(play(board, lo))
}

func main() {
	board := readInput()
	fmt.Println(play(copyBoard(board), 3))
	part2(board)
}
