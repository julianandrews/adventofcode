package intcode

import (
    "errors"
)

type OpType int64
type ValueMode int64

const (
	VALUEMODE_POSITION  = 0
	VALUEMODE_IMMEDIATE = 1
	VALUEMODE_RELATIVE  = 2
)

const (
	OP_ADD               = 1
	OP_MULTIPLY          = 2
	OP_STORE             = 3
	OP_OUTPUT            = 4
	OP_JUMP_IF_TRUE      = 5
	OP_JUMP_IF_FALSE     = 6
	OP_LESS_THAN         = 7
	OP_EQUALS            = 8
	OP_ADJUST_REL_OFFSET = 9
	OP_HALT              = 99
)

func (opType OpType) numParams() (int, error) {
	switch opType {
	case OP_ADD, OP_MULTIPLY, OP_LESS_THAN, OP_EQUALS:
		return 3, nil
	case OP_JUMP_IF_FALSE, OP_JUMP_IF_TRUE:
		return 2, nil
	case OP_STORE, OP_OUTPUT, OP_ADJUST_REL_OFFSET:
		return 1, nil
	case OP_HALT:
		return 0, nil
	default:
		return 0, errors.New("Unrecognized operation")
	}
}
