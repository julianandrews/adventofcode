package intcode

import (
	"errors"
	"fmt"
	"strconv"
	"strings"
)

func ParseProgram(input string) ([]int64, error) {
	parts := strings.Split(strings.TrimSpace(input), ",")
	values := make([]int64, len(parts))
	for i, part := range parts {
		value, err := strconv.ParseInt(part, 10, 64)
		if err != nil {
			return nil, errors.New(fmt.Sprintf("Failed to parse: %v", part))
		}
		values[i] = int64(value)
	}

	return values, nil
}
