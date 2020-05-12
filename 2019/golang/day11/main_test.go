package main

import (
    "testing"
	"github.com/stretchr/testify/assert"
    "julianandrews/adventofcode/aoc/intcode"
)

func TestGetPaintSteps(t *testing.T) {
    outputs := make(chan int64)
    go func() {
        for _, v := range []int64{1, 0, 0, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1, 0} {
            outputs <- v
        }
        close(outputs)
    }()
    vm := intcode.NewMockVM(make(chan int64), outputs)
    expected := []PaintStep{
        NewPaintStep(0, 0, true),
        NewPaintStep(-1, 0, false),
        NewPaintStep(-1, -1, true),
        NewPaintStep(0, -1, true),
        NewPaintStep(0, 0, false),
        NewPaintStep(1, 0, true),
        NewPaintStep(1, 1, true),
    }
    var actual []PaintStep;
    panel := make(Panel)
    for step := range paintPanel(&vm, &panel) {
        actual = append(actual, step)
    }
    assert.Equal(t, expected, actual)
}
