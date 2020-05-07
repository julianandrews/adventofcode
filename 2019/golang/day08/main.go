package main

import (
    "errors"
    "fmt"
    "julianandrews/adventofcode/aoc"
    "os"
    "strings"
)

type Image struct {
    pixels []int
    width int
    height int
}

func (image Image) countLayerDigit(layer int, digit int) int {
    imageSize := image.width * image.height
    count := 0
    for i := layer * imageSize; i < (layer + 1) * imageSize; i++ {
        if image.pixels[i] == digit {
            count++
        }
    }
    return count
}

func (image Image) getPixel(x int, y int) rune {
    base := image.width * y + x
    offset := image.width * image.height
    for index := base; index < len(image.pixels); index += offset {
        value := image.pixels[index]
        switch value {
        case 0:
            return ' '
        case 1:
            return '█'
        case 2:
        default:
            return rune(value)
        }
    }
    return '░'
}

func (image Image) String() string {
    var builder strings.Builder
    builder.Grow(((image.width + 1) * image.height - 1) * 4)
    for y := 0; y < image.height; y++ {
        for x := 0; x < image.width; x++ {
            builder.WriteRune(image.getPixel(x, y))
        }
        if y < image.height - 1 {
            builder.WriteRune('\n')
        }
    }
    return builder.String()
}

func parseImage(input string, width int, height int) (Image, error) {
    pixels := make([]int, len(input))
    for i, r := range input {
        if r < '0' || r > '9' {
            return Image{}, errors.New("Invalid character for image")
        }
        pixels[i] = int(r - '0')
    }

    return Image{pixels: pixels, width: width, height: height}, nil
}

func part1(image Image) int {
    imageSize := image.width * image.height
    bestLayer := 0
    bestCount := imageSize
    for layer := 0; layer * imageSize < len(image.pixels); layer++ {
        zeroCount := image.countLayerDigit(layer, 0)
        if zeroCount < bestCount {
            bestCount = zeroCount
            bestLayer = layer
        }
    }
    return image.countLayerDigit(bestLayer, 2) * image.countLayerDigit(bestLayer, 1)
}

func main() {
    input, err := aoc.GetInput()
    if err != nil {
        fmt.Fprintln(os.Stderr, "Failed to read input")
        os.Exit(1)
    }
    image, err := parseImage(strings.TrimSpace(input), 25, 6)
    if err != nil {
        fmt.Fprintln(os.Stderr, "Failed to parse input")
        os.Exit(1)
    }

    fmt.Println("Part 1: ", part1(image))
    fmt.Printf("Part 2:\n%v\n", image)
}
