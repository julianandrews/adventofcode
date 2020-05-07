package main

import (
    "bufio"
    "errors"
    "fmt"
    "julianandrews/adventofcode/aoc"
    "julianandrews/adventofcode/aoc/graphs"
    "strings"
    "os"
)

type OrbitMap map[interface {}][]interface {}

func parseOrbits(scanner *bufio.Scanner) (OrbitMap, error) {
    orbitMap := make(OrbitMap)
	for scanner.Scan() {
        parts := strings.Split(strings.TrimSpace(scanner.Text()), ")")
        if len(parts) != 2 {
            return nil, errors.New("Invalid input")
        }
        orbitMap[parts[0]] = append(orbitMap[parts[0]], parts[1])
        orbitMap[parts[1]] = append(orbitMap[parts[1]], parts[0])
	}
    return orbitMap, nil
}

func part1(orbitMap OrbitMap) uint {
    neighbors := func(s interface {}) []interface {} {return orbitMap[s]}
    traversal := graphs.BFTraversal("COM", neighbors)
    var total uint
    for node := traversal.Next(); node != nil; node = traversal.Next() {
        total += node.Depth
    }
    return total
}

func part2(orbitMap OrbitMap) (uint, error) {
    neighbors := func(s interface {}) []interface {} {return orbitMap[s]}
    traversal := graphs.BFTraversal("YOU", neighbors)
    for node := traversal.Next(); node != nil; node = traversal.Next() {
        if node.Value == "SAN" {
            return node.Depth - 2, nil
        }
    }
    return 0, errors.New("SAN not found!")
}

func main() {
	pFile, err := aoc.OpenInputFile()
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
    orbitMap, err := parseOrbits(bufio.NewScanner(pFile))
    if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
    }

    fmt.Println("Part 1: ", part1(orbitMap))
    part2Result, err := part2(orbitMap)
    if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
    }
    fmt.Println("Part 2: ", part2Result)
}
