package main

import (
	"fmt"
	"julianandrews/adventofcode/aoc"
	"os"
)

func simple_fuel(mass int) int {
	val := mass/3 - 2
	if val > 0 {
		return val
	} else {
		return 0
	}
}

func fuel(mass int) int {
	total_mass := 0
	for mass > 0 {
		mass = simple_fuel(mass)
		total_mass += mass
	}
	return total_mass
}

func part_1(masses []int) int {
	total := 0
	for _, mass := range masses {
		total += simple_fuel(mass)
	}
	return total
}

func part_2(masses []int) int {
	total := 0
	for _, mass := range masses {
		total += fuel(mass)
	}
	return total
}

func main() {
	masses, err := aoc.GetInts()
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}

	fmt.Println("Part 1: ", part_1(masses))
	fmt.Println("Part 2: ", part_2(masses))
}
