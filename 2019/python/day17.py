import re

from utils import read_data
from utils.direction import Direction
from utils.intcode import VM


class Scaffold:
    VACUUM_CHARS = "^>v<X"

    def __init__(self, data):
        s = "".join(map(chr, data)).strip()
        self.map = s.split("\n")
        if not all(len(line) == self.width for line in self.map):
            raise ValueError("Non-rectangular input data")
        if sum(s.count(c) for c in self.VACUUM_CHARS) != 1:
            raise ValueError("Multiple vacuums found")

    def __str__(self):
        return "\n".join(self.map)

    @property
    def height(self):
        return len(self.map)

    @property
    def width(self):
        return len(self.map[0]) if self.map else 0

    def at(self, x, y):
        return self.map[self.height - y - 1][x]

    def on_scaffold(self, x, y):
        return 0 <= x < self.width and 0 <= y < self.height and self.at(x, y) != "."

    def vacuum_location(self):
        for y in range(self.height):
            for x in range(self.width):
                if self.at(x, y) in self.VACUUM_CHARS:
                    return (x, y)

    def intersections(self):
        for y in range(1, self.height - 1):
            for x in range(1, self.width - 1):
                if self.on_scaffold(x, y) and len(self.neighbors(x, y)) == 4:
                    yield (x, y)

    def neighbors(self, x, y):
        offsets = [direction.offset for direction in Direction]
        return {(x + dx, y + dy) for dx, dy in offsets if self.on_scaffold(x + dx, y + dy)}

    def full_directions(self):
        x, y = self.vacuum_location()
        visited = {(x, y)}

        directions = []
        current_direction = Direction(self.VACUUM_CHARS.index(self.at(x, y)))
        while True:
            dx, dy = current_direction.offset
            distance = 0
            while self.on_scaffold(x + dx, y + dy):
                distance += 1
                x += dx
                y += dy
                visited.add((x, y))
            if distance:
                directions.append(str(distance))
            else:
                candidates = self.neighbors(x, y) - visited
                if len(candidates) == 0:
                    break
                elif len(candidates) > 1:
                    raise RuntimeError("Multiple paths avaiable!")
                new_x, new_y = next(iter(candidates))
                new_direction = Direction.from_offset(new_x - x, new_y - y)
                turn_count = 0
                while current_direction != new_direction:
                    current_direction = current_direction.right_turn()
                    turn_count += 1
                if turn_count == 3:
                    directions.append("L")
                else:
                    for _ in range(turn_count):
                        directions.append("R")

        return directions


def get_routines(directions):
    valid_routine_re = re.compile("^[ABC](,[ABC])*$")
    max_func_len = 20
    full_routine = ",".join(directions)

    for i in range(max_func_len, 0, -1):
        if full_routine[i] != ",":
            continue
        function_a = full_routine[:i]
        main_routine = full_routine.replace(function_a, "A")
        a_stripped = main_routine.strip("A,")
        for j in range(max_func_len, 0, -1):
            if a_stripped[j] != ",":
                continue
            function_b = a_stripped[:j]
            if "A" in function_b:
                continue
            main_routine = main_routine.replace(function_b, "B")
            b_stripped = main_routine.strip("AB,")
            for k in range(len(b_stripped)):
                if b_stripped[k] in "AB":
                    k -= 1
                    break
            function_c = b_stripped[:k]
            main_routine = main_routine.replace(function_c, "C")
            if valid_routine_re.match(main_routine):
                return main_routine, function_a, function_b, function_c


def p1(program):
    vm = VM(program[:])
    scaffold = Scaffold(vm.outputs())
    return sum(x * (scaffold.height - y - 1) for x, y in scaffold.intersections())


def p2(program):
    directions = Scaffold(VM(program[:]).outputs()).full_directions()
    input_string = "\n".join(list(get_routines(directions)) + ["n", ""])

    def inputs():
        for c in input_string:
            yield ord(c)

    program[0] = 2
    vm = VM(program[:], inputs=inputs())
    *_, final_output = vm.outputs()

    return final_output


if __name__ == "__main__":
    print("No tests run")

    data = read_data()
    program = [int(x) for x in data.strip().split(',')]
    print("Part 1: {}".format(p1(program)))
    print("Part 2: {}".format(p2(program)))
