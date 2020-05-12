package intcode

type mockVM struct {
	inputs  chan int64
	outputs chan int64
}

func NewMockVM(inputs chan int64, outputs chan int64) mockVM {
	return mockVM{inputs: inputs, outputs: outputs}
}

func (vm *mockVM) Inputs() chan<- int64 {
	return vm.inputs
}

func (vm *mockVM) Outputs() <-chan int64 {
	return vm.outputs
}

func (vm *mockVM) Run() error {
	return nil
}
