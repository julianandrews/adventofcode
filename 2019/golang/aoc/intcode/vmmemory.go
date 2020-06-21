package intcode

type vmAddress uint64

type vmMemory struct {
	memory []int64
}

func (memory *vmMemory) resize(size vmAddress) {
	if size >= vmAddress(len(memory.memory)) {
		newMemory := make([]int64, size)
		copy(newMemory, memory.memory)
		memory.memory = newMemory
	}
}

func (memory *vmMemory) get(address vmAddress) int64 {
	memory.resize(address + 1)
	return memory.memory[address]
}

func (memory *vmMemory) set(address vmAddress, value int64) {
	memory.resize(address + 1)
	memory.memory[address] = value
}
