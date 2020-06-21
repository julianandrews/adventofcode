package main

import (
	"errors"
	"fmt"
	"julianandrews/adventofcode/aoc"
	"julianandrews/adventofcode/aoc/intcode"
	"log"
	"os"
	// "sync"
)

// TODO: Don't depend on a finite buffer
const InputBufferSize = 100

type Packet struct {
	x int64
	y int64
}

type Network struct {
	natChannel    chan Packet
	inputChannels []chan Packet
	idleCheck     chan bool
	idleVMs       []bool
	logger        *log.Logger
}

func NewNetwork(numVMs int, logger *log.Logger) *Network {
	inputChannels := make([]chan Packet, numVMs)
	for address := 0; address < numVMs; address++ {
		inputChannels[address] = make(chan Packet, InputBufferSize)
	}

	return &Network{
		natChannel:    make(chan Packet),
		inputChannels: inputChannels,
		idleCheck:     make(chan bool),
		idleVMs:       make([]bool, numVMs),
		logger:        logger,
	}
}

func (network *Network) sendMessage(destinationAddress int, packet Packet) {
	network.idleVMs[destinationAddress] = false
	(*network).inputChannels[destinationAddress] <- packet
}

func (network *Network) sendToNat(packet Packet) {
	network.natChannel <- packet
}

func getNextMessage(vm intcode.VM) (int64, Packet, error) {
	destinationAddress, ok := <-vm.Outputs()
	if !ok {
		return 0, Packet{}, errors.New("Unexepected end of outputs")
	}
	x, ok := <-vm.Outputs()
	if !ok {
		return 0, Packet{}, errors.New("Unexepected end of outputs")
	}
	y, ok := <-vm.Outputs()
	if !ok {
		return 0, Packet{}, errors.New("Unexepected end of outputs")
	}
	return destinationAddress, Packet{x: x, y: y}, nil
}

func (network *Network) start(program []int64) {
	for address := 0; address < len(network.inputChannels); address++ {
		network.startVM(program, address)
	}
}

func (network *Network) startVM(program []int64, address int) {
	// Buffer the inputs by one so we can send the initial value without blocking.
	// This should also be more efficient since we can sent both x and y packets in one go.
	vmOptions := intcode.VMOptions{InputBufferSize: 1}
	vm := intcode.NewVMWithOptions(append([]int64(nil), program...), vmOptions)

	go func() {
		err := vm.Run()
		if err != nil {
			panic(fmt.Sprintf("Error in VM %d: %s\n", address, err))
		}
	}()

	go func() {
		// This goroutine owns vm.Inputs(). All writes go through here.
		vm.Inputs() <- int64(address)
		for {
			select {
			case packet := <-network.inputChannels[address]:
				vm.Inputs() <- packet.x
				vm.Inputs() <- packet.y
			default:
                select {
                case network.idleCheck <- true:
                default:
                }
                network.idleVMs[address] = true
				vm.Inputs() <- -1
			}
		}
	}()

	go func() {
		// This goroutine owns vm.Outputs(). All reads go through here.
		for {
			destinationAddress, packet, err := getNextMessage(vm)
			if err != nil {
				panic(err)
			}
			if network.logger != nil {
				network.logger.Printf("%d->%d: %d\n", address, destinationAddress, packet)
			}
			if destinationAddress < int64(len(network.inputChannels)) {
				network.sendMessage(int(destinationAddress), packet)
			} else if destinationAddress == 255 {
				network.sendToNat(packet)
			} else {
				panic("Unexpected destination!")
			}
		}
	}()
}

func (network *Network) isIdle() bool {
	for _, isIdle := range network.idleVMs {
		if !isIdle {
			return false
		}
	}
	return true
}

func (network *Network) runNAT(natOutput chan<- Packet) {
	var packet Packet

	go func() {
		count := 0
		for {
			_ = <-network.idleCheck
			if network.isIdle() {
				count++
			} else {
				count = 0
			}
			if count > 10000 {
                count = 0
                if network.logger != nil {
                    network.logger.Printf("255->0: %d\n", packet)
                }
                natOutput <- packet
				network.sendMessage(0, packet)
			}
		}
	}()

	for packet = range network.natChannel {
	}
}

func part1(program []int64) int64 {
	network := NewNetwork(50, nil)
	network.start(program)
	packet := <-network.natChannel
	return packet.y
}

func part2(program []int64) int64 {
	// logger := log.New(os.Stdout, "[Network] ", log.Ltime)
	network := NewNetwork(50, nil)
	network.start(program)
	natOutput := make(chan Packet)
	go network.runNAT(natOutput)

	var lastPacket Packet
	for packet := range natOutput {
		if packet.y == lastPacket.y {
			return packet.y
		}
		lastPacket = packet
	}
	panic("Network terminated with no duplicated packet.")
}

func main() {
	program, err := aoc.GetIntcodeProgram()
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
	fmt.Println("Part 1:", part1(program))
	fmt.Println("Part 2:", part2(program))
}
