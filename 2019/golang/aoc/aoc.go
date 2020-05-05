package aoc

import (
	"errors"
	"flag"
	"fmt"
	"io/ioutil"
	"julianandrews/adventofcode/aoc/intcode"
	"os"
	"strconv"
	"strings"
)

func OpenInputFile() (*os.File, error) {
	flag.Parse()
	if flag.NArg() == 0 {
		return os.Stdin, nil
	} else if flag.NArg() == 1 {
		return os.Open(flag.Arg(0))
	} else {
		return nil, errors.New("Unexpected arguments")
	}
}

func GetInput() (string, error) {
	pFile, err := OpenInputFile()
	if err != nil {
		return "", err
	}
	data, err := ioutil.ReadAll(pFile)
	return string(data[:]), err
}

func GetInts() ([]int, error) {
	input, err := GetInput()
	if err != nil {
		return nil, err
	}
	lines := strings.Split(strings.TrimSpace(input), "\n")
	ints := make([]int, len(lines))
	for i, line := range lines {
		ints[i], err = strconv.Atoi(line)
		if err != nil {
			return nil, errors.New(fmt.Sprintf("Failed to parse: %v", line))
		}
	}

	return ints, nil
}

func GetIntcodeProgram() ([]int64, error) {
	data, err := GetInput()
	if err != nil {
        return nil, err
	}
	return intcode.ParseProgram(strings.TrimSpace(data))
}
