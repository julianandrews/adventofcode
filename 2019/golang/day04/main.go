package main

import (
	"fmt"
	"julianandrews/adventofcode/aoc"
	"os"
	"strconv"
	"strings"
)

func getDigits(n int) []int {
	var digits []int
	for n > 0 {
		digits = append(digits, n%10)
		n /= 10
	}

	// reverse the digits
	for i := len(digits)/2 - 1; i >= 0; i-- {
		opp := len(digits) - 1 - i
		digits[i], digits[opp] = digits[opp], digits[i]
	}

	return digits
}

func nonDecreasing(digits []int) bool {
	for i, d := range digits[1:] {
		if d < digits[i] {
			return false
		}
	}
	return true
}

func hasAdjacentPair(digits []int) bool {
	for i, d := range digits[1:] {
		if d == digits[i] {
			return true
		}
	}
	return false
}

func hasExactRun(n int, digits []int) bool {
	runLength := 1
	for i, d := range digits[1:] {
		if d == digits[i] {
			runLength++
		} else {
			if runLength == n {
				break
			}
			runLength = 1
		}
	}
	return runLength == n
}

func isSimpleCandidate(n int) bool {
	digits := getDigits(n)
	return len(digits) == 6 && nonDecreasing(digits) && hasAdjacentPair(digits)
}

func isCandidate(n int) bool {
	digits := getDigits(n)
	return len(digits) == 6 && nonDecreasing(digits) && hasExactRun(2, digits)
}

func part1(start int, end int) uint {
	count := uint(0)
	for x := start; x <= end; x++ {
		if isSimpleCandidate(x) {
			count++
		}
	}
	return count
}

func part2(start int, end int) uint {
	count := uint(0)
	for x := start; x <= end; x++ {
		if isCandidate(x) {
			count++
		}
	}
	return count
}

func main() {
	input, err := aoc.GetInput()
	if err != nil {
		fmt.Fprintln(os.Stderr, "Failed to read input")
		os.Exit(1)
	}
	parts := strings.Split(strings.TrimSpace(input), "-")
	if len(parts) != 2 {
		fmt.Fprintln(os.Stderr, "Invalid range")
	}
	start, err := strconv.Atoi(parts[0])
	if err != nil {
		fmt.Fprintln(os.Stderr, "Failed to parse start")
		os.Exit(1)
	}
	end, err := strconv.Atoi(parts[1])
	if err != nil {
		fmt.Fprintln(os.Stderr, "Failed to parse end")
		os.Exit(1)
	}

	fmt.Println("Part 1: ", part1(start, end))
	fmt.Println("Part 2: ", part2(start, end))
}
