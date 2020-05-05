package main

import (
	"errors"
	"fmt"
	"julianandrews/adventofcode/aoc"
	"julianandrews/adventofcode/aoc/intcode"
	"os"
)

func runWithInputs(program []int64, noun int64, verb int64) (intcode.VM, error) {
    programCopy := append([]int64(nil), program...)
    programCopy[1] = noun
    programCopy[2] = verb
	vm := intcode.New(programCopy)
    err := vm.Run()

	return vm, err
}

func part1(program []int64) (int64, error) {
	vm, err := runWithInputs(program, 12, 2)
    if err != nil {
        return 0, err
    }
	return vm.DiagnosticCode(), nil
}

func part2(program []int64) (int, error) {
	for a := 0; a < 99; a++ {
		for b := 0; b < 99; b++ {
			vm, err := runWithInputs(program, int64(a), int64(b))
            if err != nil {
                return 0, err
            }
			if vm.DiagnosticCode() == 19690720 {
				return 100*a + b, nil
			}
		}
	}
	return 0, errors.New("No input found for desired output")
}

func main() {
	program, err := aoc.GetIntcodeProgram()
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
