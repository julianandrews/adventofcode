package main

import (
	"bufio"
	"errors"
	"fmt"
	"io"
	"julianandrews/adventofcode/aoc"
	"os"
	"reflect"
	"regexp"
	"strconv"
	"strings"
)

type Point [3]int

type Moon struct {
	position Point
	velocity Point
}

func parseMoon(s string) (Moon, error) {
	regex := regexp.MustCompile("^<x=(?P<x>-?[0-9]+), y=(?P<y>-?[0-9]+), z=(?P<z>-?[0-9]+)>$")
	match := regex.FindStringSubmatch(s)
	if match == nil {
		return Moon{}, errors.New("Failed to parse point")
	}
	x, _ := strconv.Atoi(match[1])
	y, _ := strconv.Atoi(match[2])
	z, _ := strconv.Atoi(match[3])
    return Moon{position: [3]int{x, y, z}}, nil
}

func readSystem(reader io.Reader) (System, error) {
	var system System
	scanner := bufio.NewScanner(reader)
	for scanner.Scan() {
		moon, err := parseMoon(strings.TrimSpace(scanner.Text()))
		if err != nil {
			return System{}, err
		}
		system = append(system, moon)
	}

	return system, nil
}

func (moon *Moon) updateVelocity(other Moon) {
	for i := range moon.position {
		diff := moon.position[i] - other.position[i]
		if diff > 0 {
			moon.velocity[i] -= 1
		} else if diff < 0 {
			moon.velocity[i] += 1
		}
	}
}

func (moon *Moon) updatePosition() {
	for i := range moon.position {
		moon.position[i] += moon.velocity[i]
	}
}

func (moon *Moon) energy() int {
	abs := func(x int) int {
		if x < 0 {
			return -x
		}
		return x
	}
	absSum := func(p Point) int {
		return abs(p[0]) + abs(p[1]) + abs(p[2])
	}

	return absSum(moon.position) * absSum(moon.velocity)
}

type System []Moon

func (system *System) step() {
	for i := range *system {
		for j := range *system {
			(*system)[i].updateVelocity((*system)[j])
		}
	}
	for i := range *system {
		(*system)[i].updatePosition()
	}
}

func (system *System) energy() int {
	total := 0
	for _, moon := range *system {
		total += moon.energy()
	}
	return total
}

func (system *System) state() [3][]int {
	var s [3][]int
	for i := range s {
		s[i] = make([]int, len(*system)*2)
		for j, moon := range *system {
			s[i][2*j] = moon.position[i]
			s[i][2*j+1] = moon.velocity[i]
		}
	}
	return s
}

func (system *System) cycleLength() uint64 {
	var cycleLengths [3]uint64
    allCyclesFound := func() bool {
        return cycleLengths[0] != 0 && cycleLengths[1] != 0 && cycleLengths[2] != 0
    }
	initialState := system.state()
	for t := uint64(1); !allCyclesFound(); t++ {
		system.step()
		for i, component := range system.state() {
			if cycleLengths[i] == 0 && reflect.DeepEqual(component, initialState[i]) {
				cycleLengths[i] = t
			}
		}
	}
	x := uint64(1)
	for _, y := range cycleLengths {
		x = lcm(x, uint64(y))
	}
	return x
}

func lcm(x, y uint64) uint64 {
	product := x * y
	for y != 0 {
		x, y = y, x%y
	}
	return product / x
}

func part1(system System) int {
	for i := 0; i < 1000; i++ {
		system.step()
	}
	return system.energy()
}

func part2(system System) uint64 {
	return system.cycleLength()
}

func main() {
	file, err := aoc.OpenInputFile()
	if err != nil {
		fmt.Fprintln(os.Stderr, "Failed to open input file")
		os.Exit(1)
	}
	system, err := readSystem(file)
	if err != nil {
		fmt.Fprintln(os.Stderr, "Failed to parse input")
		os.Exit(1)
	}
	fmt.Println("Part 1: ", part1(system))
	fmt.Println("Part 2: ", part2(system))
}
