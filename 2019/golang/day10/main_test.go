package main

import (
    "bufio"
    "io"
	"github.com/stretchr/testify/assert"
    "strings"
	"testing"
)

func trimInput(s string) io.Reader {
    lines := strings.Split(strings.TrimSpace(s), "\n")
    for i, line := range lines {
        lines[i] = strings.TrimSpace(line)
    }

    return strings.NewReader(strings.Join(lines, "\n"))
}

func TestCase1(t *testing.T) {
    asteroidField, err := parseAsteroidField(bufio.NewScanner(trimInput(`
        .#..#
        .....
        #####
        ....#
        ...##
    `)))
    assert.Nil(t, err)
    monitoringStation := asteroidField.monitoringStation()
    assert.Equal(t, Point{x: 3, y: 4}, monitoringStation)
    assert.Equal(t, 8, asteroidField.visibleCount(Point{x: 3, y: 4}))
}

func TestCase2(t *testing.T) {
    asteroidField, err := parseAsteroidField(bufio.NewScanner(trimInput(`
        ......#.#.
        #..#.#....
        ..#######.
        .#.#.###..
        .#..#.....
        ..#....#.#
        #..#....#.
        .##.#..###
        ##...#..#.
        .#....####
    `)))
    assert.Nil(t, err)
    monitoringStation := asteroidField.monitoringStation()
    assert.Equal(t, Point{x: 5, y: 8}, monitoringStation)
    assert.Equal(t, 33, asteroidField.visibleCount(Point{x: 5, y: 8}))
}

func TestCase3(t *testing.T) {
    asteroidField, err := parseAsteroidField(bufio.NewScanner(trimInput(`
        .#....#####...#..
        ##...##.#####..##
        ##...#...#.#####.
        ..#.........###..
        ..#.#.....#....##
    `)))
    assert.Nil(t, err)
    assert.Equal(t, Point{x: 11, y: 2}, asteroidField.nthAsteroid(Point{x: 8, y: 3}, 8))
}
