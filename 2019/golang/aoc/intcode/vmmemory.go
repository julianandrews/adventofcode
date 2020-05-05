package intcode

type Address uint

type VMMemory struct {
	memory []int64
}

func (memory *VMMemory) resize(size Address) {
	if size >= Address(len(memory.memory)) {
		newMemory := make([]int64, size)
		copy(newMemory, memory.memory)
		memory.memory = newMemory
	}
}

func (memory *VMMemory) Get(address Address) int64 {
	memory.resize(address + 1)
	return memory.memory[address]
}

func (memory *VMMemory) Set(address Address, value int64) {
	memory.resize(address + 1)
	memory.memory[address] = value
}

func (memory *VMMemory) CopySlice(start Address, end Address) []int64 {
	memory.resize(end)
	return append([]int64(nil), memory.memory[start:end]...)
}
