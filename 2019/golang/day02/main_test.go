package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestRunWithInputs(t *testing.T) {
	program := []int64{1, 0, 0, 3, 2, 3, 11, 0, 99, 30, 40, 50}
	vm, err := runWithInputs(program, 9, 10)
	assert.Nil(t, err)
	assert.Equal(t, vm.Memory()[0], int64(3500))
}
