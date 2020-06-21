package main

import (
	"fmt"
	"julianandrews/adventofcode/aoc"
	"julianandrews/adventofcode/aoc/intcode"
	"os"
)

func runWithInput(vm intcode.VM, input int64) int64 {
	go vm.Run()
	vm.Inputs() <- input
	return <-vm.Outputs()
}

func part1(program []int64) int64 {
	vm := intcode.NewVM(append([]int64(nil), program...))
    return runWithInput(vm, 1)
}

func part2(program []int64) int64 {
	vm := intcode.NewVM(append([]int64(nil), program...))
    return runWithInput(vm, 2)
}

func main() {
	program, err := aoc.GetIntcodeProgram()
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
	fmt.Println("Part 1: ", part1(program))
	fmt.Println("Part 2: ", part2(program))
}
