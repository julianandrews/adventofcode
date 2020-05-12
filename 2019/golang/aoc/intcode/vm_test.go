package intcode

import (
	"github.com/stretchr/testify/assert"
    "testing"
)

func TestFirstProgram(t *testing.T) {
	program := []int64{1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50}
	expected := []int64{3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50}

    vm := NewVM(program)
    err := vm.Run()

    assert.Nil(t, err)
    assert.Equal(t, expected, vm.Memory())
}

func TestAdd(t *testing.T) {
	program := []int64{1, 0, 0, 0, 99}
	expected := []int64{2, 0, 0, 0, 99}

    vm := NewVM(program)
    err := vm.Run()

    assert.Nil(t, err)
    assert.Equal(t, expected, vm.Memory())
}

func TestMultiply1(t *testing.T) {
	program := []int64{2, 3, 0, 3, 99}
	expected := []int64{2, 3, 0, 6, 99}

    vm := NewVM(program)
    err := vm.Run()

    assert.Nil(t, err)
    assert.Equal(t, expected, vm.Memory())
}

func TestMultiply2(t *testing.T) {
	program := []int64{2, 4, 4, 5, 99, 0}
	expected := []int64{2, 4, 4, 5, 99, 9801}

    vm := NewVM(program)
    err := vm.Run()

    assert.Nil(t, err)
    assert.Equal(t, expected, vm.Memory())
}

func TestAddThenMultiply(t *testing.T) {
	program := []int64{1,1,1,4,99,5,6,0,99}
	expected := []int64{30,1,1,4,2,5,6,0,99}

    vm := NewVM(program)
    err := vm.Run()

    assert.Nil(t, err)
    assert.Equal(t, expected, vm.Memory())
}

func TestImmediateMultiply(t *testing.T) {
	program := []int64{1002, 4, 3, 4, 33}
	expected := []int64{1002, 4, 3, 4, 99}

	vm := NewVM(program)
    err := vm.Run()

    assert.Nil(t, err)
	assert.Equal(t, expected, vm.Memory())
}

func TestEquals(t *testing.T) {
	program := []int64{3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8}

	firstVM := NewVM(append([]int64(nil), program...))
	go firstVM.Run()
	firstVM.Inputs() <- 8
	firstVMOutput := <-firstVM.Outputs()

	secondVM := NewVM(append([]int64(nil), program...))
	go secondVM.Run()
	secondVM.Inputs() <- 7
    secondVMOutput := <-secondVM.Outputs()

	assert.Equal(t, int64(1), firstVMOutput)
	assert.Equal(t, int64(0), secondVMOutput)
}

func TestLessThan(t *testing.T) {
	program := []int64{3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8}

	firstVM := NewVM(append([]int64(nil), program...))
	go firstVM.Run()
	firstVM.Inputs() <- 7
	firstVMOutput := <-firstVM.Outputs()

	secondVM := NewVM(append([]int64(nil), program...))
	go secondVM.Run()
	secondVM.Inputs() <- 8
    secondVMOutput := <-secondVM.Outputs()

	assert.Equal(t, int64(1), firstVMOutput)
	assert.Equal(t, int64(0), secondVMOutput)
}

func TestEqualsImmediate(t *testing.T) {
	program := []int64{3, 3, 1108, -1, 8, 3, 4, 3, 99}

	firstVM := NewVM(append([]int64(nil), program...))
	go firstVM.Run()
	firstVM.Inputs() <- 8
	firstVMOutput := <-firstVM.Outputs()

	secondVM := NewVM(append([]int64(nil), program...))
	go secondVM.Run()
	secondVM.Inputs() <- 10
    secondVMOutput := <-secondVM.Outputs()

	assert.Equal(t, int64(1), firstVMOutput)
	assert.Equal(t, int64(0), secondVMOutput)
}

func TestLessThanImmediate(t *testing.T) {
	program := []int64{3, 3, 1107, -1, 8, 3, 4, 3, 99}

	firstVM := NewVM(append([]int64(nil), program...))
	go firstVM.Run()
	firstVM.Inputs() <- 7
	firstVMOutput := <-firstVM.Outputs()

	secondVM := NewVM(append([]int64(nil), program...))
	go secondVM.Run()
	secondVM.Inputs() <- 10
    secondVMOutput := <-secondVM.Outputs()

	assert.Equal(t, int64(1), firstVMOutput)
	assert.Equal(t, int64(0), secondVMOutput)
}

func TestLongerProgram(t *testing.T) {
	program := []int64{3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20,
		1006, 20, 31, 1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1,
		46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99}

	firstVM := NewVM(append([]int64(nil), program...))
	go firstVM.Run()
	firstVM.Inputs() <- 7
	firstVMOutput := <-firstVM.Outputs()

	secondVM := NewVM(append([]int64(nil), program...))
	go secondVM.Run()
	secondVM.Inputs() <- 8
    secondVMOutput := <-secondVM.Outputs()

	thirdVM := NewVM(append([]int64(nil), program...))
	go thirdVM.Run()
	thirdVM.Inputs() <- 10
    thirdVMOutput := <-thirdVM.Outputs()

	assert.Equal(t, int64(999), firstVMOutput)
	assert.Equal(t, int64(1000), secondVMOutput)
	assert.Equal(t, int64(1001), thirdVMOutput)
}

func TestQuine(t *testing.T) {
	program := []int64{
		109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
	}
    expected := append([]int64(nil), program...)

	vm := NewVM(program)
    go vm.Run()
	var outputs []int64
	for output := range vm.Outputs() {
		outputs = append(outputs, output)
	}

	assert.Equal(t, expected, outputs)
}

func TestBigMultiply(t *testing.T) {
	program := []int64{1102, 34915192, 34915192, 7, 4, 7, 99, 0}
    expected := int64(34915192) * int64(34915192)

	vm := NewVM(program)
    go vm.Run()
    output := <- vm.Outputs()

	assert.Equal(t, expected, output)
}

func TestOutputsMiddle(t *testing.T) {
	program := []int64{104, 1125899906842624, 99}
    expected := program[1]

	vm := NewVM(program)
    go vm.Run()
    output := <- vm.Outputs()

	assert.Equal(t, expected, output)
}
