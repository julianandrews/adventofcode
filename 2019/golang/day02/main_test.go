package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestCase1(t *testing.T) {
	program := []int64{1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50}
	vm, err := runWithInputs(program, 9, 10)
	assert.Nil(t, err)
	assert.Equal(t, vm.Memory()[0], int64(3500))
}

func TestCase2(t *testing.T) {
	program := []int64{1, 0, 0, 0, 99}
	vm, err := runWithInputs(program, 0, 0)
	assert.Nil(t, err)
	assert.Equal(t, []int64{2, 0, 0, 0, 99}, vm.Memory())
}

func TestCase3(t *testing.T) {
	program := []int64{2, 3, 0, 3, 99}
	vm, err := runWithInputs(program, 3, 0)
	assert.Nil(t, err)
	assert.Equal(t, []int64{2, 3, 0, 6, 99}, vm.Memory())
}

func TestCase4(t *testing.T) {
	program := []int64{2, 4, 4, 5, 99, 0}
	vm, err := runWithInputs(program, 4, 4)
	assert.Nil(t, err)
	assert.Equal(t, []int64{2, 4, 4, 5, 99, 9801}, vm.Memory())
}

func TestCase5(t *testing.T) {
	program := []int64{1, 1, 1, 4, 99, 5, 6, 0, 99}
	vm, err := runWithInputs(program, 1, 1)
	assert.Nil(t, err)
	assert.Equal(t, []int64{30, 1, 1, 4, 2, 5, 6, 0, 99}, vm.Memory())
}
