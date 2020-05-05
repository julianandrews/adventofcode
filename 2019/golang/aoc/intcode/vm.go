package intcode

import (
	"errors"
	"fmt"
)

type VM struct {
	Memory        VMMemory
	inputs        chan int64
	outputs       chan int64
	ip            Address
	relative_base int64
}

func New(program []int64) VM {
	var vm VM
	vm.Memory.memory = program
	vm.inputs = make(chan int64)
	vm.outputs = make(chan int64)

	return vm
}

func (vm *VM) Inputs() chan<- int64 {
    return vm.inputs
}

func (vm *VM) Outputs() <-chan int64 {
    return vm.outputs
}

func (vm *VM) Run() error {
	for {
		opType, _, err := vm.Step()
		if err != nil {
			return err
		} else if opType == OP_HALT {
			return nil
		}
	}
}

func (vm *VM) Step() (OpType, []int64, error) {
	opType := vm.getOpType()
	numParams, err := opType.numParams()
	if err != nil {
		return 0, nil, err
	}
	params := vm.getParams(numParams)
	modes, err := vm.getModes(numParams)
	if err != nil {
		return 0, nil, err
	}

	switch opType {
	case OP_ADD:
		vm.add(params, modes)
	case OP_MULTIPLY:
		vm.multiply(params, modes)
	case OP_STORE:
		vm.store(params, modes)
	case OP_OUTPUT:
		vm.output(params, modes)
	case OP_JUMP_IF_TRUE:
		vm.jumpIfTrue(params, modes)
	case OP_JUMP_IF_FALSE:
		vm.jumpIfFalse(params, modes)
	case OP_LESS_THAN:
		vm.lessThan(params, modes)
	case OP_EQUALS:
		vm.equals(params, modes)
	case OP_ADJUST_REL_OFFSET:
		vm.adjustRelOffset(params, modes)
	case OP_HALT:
        close(vm.outputs)
		break
	default:
		return 0, nil, errors.New("Unrecognized operation")
	}

	return opType, params, nil
}

func (vm *VM) getOpType() OpType {
	return OpType(vm.Memory.Get(vm.ip) % 100)
}

func (vm *VM) getParams(numParams int) []int64 {
	return vm.Memory.CopySlice(vm.ip+Address(1), vm.ip+Address(1+numParams))
}

func (vm *VM) getModes(numParams int) ([]ValueMode, error) {
	modes := make([]ValueMode, numParams)
	x := vm.Memory.Get(vm.ip) / 100
	for i := range modes {
		mode := x % 10
		if mode > 2 {
			return nil, errors.New("Unrecognized mode")
		}
		modes[i] = ValueMode(mode)
		x = x / 10
	}
	return modes, nil
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

func (vm *VM) binary_operands(params []int64, modes []ValueMode) (int64, int64, Address, error) {
	if len(params) != 3 {
		mess := fmt.Sprintf("Unexpected number of params %v for binary operation", len(params))
		return 0, 0, 0, errors.New(mess)
	}
	if len(modes) != 3 {
		mess := fmt.Sprintf("Unexpected number of modes %v for binary operation", len(modes))
		return 0, 0, 0, errors.New(mess)
	}
	if modes[2] == VALUEMODE_IMMEDIATE {
		mess := fmt.Sprintf("Unexpected mode %v for binary operation", modes[2])
		return 0, 0, 0, errors.New(mess)
	}
	a, err := vm.getValue(params[0], modes[0])
	if err != nil {
		return 0, 0, 0, err
	}
	b, err := vm.getValue(params[1], modes[1])
	if err != nil {
		return 0, 0, 0, err
	}
	address, err := vm.getAddress(params[2], modes[2])
	if err != nil {
		return 0, 0, 0, err
	}
	return a, b, address, nil
}

func (vm *VM) jump_operands(params []int64, modes []ValueMode) (int64, Address, error) {
	if len(params) != 2 {
		mess := fmt.Sprintf("Unexpected number of params %v for jump operation", len(params))
		return 0, 0, errors.New(mess)
	}
	if len(modes) != 2 {
		mess := fmt.Sprintf("Unexpected number of modes %v for jump operation", len(modes))
		return 0, 0, errors.New(mess)
	}
	a, err := vm.getValue(params[0], modes[0])
	if err != nil {
		return 0, 0, err
	}
	b, err := vm.getValue(params[1], modes[1])
	if err != nil {
		return 0, 0, err
	}
	if b < 0 {
		return 0, 0, errors.New("Invalid negative address")
	}
	return a, Address(b), nil
}

func (vm *VM) add(params []int64, modes []ValueMode) error {
	a, b, address, err := vm.binary_operands(params, modes)
	if err != nil {
		return err
	}
	vm.Memory.Set(address, a+b)
	vm.ip += Address(len(params) + 1)
	return nil
}

func (vm *VM) multiply(params []int64, modes []ValueMode) error {
	a, b, address, err := vm.binary_operands(params, modes)
	if err != nil {
		return err
	}
	vm.Memory.Set(address, a*b)
	vm.ip += Address(len(params) + 1)
	return nil
}

func (vm *VM) store(params []int64, modes []ValueMode) error {
	if len(params) != 1 || len(modes) != 1 {
		return errors.New("Incorrect number of parameters or modes")
	}
	if modes[0] == VALUEMODE_IMMEDIATE {
		return errors.New("Unexpected VALUEMODE_IMMEDIATE for store")
	}
	address, err := vm.getAddress((params)[0], modes[0])
	if err != nil {
		return err
	}
	value := <-vm.inputs
	vm.Memory.Set(address, value)
	vm.ip += Address(len(params) + 1)
	return nil
}

func (vm *VM) output(params []int64, modes []ValueMode) error {
	if len(params) != 1 || len(modes) != 1 {
		return errors.New("Incorrect number of parameters or modes")
	}
	value, err := vm.getValue(params[0], modes[0])
	if err != nil {
		return err
	}
	vm.outputs <- value
	vm.ip += Address(len(params) + 1)
	return nil
}

func (vm *VM) jumpIfTrue(params []int64, modes []ValueMode) error {
	a, address, err := vm.jump_operands(params, modes)
	if err != nil {
		return err
	}
	if a != 0 {
		vm.ip = address
	} else {
		vm.ip += Address(len(params) + 1)
	}

	return nil
}

func (vm *VM) jumpIfFalse(params []int64, modes []ValueMode) error {
	a, address, err := vm.jump_operands(params, modes)
	if err != nil {
		return err
	}
	if a == 0 {
		vm.ip = address
	} else {
		vm.ip += Address(len(params) + 1)
	}

	return nil
}

func (vm *VM) lessThan(params []int64, modes []ValueMode) error {
	a, b, address, err := vm.binary_operands(params, modes)
	if err != nil {
		return err
	}
	if a < b {
		vm.Memory.Set(address, 1)
	} else {
		vm.Memory.Set(address, 0)
	}
	vm.ip += Address(len(params) + 1)
	return nil
}

func (vm *VM) equals(params []int64, modes []ValueMode) error {
	a, b, address, err := vm.binary_operands(params, modes)
	if err != nil {
		return err
	}
	if a == b {
		vm.Memory.Set(address, 1)
	} else {
		vm.Memory.Set(address, 0)
	}
	vm.ip += Address(len(params) + 1)
	return nil
}

func (vm *VM) adjustRelOffset(params []int64, modes []ValueMode) error {
	// TODO
	return errors.New("Unimplemented")
}
