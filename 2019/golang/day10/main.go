package main

import (
	"bufio"
	"errors"
	"fmt"
	"julianandrews/adventofcode/aoc"
    "math"
	"os"
    "sort"
	"strings"
)

type AsteroidField [][]bool

type Point struct {
	x int
	y int
}

func (asteroidField AsteroidField) height() int {
	return len(asteroidField)
}

func (asteroidField AsteroidField) width() int {
	if len(asteroidField) > 0 {
		return len(asteroidField[0])
	} else {
		return 0
	}
}

func (asteroidField AsteroidField) firstVisibleAsteroid(point Point, direction Point) (Point, bool) {
	for {
		point.x += direction.x
		point.y += direction.y
		if point.x < 0 ||
			point.x >= asteroidField.width() ||
			point.y < 0 ||
			point.y >= asteroidField.height() {
			return point, false
		} else if asteroidField[point.y][point.x] {
			return point, true
		}
	}
}

func gcd(x, y int) int {
    if x < 0 {
        x = -x
    }
    if y < 0 {
        y = -y
    }
    for y != 0 {
        x, y = y, x%y
    }
    return x
}

func (asteroidField AsteroidField) getDirections(point Point) []Point {
    directionSet := make(map[Point]bool)
    for x := 0; x < asteroidField.width(); x++ {
        dx := x - point.x
        for y := 0; y < asteroidField.height(); y++ {
            dy := y - point.y
            if dx != 0 || dy != 0 {
                denom := gcd(dx, dy)
                p := Point{x: dx / denom, y: dy / denom}
                directionSet[p] = true
            }
        }
    }

    directions := make([]Point, 0, len(directionSet))
    for k := range directionSet {
        directions = append(directions, k)
    }
    sortKey := func(p Point) float64 {
        val := math.Atan2(float64(p.y), float64(p.x)) + math.Pi / 2
        if val < 0 {
            val += 2 * math.Pi
        }
        return val
    }
    sort.Slice(directions, func(i, j int) bool {
        return sortKey(directions[i]) < sortKey(directions[j])
    })
    return directions
}

func (asteroidField AsteroidField) visibleCount(point Point) int {
    count := 0
    for _, direction := range asteroidField.getDirections(point) {
        _, ok := asteroidField.firstVisibleAsteroid(point, direction);
        if ok {
            count++
        }
    }
	return count
}

func (asteroidField AsteroidField) monitoringStation() Point {
	bestCount := 0
	var bestPoint Point
	for y, row := range asteroidField {
		for x := range row {
            if asteroidField[y][x] {
                point := Point{x: x, y: y}
                count := asteroidField.visibleCount(point)
                if count > bestCount {
                    bestCount = count
                    bestPoint = point
                }
            }
		}
	}
	return bestPoint
}

func (asteroidField AsteroidField) nthAsteroid(point Point, n int) Point {
    count := 0
    directions := asteroidField.getDirections(point)
    for i := 0; true; i = (i + 1) % len(directions) {
        asteroidCoords, ok := asteroidField.firstVisibleAsteroid(point, directions[i])
        if ok {
            asteroidField[asteroidCoords.y][asteroidCoords.x] = false
            count += 1
            if count == n {
                return asteroidCoords
            }
        }
    }
    panic("Should never happen")
}

func parseAsteroidField(scanner *bufio.Scanner) (AsteroidField, error) {
	var asteroidField AsteroidField
    width := -1
	for scanner.Scan() {
		line := scanner.Text()
		row := make([]bool, len(line))
		for x, r := range line {
			switch r {
			case '#':
				row[x] = true
			case '.':
				row[x] = false
			default:
				return nil, errors.New("Unrecognized asteroid field tile")
			}
		}
        if width == -1 {
            width = len(row)
        } else if width != len(row) {
            return nil, errors.New("Non-rectangular asteroid field")
        }
		asteroidField = append(asteroidField, row)
	}

	return asteroidField, nil
}

func (asteroidField AsteroidField) String() string {
	var builder strings.Builder
	for y, row := range asteroidField {
		builder.Grow(len(row))
		for _, v := range row {
			if v {
				builder.WriteRune('#')
			} else {
				builder.WriteRune('.')
			}
		}
		if y < len(asteroidField)-1 {
			builder.WriteRune('\n')
		}
	}

	return builder.String()
}

func part1(asteroidField AsteroidField) int {
	monitoringStation := asteroidField.monitoringStation()
	return asteroidField.visibleCount(monitoringStation)
}

func part2(asteroidField AsteroidField) int {
    coords := asteroidField.nthAsteroid(asteroidField.monitoringStation(), 200)
    return 100 * coords.x + coords.y
}

func main() {
	pFile, err := aoc.OpenInputFile()
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
	asteroidField, err := parseAsteroidField(bufio.NewScanner(pFile))
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
	fmt.Println("Part 1: ", part1(asteroidField))
	fmt.Println("Part 2: ", part2(asteroidField))
}
