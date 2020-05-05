package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
	"julianandrews/adventofcode/aoc/intcode"
)

func TestTimes3Immediate(t *testing.T) {
	program := []int64{1002, 4, 3, 4, 33}
	vm := intcode.New(program)
	_, _, err := vm.Step()
	assert.Nil(t, err)
	assert.Equal(t, []int64{1002, 4, 3, 4, 99}, vm.Memory.CopySlice(0, 5))
}

func TestEquals8(t *testing.T) {
	program := []int64{3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8}

	vm := intcode.New(append([]int64(nil), program...))
	go vm.Run()
	vm.Inputs() <- 8
	output := <-vm.Outputs()
	assert.Equal(t, output, int64(1))

    vm2 := intcode.New(append([]int64(nil), program...))
	go vm2.Run()
	vm2.Inputs() <- 7
	output = <-vm2.Outputs()
	assert.Equal(t, output, int64(0))
}

func TestLessThan8(t *testing.T) {
	program := []int64{3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8}

	vm := intcode.New(append([]int64(nil), program...))
	go vm.Run()
	vm.Inputs() <- 7
	output := <-vm.Outputs()
	assert.Equal(t, output, int64(1))

    vm2 := intcode.New(append([]int64(nil), program...))
	go vm2.Run()
	vm2.Inputs() <- 8
	output = <-vm2.Outputs()
	assert.Equal(t, output, int64(0))
}

func TestEquals8Immediate(t *testing.T) {
	program := []int64{3, 3, 1108, -1, 8, 3, 4, 3, 99}

	vm := intcode.New(append([]int64(nil), program...))
	go vm.Run()
	vm.Inputs() <- 8
	output := <-vm.Outputs()
	assert.Equal(t, output, int64(1))

    vm2 := intcode.New(append([]int64(nil), program...))
	go vm2.Run()
	vm2.Inputs() <- 10
	output = <-vm2.Outputs()
	assert.Equal(t, output, int64(0))
}

func TestLessThan8Immediate(t *testing.T) {
	program := []int64{3, 3, 1107, -1, 8, 3, 4, 3, 99}

	vm := intcode.New(append([]int64(nil), program...))
	go vm.Run()
	vm.Inputs() <- 7
	output := <-vm.Outputs()
	assert.Equal(t, output, int64(1))

    vm2 := intcode.New(append([]int64(nil), program...))
	go vm2.Run()
	vm2.Inputs() <- 10
	output = <-vm2.Outputs()
	assert.Equal(t, output, int64(0))
}

func TestMoreComplexCase(t *testing.T) {
	program := []int64{3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20,
		1006, 20, 31, 1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1,
		46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99}

	vm1 := intcode.New(append([]int64(nil), program...))
	go vm1.Run()
	vm1.Inputs() <- 7
    output := <-vm1.Outputs()
	assert.Equal(t, output, int64(999))

    vm2 := intcode.New(append([]int64(nil), program...))
	go vm2.Run()
	vm2.Inputs() <- 8
    output = <-vm2.Outputs()
	assert.Equal(t, output, int64(1000))

    vm3 := intcode.New(append([]int64(nil), program...))
	go vm3.Run()
	vm3.Inputs() <- 10
    output = <-vm3.Outputs()
	assert.Equal(t, output, int64(1001))
}
