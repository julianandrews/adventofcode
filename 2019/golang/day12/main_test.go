package main

import (
    "github.com/stretchr/testify/assert"
    "strings"
    "testing"
)

func TestEnergyCase1(t *testing.T) {
    input := `
        <x=-1, y=0, z=2>
        <x=2, y=-10, z=-7>
        <x=4, y=-8, z=8>
        <x=3, y=5, z=-1>
    `
    system, err := readSystem(strings.NewReader(strings.TrimSpace(input)))
    assert.Nil(t, err)
    for i := 0; i < 10; i++ {
        system.step()
    }

    assert.Equal(t, 179, system.energy())
}

func TestEnergyCase2(t *testing.T) {
    input := `
        <x=-8, y=-10, z=0>
        <x=5, y=5, z=10>
        <x=2, y=-7, z=3>
        <x=9, y=-8, z=-3>
    `
    system, err := readSystem(strings.NewReader(strings.TrimSpace(input)))
    assert.Nil(t, err)
    for i := 0; i < 100; i++ {
        system.step()
    }

    assert.Equal(t, 1940, system.energy())
}

func TestCycleLengthCase1(t *testing.T) {
    input := `
        <x=-1, y=0, z=2>
        <x=2, y=-10, z=-7>
        <x=4, y=-8, z=8>
        <x=3, y=5, z=-1>
    `
    system, err := readSystem(strings.NewReader(strings.TrimSpace(input)))

    assert.Nil(t, err)
    assert.Equal(t, uint64(2772), system.cycleLength())
}

func TestCycleLengthCase2(t *testing.T) {
    input := `
        <x=-8, y=-10, z=0>
        <x=5, y=5, z=10>
        <x=2, y=-7, z=3>
        <x=9, y=-8, z=-3>
    `
    system, err := readSystem(strings.NewReader(strings.TrimSpace(input)))

    assert.Nil(t, err)
    assert.Equal(t, uint64(4686774924), system.cycleLength())
}
