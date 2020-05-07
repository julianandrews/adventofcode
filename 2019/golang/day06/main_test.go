package main

import (
	"bufio"
	"github.com/stretchr/testify/assert"
	"strings"
	"testing"
)

func TestPart1(t *testing.T) {
	reader := strings.NewReader(strings.TrimSpace(`
        COM)B
        B)C
        C)D
        D)E
        E)F
        B)G
        G)H
        D)I
        E)J
        J)K
        K)L
    `))
	orbitMap, err := parseOrbits(bufio.NewScanner(reader))
    assert.Nil(t, err)
	assert.Equal(t, uint(42), part1(orbitMap))
}

func TestPart2(t *testing.T) {
	reader := strings.NewReader(strings.TrimSpace(`
        COM)B
        B)C
        C)D
        D)E
        E)F
        B)G
        G)H
        D)I
        E)J
        J)K
        K)L
        K)YOU
        I)SAN
    `))
	orbitMap, err := parseOrbits(bufio.NewScanner(reader))
    assert.Nil(t, err)
    result, err := part2(orbitMap)
    assert.Nil(t, err)
	assert.Equal(t, uint(4), result)
}
