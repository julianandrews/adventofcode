package main

import (
    "errors"
	"fmt"
	"julianandrews/adventofcode/aoc"
	"julianandrews/adventofcode/aoc/intcode"
	"os"
)

func getDiagnosticCode(program []int64, input int64) (int64, error) {
	vm := intcode.New(append([]int64(nil), program...))
    go vm.Run()
    vm.Inputs() <- input
    for output := range vm.Outputs() {
        if output != 0 {
            return output, nil
        }
    }
    return 0, errors.New("No diagnostic code found")
}

func part1(program []int64) (int64, error) {
    return getDiagnosticCode(program, 1)
}

func part2(program []int64) (int64, error) {
    return getDiagnosticCode(program, 5)
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