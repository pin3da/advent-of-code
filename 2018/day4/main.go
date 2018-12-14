package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
	"time"
)

// Event ...
type Event struct {
	time   time.Time
	info   string
	minute int
}

func parseTime(timeStr string) (time.Time, int) {
	var y, mo, d, h, mi int
	if _, err := fmt.Sscanf(timeStr, "%d-%d-%d %d:%d", &y, &mo, &d, &h, &mi); err != nil {
		panic(err)
	}
	return time.Date(y, time.Month(mo), d, h, mi, 0, 0, time.UTC), mi
}
func readInput() (result []Event) {
	scanner := bufio.NewScanner(os.Stdin)

	for scanner.Scan() {
		line := scanner.Text()
		parts := strings.Split(line, "] ")
		timeStr := parts[0][1:]
		info := parts[1]
		timeStamp, minute := parseTime(timeStr)
		result = append(result, Event{timeStamp, info, minute})
	}
	sort.Slice(result, func(i, j int) bool {
		return result[i].time.Before(result[j].time)
	})
	return result
}

// Guard ...
type Guard struct {
	frec  map[int]int
	total int
}

func parseGuards(input []Event) map[string]*Guard {
	lastGuard := ""
	start := int(-1)
	guards := make(map[string]*Guard)
	for _, event := range input {
		if strings.HasPrefix(event.info, "Guard") {
			lastGuard = strings.Split(event.info, " ")[1][1:]
			if guards[lastGuard] == nil {
				guards[lastGuard] = &Guard{frec: make(map[int]int)}
			}
		}
		if strings.HasPrefix(event.info, "falls") {
			start = event.minute
		}
		if strings.HasPrefix(event.info, "wakes") {
			duration := event.minute - start
			guards[lastGuard].total += duration
			for i := 0; i < duration; i++ {
				guards[lastGuard].frec[start+i]++
			}
		}
	}
	return guards
}

func main() {
	input := readInput()
	guards := parseGuards(input)
	best, bestFrec := int(0), int(0)
	first, second := 0, 0

	for key, val := range guards {
		localFrec := 0
		localID := 0
		tmp, err := strconv.Atoi(key)
		if err != nil {
			panic("invalid id")
		}
		for i, f := range val.frec {
			if f > localFrec {
				localFrec = f
				localID = i
			}
		}
		if val.total > best {
			best = val.total
			first = tmp * localID
		}
		if localFrec > bestFrec {
			second = tmp * localID
			bestFrec = localFrec
		}
	}

	fmt.Println(first)
	fmt.Println(second)
}
