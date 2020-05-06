package main

import (
	"bufio"
	"errors"
	"fmt"
	"julianandrews/adventofcode/aoc"
	"os"
	"strconv"
	"strings"
)

type Point struct {
	x int
	y int
}

type Wire map[Point]uint

func wireFromInstructions(instructions string) (Wire, error) {
	wire := make(Wire)
	var signal_distance uint
	var point Point
	for _, instruction := range strings.Split(instructions, ",") {
		if len(instruction) == 0 {
			return nil, errors.New("Invalid empty instruction")
		}
		direction := instruction[0]
		distance, err := strconv.Atoi(instruction[1:])
		if err != nil {
			return nil, err
		}
		for i := 0; i < distance; i++ {
			switch direction {
			case 'U':
				point.y++
			case 'R':
				point.x++
			case 'D':
				point.y--
			case 'L':
				point.x--
			}
			signal_distance++
			if _, ok := wire[point]; !ok {
				wire[point] = signal_distance
			}
		}
	}

	return wire, nil
}

func (wire Wire) intersections(other Wire) []Point {
	var intersections []Point
	for point := range wire {
		if _, ok := other[point]; ok {
			intersections = append(intersections, point)
		}
	}

	return intersections
}

func minDistance(wire1 Wire, wire2 Wire, distance func(Point) uint) (uint, error) {
	intersections := wire1.intersections(wire2)
	if len(intersections) == 0 {
		return 0, errors.New("No intersections found")
	}
	minDistance := distance(intersections[0])
	for _, point := range intersections {
		distance := distance(point)
		if distance < minDistance {
			minDistance = distance
		}
	}
	return minDistance, nil
}

func part1(wire1 Wire, wire2 Wire) (uint, error) {
	manhattanDistance := func(point Point) uint {
        abs := func(x int) uint {
            if x < 0 {
                return uint(-x)
            }
            return uint(x)
        }

		return abs(point.x) + abs(point.y)
	}
	return minDistance(wire1, wire2, manhattanDistance)
}

func part2(wire1 Wire, wire2 Wire) (uint, error) {
	totalSignalDistance := func(p Point) uint {
		return wire1[p] + wire2[p]
	}
	return minDistance(wire1, wire2, totalSignalDistance)
}

func main() {
	pFile, err := aoc.OpenInputFile()
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
	scanner := bufio.NewScanner(pFile)
	var wires []Wire
	for scanner.Scan() {
		wire, err := wireFromInstructions(scanner.Text())
		if err != nil {
			fmt.Fprintln(os.Stderr, err)
			os.Exit(1)
		}
		wires = append(wires, wire)
	}
	if len(wires) != 2 {
		fmt.Fprintln(os.Stderr, "Expected exactly two wires")
		os.Exit(1)
	}
	part1Result, err := part1(wires[0], wires[1])
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
	fmt.Println("Part 1: ", part1Result)

	part2Result, err := part2(wires[0], wires[1])
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
	fmt.Println("Part 2: ", part2Result)
}
