package main

import (
    "testing"
    "github.com/stretchr/testify/assert"
)

func TestSimpleFuel(t *testing.T) {
    assert := assert.New(t)

    assert.Equal(simple_fuel(12), 2)
    assert.Equal(simple_fuel(14), 2)
    assert.Equal(simple_fuel(1969), 654)
    assert.Equal(simple_fuel(100756), 33583)
}

func TestFuel(t *testing.T) {
    assert := assert.New(t)

    assert.Equal(fuel(14), 2)
    assert.Equal(fuel(1969), 966)
    assert.Equal(fuel(100756), 50346)
}
