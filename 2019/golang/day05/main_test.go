package main

import (
	"github.com/stretchr/testify/assert"
    "julianandrews/adventofcode/aoc/intcode"
	"testing"
)

func TestGetDiagnosticCode(t *testing.T) {
    inputs := make(chan int64)
    outputs := make(chan int64)
    go func() {
        <- inputs
    }()
    go func() {
        outputs <- 0
        outputs <- 0
        outputs <- 17
        outputs <- 5
        close(outputs)
    }()
    vm := intcode.NewMockVM(inputs, outputs)
	result, err := getDiagnosticCode(&vm, 8)

	assert.Nil(t, err)
	assert.Equal(t, int64(17), result)
}

func TestGetDiagnosticCodeFails(t *testing.T) {
    inputs := make(chan int64)
    outputs := make(chan int64)
    go func() {
        <- inputs
    }()
    go func() {
        outputs <- 0
        outputs <- 0
        outputs <- 0
        outputs <- 0
        close(outputs)
    }()
    vm := intcode.NewMockVM(inputs, outputs)
	_, err := getDiagnosticCode(&vm, 8)

	assert.NotNil(t, err)
}
