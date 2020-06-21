package main

import (
	"fmt"
	"julianandrews/adventofcode/aoc"
	"julianandrews/adventofcode/aoc/intcode"
	"os"
	"strings"
)

type Direction int

const (
	north = Direction(iota)
	east  = Direction(iota)
	south = Direction(iota)
	west  = Direction(iota)
)

type Panel map[Point]bool

func (panel Panel) String() string {
	if len(panel) == 0 {
		return ""
	}
	var minX, maxX, minY, maxY int64
	for k := range panel {
		minX = k.x
		maxX = k.x
		minY = k.y
		maxY = k.y
		break
	}
	for k := range panel {
		if k.x < minX {
			minX = k.x
		}
		if k.x > maxX {
			maxX = k.x
		}
		if k.y < minY {
			minY = k.y
		}
		if k.y > maxY {
			maxY = k.y
		}
	}
	var builder strings.Builder
	builder.Grow(int((maxY - minY) * (maxX - minX + 1) - 1))
	for y := maxY; y >= minY; y-- {
		for x := minX; x <= maxX; x++ {
			if panel[Point{x: x, y: y}] {
				builder.WriteRune('█')
			} else {
				builder.WriteRune(' ')
			}
		}
		if y > minY {
			builder.WriteRune('\n')
		}
	}
	return builder.String()
}

type Point struct {
	x int64
	y int64
}

type PaintStep struct {
	paintLocation Point
	paintWhite    bool
}

func NewPaintStep(x int64, y int64, paintWhite bool) PaintStep {
	return PaintStep{
		paintLocation: Point{x: x, y: y},
		paintWhite:    paintWhite,
	}
}

func (d Direction) turnRight() Direction {
	return (d + 1) % 4
}

func (d Direction) turnLeft() Direction {
	return (d + 3) % 4
}

func (p Point) move(d Direction) Point {
	switch d {
	case north:
		return Point{x: p.x, y: p.y + 1}
	case east:
		return Point{x: p.x + 1, y: p.y}
	case south:
		return Point{x: p.x, y: p.y - 1}
	case west:
		return Point{x: p.x - 1, y: p.y}
	default:
		panic("Unrecognized direction")
	}
}

func paintPanel(vm intcode.VM, paintedPanels *Panel) <-chan PaintStep {
	ch := make(chan PaintStep)

	go func() {
		go vm.Run()
		p := Point{x: 0, y: 0}
		d := north
		for {
			go func(p Point) {
				if (*paintedPanels)[p] {
					vm.Inputs() <- 1
				} else {
					vm.Inputs() <- 0
				}
			}(p)
			paintWhite, ok := <-vm.Outputs()
			if !ok {
				break
			}
			turnRight, ok := <-vm.Outputs()
			if !ok {
				panic("Failed on second output!")
			}
			if paintWhite != 0 {
				(*paintedPanels)[p] = true
			} else {
				delete((*paintedPanels), p)
			}
			if turnRight != 0 {
				d = d.turnRight()
			} else {
				d = d.turnLeft()
			}
			ch <- NewPaintStep(p.x, p.y, paintWhite != 0)
			p = p.move(d)
		}
		close(ch)
	}()

	return ch
}

func part1(program []int64) int {
	panel := make(Panel)
	everPainted := make(Panel)
	vm := intcode.NewVM(append([]int64(nil), program...))
	for step := range paintPanel(vm, &panel) {
		if step.paintWhite {
			everPainted[step.paintLocation] = true
		}
	}

	return len(everPainted)
}

func part2(program []int64) string {
	panel := make(Panel)
	panel[Point{x: 0, y: 0}] = true
	vm := intcode.NewVM(append([]int64(nil), program...))
	for range paintPanel(vm, &panel) {
	}
	return panel.String()
}

func main() {
	program, err := aoc.GetIntcodeProgram()
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
	fmt.Println("Part 1: ", part1(program))
	fmt.Printf("Part 2:\n%v\n", part2(program))
}
