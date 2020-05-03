package intcode

import (
	"errors"
	"fmt"
)

type VM struct {
	Memory        VMMemory
	ip            Address
	relative_base int64
	inputs        chan int64
	outputs       chan int64
}

func New(program []int64) VM {
	var vm VM
	vm.Memory.memory = program

	return vm
}

func (vm *VM) Step() (OpType, []int64, error) {
	opType := vm.getOpType()
	numParams, err := opType.numParams()
	if err != nil {
		return 0, nil, err
	}
	params := vm.getParams(numParams)
	modes := vm.getModes(numParams)

	// TODO: implement remaining methods
	switch opType {
	case OP_ADD:
		vm.add(&params, &modes)
	case OP_MULTIPLY:
		vm.multiply(&params, &modes)
	case OP_HALT:
		break
	default:
		return 0, nil, errors.New("Unimplemented operation")
	}

	return opType, params, nil
}

func (vm *VM) getOpType() OpType {
	return OpType(vm.Memory.Get(vm.ip))
}

func (vm *VM) getParams(numParams int) []int64 {
	return vm.Memory.CopySlice(vm.ip+Address(1), vm.ip+Address(1+numParams))
}

func (vm *VM) getModes(numParams int) []ValueMode {
	// TODO: implement getModes
	modes := make([]ValueMode, numParams)
	return modes
}

func (vm *VM) getAddress(baseAddress int64, mode ValueMode) (Address, error) {
	switch mode {
	case VALUEMODE_POSITION:
		if baseAddress < 0 {
			return 0, errors.New("Negative address not allowed")
		}
		return Address(baseAddress), nil
	case VALUEMODE_IMMEDIATE, VALUEMODE_RELATIVE:
		address := vm.relative_base + baseAddress
		if address < 0 {
			return 0, errors.New("Negative address not allowed")
		}
		return Address(address), nil
	default:
		return 0, errors.New("Unrecognized value mode")
	}
}

func (vm *VM) getValue(value int64, mode ValueMode) (int64, error) {
	switch mode {
	case VALUEMODE_POSITION:
		if value < 0 {
			return 0, errors.New("Negative address not allowed")
		}
		return vm.Memory.Get(Address(value)), nil
	case VALUEMODE_IMMEDIATE:
		return value, nil
	case VALUEMODE_RELATIVE:
		address := vm.relative_base + value
		if address < 0 {
			return 0, errors.New("Negative address not allowed")
		}
		return vm.Memory.Get(Address(address)), nil
	default:
		return 0, errors.New("Unrecognized value mode")
	}
}

func (vm *VM) binary_operands(params *[]int64, modes *[]ValueMode) (int64, int64, Address, error) {
	if len(*params) != 3 {
		mess := fmt.Sprintf("Unexpected number of params %v for binary operation", len(*params))
		return 0, 0, 0, errors.New(mess)
	}
	if len(*modes) != 3 {
		mess := fmt.Sprintf("Unexpected number of modes %v for binary operation", len(*modes))
		return 0, 0, 0, errors.New(mess)
	}
	if (*modes)[2] == VALUEMODE_IMMEDIATE {
		mess := fmt.Sprintf("Unexpected mode %v for binary operation", (*modes)[2])
		return 0, 0, 0, errors.New(mess)
	}
	a, err := vm.getValue((*params)[0], (*modes)[0])
	if err != nil {
		return 0, 0, 0, err
	}
	b, err := vm.getValue((*params)[1], (*modes)[1])
	if err != nil {
		return 0, 0, 0, err
	}
	address, err := vm.getAddress((*params)[2], (*modes)[2])
	if err != nil {
		return 0, 0, 0, err
	}
	return a, b, address, nil
}

func (vm *VM) add(params *[]int64, modes *[]ValueMode) error {
	a, b, address, err := vm.binary_operands(params, modes)
	if err != nil {
		return err
	}
	vm.Memory.Set(address, a+b)
	vm.ip += Address(len(*params) + 1)
	return nil
}

func (vm *VM) multiply(params *[]int64, modes *[]ValueMode) error {
	a, b, address, err := vm.binary_operands(params, modes)
	if err != nil {
		return err
	}
	vm.Memory.Set(address, a*b)
	vm.ip += Address(len(*params) + 1)
	return nil
}
