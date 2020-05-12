package main

import (
	"github.com/stretchr/testify/assert"
	"julianandrews/adventofcode/aoc/intcode"
	"testing"
)

func TestRunWithInput(t *testing.T) {
    input := int64(5)
    output := int64(6)
    inputs := make(chan int64)
    outputs := make(chan int64)
    go func() {
        <- inputs
    }()
    go func() {
        outputs <- output
        close(outputs)
    }()
    vm := intcode.NewMockVM(inputs, outputs)
    result := runWithInput(&vm, input)

	assert.Equal(t, output, result)
}
