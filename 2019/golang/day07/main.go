package main

import (
	"fmt"
	"julianandrews/adventofcode/aoc"
	"julianandrews/adventofcode/aoc/intcode"
	"os"
	"sync"
)

func generatePermutations(values []int64) <-chan []int64 {
	ch := make(chan []int64)

	go func() {
		c := make([]int, len(values))
		perm := append([]int64(nil), values...)

		ch <- append([]int64(nil), perm...)

		i := 0
		for i < len(perm) {
			if c[i] < i {
				if i%2 == 0 {
					perm[0], perm[i] = perm[i], perm[0]
				} else {
					perm[c[i]], perm[i] = perm[i], perm[c[i]]
				}
				ch <- append([]int64(nil), perm...)
				c[i] += 1
				i = 0
			} else {
				c[i] = 0
				i++
			}
		}
		close(ch)
	}()

	return ch
}

func part1(program []int64) int64 {
	best := int64(0)
	for perm := range generatePermutations([]int64{0, 1, 2, 3, 4}) {
		signal := int64(0)
		for _, phase := range perm {
			vm := intcode.New(append([]int64(nil), program...))
			go vm.Run()
			vm.Inputs() <- phase
			vm.Inputs() <- signal
			signal = <-vm.Outputs()
		}
		if signal > best {
			best = signal
		}
	}
	return best
}

func part2(program []int64) int64 {
	best := int64(0)

	sendInput := func(vm *intcode.VM, output int64) { vm.Inputs() <- output }

	forwardOutputs := func(
		wg *sync.WaitGroup, source *intcode.VM, dest *intcode.VM, trackBest bool,
	) {
		defer wg.Done()
		for output := range source.Outputs() {
			go sendInput(dest, output)
			if trackBest && output > best {
				best = output
			}
		}
	}

	for perm := range generatePermutations([]int64{5, 6, 7, 8, 9}) {
		vms := make([]intcode.VM, len(perm))
		for i, phase := range perm {
			vms[i] = intcode.New(append([]int64(nil), program...))
			go vms[i].Run()
			sendInput(&vms[i], phase)
		}
		vms[0].Inputs() <- 0

		var wg sync.WaitGroup
		for i := range vms {
			wg.Add(1)
			go forwardOutputs(&wg, &vms[i], &vms[(i+1)%len(vms)], i == len(vms)-1)
		}
		wg.Wait()
	}

	return best
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
