package utils

import (
	"bufio"
	"os"
	"strconv"
)

type Reader struct {
	scanner *bufio.Scanner
	Ended   bool
}

func NewReader() *Reader {
	var reader Reader
	reader.scanner = bufio.NewScanner(os.Stdin)
	reader.scanner.Split(bufio.ScanWords)
	reader.Ended = false
	return &reader
}

func (r *Reader) Int() int {
	tmp := r.Next()
	if !r.Ended {
		res, _ := strconv.Atoi(tmp)
		return res
	}
	return 0
}

func (r *Reader) IntArray(len int) []int {
	result := make([]int, len)
	for i := 0; i < len; i++ {
		result[i] = r.Int()
	}
	return result
}

func (r *Reader) Next() string {
	if r.scanner.Scan() {
		return r.scanner.Text()
	}
	r.Ended = true
	return ""
}
