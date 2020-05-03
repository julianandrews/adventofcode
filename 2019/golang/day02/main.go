package main

import (
    "errors"
	"fmt"
	"julianandrews/adventofcode/aoc"
	"julianandrews/adventofcode/aoc/intcode"
	"os"
	"strings"
)

func runWithInputs(program []int64, noun int64, verb int64) (intcode.VM, error) {
    programCopy := append([]int64(nil), program...)
	vm := intcode.New(programCopy)
	vm.Memory.Set(1, noun)
	vm.Memory.Set(2, verb)
	for {
		op, _, err := vm.Step()
		if err != nil {
			return vm, err
		}
		if op == intcode.OP_HALT {
			break
		}
	}

	return vm, nil
}

func part1(program []int64) (int64, error) {
	vm, err := runWithInputs(program, 12, 2)
    if err != nil {
        return 0, err
    }
	return vm.Memory.Get(0), nil
}

func part2(program []int64) (int, error) {
	for a := 0; a < 99; a++ {
		for b := 0; b < 99; b++ {
			vm, err := runWithInputs(program, int64(a), int64(b))
            if err != nil {
                return 0, err
            }
			if vm.Memory.Get(0) == 19690720 {
				return 100*a + b, nil
			}
		}
	}
	return 0, errors.New("No input found for desired output")
}

func main() {
	data, err := aoc.GetInput()
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
	program, err := intcode.ParseProgram(strings.TrimSpace(data))
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
    part1Result, err := part1(program)
    if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
    }
	fmt.Println("Part 1: ", part1Result)
    part2Result, err := part2(program)
    if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
    }
	fmt.Println("Part 2: ", part2Result)
}
