package intcode

import (
	"errors"
)

type VM struct {
	memory       vmMemory
	inputs       chan int64
	outputs      chan int64
	ip           vmAddress
	relativeBase int64
}

func New(program []int64) VM {
	var vm VM
	vm.memory.memory = program
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

func (vm *VM) DiagnosticCode() int64 {
	return vm.memory.get(0)
}

func (vm *VM) Snapshot() []int64 {
	return append([]int64(nil), vm.memory.memory...)
}

func (vm *VM) Run() error {
	for {
		running, err := vm.step()
		if err != nil {
			return err
		} else if !running {
			return nil
		}
	}
}

func (vm *VM) step() (bool, error) {
	opType := vm.getOpType()
	numParams, err := opType.numParams()
	if err != nil {
		return false, err
	}
	params := vm.getParams(numParams)
	modes, err := vm.getModes(numParams)
	if err != nil {
		return false, err
	}

	switch opType {
	case opTypeAdd:
		vm.add(params, modes)
	case opTypeMultiply:
		vm.multiply(params, modes)
	case opTypeStore:
		vm.store(params, modes)
	case opTypeOutput:
		vm.output(params, modes)
	case opTypeJumpIfTrue:
		vm.jumpIfTrue(params, modes)
	case opTypeJumpIfFalse:
		vm.jumpIfFalse(params, modes)
	case opTypeLessThan:
		vm.lessThan(params, modes)
	case opTypeEquals:
		vm.equals(params, modes)
	case opTypeAdjustRelOffset:
		vm.adjustRelOffset(params, modes)
	case opTypeHalt:
		close(vm.outputs)
		return false, nil
	default:
		return false, errors.New("Unrecognized operation")
	}

	return true, nil
}

func (vm *VM) getOpType() opType {
	return opType(vm.memory.get(vm.ip) % 100)
}

func (vm *VM) getParams(numParams int) []int64 {
	return append([]int64(nil), vm.memory.memory[vm.ip+vmAddress(1):vm.ip+vmAddress(1+numParams)]...)
}

func (vm *VM) getModes(numParams int) ([]valueMode, error) {
	modes := make([]valueMode, numParams)
	x := vm.memory.get(vm.ip) / 100
	for i := range modes {
		mode := x % 10
		switch mode {
		case valueModeImmediate, valueModePosition, valueModeRelative:
			modes[i] = valueMode(mode)
		default:
			return nil, errors.New("Unrecognized valueMode")
		}
		x = x / 10
	}
	return modes, nil
}

func (vm *VM) getAddress(baseAddress int64, mode valueMode) (vmAddress, error) {
	switch mode {
	case valueModePosition:
		if baseAddress < 0 {
			return 0, errors.New("Invalid address")
		}
		return vmAddress(baseAddress), nil
	case valueModeImmediate, valueModeRelative:
		address := vm.relativeBase + baseAddress
		if address < 0 {
			return 0, errors.New("Invalid address")
		}
		return vmAddress(address), nil
	default:
		return 0, errors.New("Unrecognized value mode")
	}
}

func (vm *VM) getValue(value int64, mode valueMode) (int64, error) {
	switch mode {
	case valueModePosition:
		if value < 0 {
			return 0, errors.New("Invalid address")
		}
		return vm.memory.get(vmAddress(value)), nil
	case valueModeImmediate:
		return value, nil
	case valueModeRelative:
		address := vm.relativeBase + value
		if address < 0 {
			return 0, errors.New("Invalid address")
		}
		return vm.memory.get(vmAddress(address)), nil
	default:
		return 0, errors.New("Unrecognized value mode")
	}
}

func (vm *VM) binaryOperands(params []int64, modes []valueMode) (int64, int64, vmAddress, error) {
	if len(params) != 3 || len(modes) != 3 {
		return 0, 0, 0, errors.New("Incorrect number of parameters or modes")
	}
	if modes[2] == valueModeImmediate {
		return 0, 0, 0, errors.New("Unexpected valueModeImmediate for binary operation")
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

func (vm *VM) jumpOperands(params []int64, modes []valueMode) (int64, vmAddress, error) {
	if len(params) != 2 || len(modes) != 2 {
		return 0, 0, errors.New("Incorrect number of parameters or modes")
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
		return 0, 0, errors.New("Invalid address")
	}
	return a, vmAddress(b), nil
}

func (vm *VM) add(params []int64, modes []valueMode) error {
	a, b, address, err := vm.binaryOperands(params, modes)
	if err != nil {
		return err
	}
	vm.memory.set(address, a+b)
	vm.ip += vmAddress(len(params) + 1)
	return nil
}

func (vm *VM) multiply(params []int64, modes []valueMode) error {
	a, b, address, err := vm.binaryOperands(params, modes)
	if err != nil {
		return err
	}
	vm.memory.set(address, a*b)
	vm.ip += vmAddress(len(params) + 1)
	return nil
}

func (vm *VM) store(params []int64, modes []valueMode) error {
	if len(params) != 1 || len(modes) != 1 {
		return errors.New("Incorrect number of parameters or modes")
	}
	if modes[0] == valueModeImmediate {
		return errors.New("Unexpected valueModeImmediate for opTypeStore")
	}
	address, err := vm.getAddress((params)[0], modes[0])
	if err != nil {
		return err
	}
	value := <-vm.inputs
	vm.memory.set(address, value)
	vm.ip += vmAddress(len(params) + 1)
	return nil
}

func (vm *VM) output(params []int64, modes []valueMode) error {
	if len(params) != 1 || len(modes) != 1 {
		return errors.New("Incorrect number of parameters or modes")
	}
	value, err := vm.getValue(params[0], modes[0])
	if err != nil {
		return err
	}
	vm.outputs <- value
	vm.ip += vmAddress(len(params) + 1)
	return nil
}

func (vm *VM) jumpIfTrue(params []int64, modes []valueMode) error {
	a, address, err := vm.jumpOperands(params, modes)
	if err != nil {
		return err
	}
	if a != 0 {
		vm.ip = address
	} else {
		vm.ip += vmAddress(len(params) + 1)
	}

	return nil
}

func (vm *VM) jumpIfFalse(params []int64, modes []valueMode) error {
	a, address, err := vm.jumpOperands(params, modes)
	if err != nil {
		return err
	}
	if a == 0 {
		vm.ip = address
	} else {
		vm.ip += vmAddress(len(params) + 1)
	}

	return nil
}

func (vm *VM) lessThan(params []int64, modes []valueMode) error {
	a, b, address, err := vm.binaryOperands(params, modes)
	if err != nil {
		return err
	}
	if a < b {
		vm.memory.set(address, 1)
	} else {
		vm.memory.set(address, 0)
	}
	vm.ip += vmAddress(len(params) + 1)
	return nil
}

func (vm *VM) equals(params []int64, modes []valueMode) error {
	a, b, address, err := vm.binaryOperands(params, modes)
	if err != nil {
		return err
	}
	if a == b {
		vm.memory.set(address, 1)
	} else {
		vm.memory.set(address, 0)
	}
	vm.ip += vmAddress(len(params) + 1)
	return nil
}

func (vm *VM) adjustRelOffset(params []int64, modes []valueMode) error {
	if len(params) != 1 || len(modes) != 1 {
		return errors.New("Incorrect number of parameters or modes")
	}
	value, err := vm.getValue(params[0], modes[0])
	if err != nil {
		return err
	}
	vm.relativeBase = vm.relativeBase + value
	vm.ip += vmAddress(len(params) + 1)

	return nil
}
