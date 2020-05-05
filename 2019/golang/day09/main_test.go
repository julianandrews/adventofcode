package main

import (
	"github.com/stretchr/testify/assert"
	"julianandrews/adventofcode/aoc/intcode"
	"strconv"
	"testing"
)

func TestQuine(t *testing.T) {
	program := []int64{
		109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
	}
	vm := intcode.New(append([]int64(nil), program...))
	go vm.Run()
	var outputs []int64
	for output := range vm.Outputs() {
		outputs = append(outputs, output)
	}
	assert.Equal(t, program, outputs)
}

func TestBigOutput(t *testing.T) {
	program := []int64{1102, 34915192, 34915192, 7, 4, 7, 99, 0}
	vm := intcode.New(append([]int64(nil), program...))
	go vm.Run()
	output := <-vm.Outputs()
	assert.Equal(t, 16, len(strconv.FormatInt(output, 10)))
}

func TestOutputsMiddle(t *testing.T) {
	program := []int64{104, 1125899906842624, 99}
	vm := intcode.New(append([]int64(nil), program...))
	go vm.Run()
	output := <-vm.Outputs()
	assert.Equal(t, program[1], output)
}
