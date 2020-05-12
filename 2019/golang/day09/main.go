package main

import (
	"fmt"
	"julianandrews/adventofcode/aoc"
	"julianandrews/adventofcode/aoc/intcode"
	"os"
)

func part1(program []int64) int64 {
	vm := intcode.NewVM(append([]int64(nil), program...))
	go vm.Run()
	vm.Inputs() <- 1
	output := <-vm.Outputs()

	return output
}

func part2(program []int64) int64 {
	vm := intcode.NewVM(append([]int64(nil), program...))
	go vm.Run()
	vm.Inputs() <- 2
	output := <-vm.Outputs()

	return output
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
