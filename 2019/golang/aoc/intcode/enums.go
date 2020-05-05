package intcode

import (
	"errors"
)

type opType int64
type valueMode int64

const (
	valueModePosition  = 0
	valueModeImmediate = 1
	valueModeRelative  = 2
)

const (
	opTypeAdd             = 1
	opTypeMultiply        = 2
	opTypeStore           = 3
	opTypeOutput          = 4
	opTypeJumpIfTrue      = 5
	opTypeJumpIfFalse     = 6
	opTypeLessThan        = 7
	opTypeEquals          = 8
	opTypeAdjustRelOffset = 9
	opTypeHalt            = 99
)

func (opType opType) numParams() (int, error) {
	switch opType {
	case opTypeAdd, opTypeMultiply, opTypeLessThan, opTypeEquals:
		return 3, nil
	case opTypeJumpIfFalse, opTypeJumpIfTrue:
		return 2, nil
	case opTypeStore, opTypeOutput, opTypeAdjustRelOffset:
		return 1, nil
	case opTypeHalt:
		return 0, nil
	default:
		return 0, errors.New("Unrecognized operation")
	}
}
