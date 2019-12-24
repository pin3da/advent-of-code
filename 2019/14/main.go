package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"os"
	"strings"

	"github.com/pin3da/advent-of-code/utils"
)

type chemical struct {
	quantity int64
	ID       string
	need     []*chemical
}

type mapID map[string]*chemical

func splitSpace(s string) []string {
	return strings.Split(strings.Trim(s, " "), " ")
}

func fromStr(s string) *chemical {
	parts := splitSpace(s)
	return &chemical{
		quantity: utils.Atoll(parts[0]),
		ID:       parts[1],
	}
}

func getOrder(byID mapID, id string, seen map[string]bool) []string {
	if id == "ORE" {
		return []string{id}
	}
	ans := []string{}
	for _, n := range byID[id].need {
		if _, ok := seen[n.ID]; !ok {
			seen[n.ID] = true
			ans = append(ans, getOrder(byID, n.ID, seen)...)
		}
	}
	ans = append(ans, id)
	return ans
}

func getMin(ord []string, byID mapID, req int64) int64 {
	frec := make(map[string]int64)
	frec["FUEL"] = req
	for i := len(ord) - 1; i > 0; i-- {
		need := frec[ord[i]]
		times := (need + byID[ord[i]].quantity - 1) / byID[ord[i]].quantity
		for _, n := range byID[ord[i]].need {
			frec[n.ID] += times * n.quantity
		}
	}
	return frec["ORE"]
}

func main() {
	content, err := ioutil.ReadFile(os.Args[1])
	if err != nil {
		log.Fatal(err)
	}
	lines := strings.Split(string(content), "\n")
	byID := make(mapID)
	for _, l := range lines {
		parts := strings.Split(l, "=>")
		lhs, rhs := parts[0], parts[1]
		target := fromStr(rhs)
		byID[target.ID] = target
		n := strings.Split(lhs, ",")
		for _, it := range n {
			target.need = append(target.need, fromStr(it))
		}
	}
	seen := make(map[string]bool)
	ord := getOrder(byID, "FUEL", seen)
	fmt.Println(getMin(ord, byID, 1))

	target := int64(1000000000000)
	lo := int64(1)
	hi := int64(1000000000000)

	for lo < hi {
		mid := (lo + hi + 1) / 2
		c := getMin(ord, byID, mid)
		if c <= target {
			lo = mid
		} else {
			hi = mid - 1
		}
	}
	fmt.Println(lo)
}
