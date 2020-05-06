package main

import (
	"github.com/stretchr/testify/assert"
	"testing"
)

func TestPart1Case1(t *testing.T) {
	wire1, err := wireFromInstructions("R8,U5,L5,D3")
    assert.Nil(t, err)
	wire2, err := wireFromInstructions("U7,R6,D4,L4")
    assert.Nil(t, err)

	distance, err := part1(wire1, wire2)
	assert.Nil(t, err)
	assert.Equal(t, uint(6), distance)
}

func TestPart1Case2(t *testing.T) {
	wire1, err := wireFromInstructions("R76,D30,R83,U83,L12,D49,R71,U7,L72")
    assert.Nil(t, err)
	wire2, err := wireFromInstructions("U62,R66,U55,R34,D71,R55,D58,R83")
    assert.Nil(t, err)

	distance, err := part1(wire1, wire2)
	assert.Nil(t, err)
	assert.Equal(t, uint(159), distance)
}

func TestPart1Case3(t *testing.T) {
	wire1, err := wireFromInstructions("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51")
    assert.Nil(t, err)
	wire2, err := wireFromInstructions("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")
    assert.Nil(t, err)

	distance, err := part1(wire1, wire2)
	assert.Nil(t, err)
	assert.Equal(t, uint(135), distance)
}

func TestPart2Case1(t *testing.T) {
	wire1, err := wireFromInstructions("R8,U5,L5,D3")
    assert.Nil(t, err)
	wire2, err := wireFromInstructions("U7,R6,D4,L4")
    assert.Nil(t, err)

	distance, err := part2(wire1, wire2)
	assert.Nil(t, err)
	assert.Equal(t, uint(30), distance)
}

func TestPart2Case2(t *testing.T) {
	wire1, err := wireFromInstructions("R75,D30,R83,U83,L12,D49,R71,U7,L72")
    assert.Nil(t, err)
	wire2, err := wireFromInstructions("U62,R66,U55,R34,D71,R55,D58,R83")
    assert.Nil(t, err)

	distance, err := part2(wire1, wire2)
	assert.Nil(t, err)
	assert.Equal(t, uint(610), distance)
}

func TestPart2Case3(t *testing.T) {
	wire1, err := wireFromInstructions("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51")
    assert.Nil(t, err)
	wire2, err := wireFromInstructions("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")
    assert.Nil(t, err)

	distance, err := part2(wire1, wire2)
	assert.Nil(t, err)
	assert.Equal(t, uint(410), distance)
}
