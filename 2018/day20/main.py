import collections
import fileinput

from bfs import bfs


class FacilityMap:
    DIRECTIONS = {"N": (0, 1), "E": (1, 0), "S": (0, -1), "W": (-1, 0)}

    def __init__(self, regex):
        current_room = (0, 0)
        self.doors = collections.defaultdict(set)
        stack = []
        for token in regex[1:-1]:
            if token in "NESW":
                next_room = tuple(map(sum, zip(current_room, self.DIRECTIONS[token])))
                self.doors[current_room].add(next_room)
                self.doors[next_room].add(current_room)
                current_room = next_room
            elif token == "(":
                stack.append(current_room)
            elif token == ")":
                current_room = stack.pop()
            elif token == "|":
                current_room = stack[-1]

    def __str__(self):
        rooms = {room for rooms in self.doors.values() for room in rooms}
        min_x = min(x for x, y in rooms)
        max_x = max(x for x, y in rooms)
        min_y = min(y for x, y in rooms)
        max_y = max(y for x, y in rooms)
        space_row = ["#" for x in range((max_x - min_x + 1) * 2 + 1)]
        room_row = list("#." * (max_x - min_x + 1) + "#")
        lines = [space_row[:]]
        for y in range(min_y, max_y + 1):
            lines.append(room_row[:])
            lines.append(space_row[:])

        print(min_y)
        for (x, y), doors in self.doors.items():
            for (v, w) in doors:
                c = "|" if y == w else "-"
                lines[y + w - 2 * min_y + 1][x + v - 2 * min_x + 1] = c
        lines[-min_y * 2 + 1][-min_x * 2 + 1] = "X"

        return "\n".join("".join(line) for line in reversed(lines))

    def get_neighbors(self, room):
        return self.doors[room]


def p1(facility_map):
    for node in bfs((0, 0), facility_map.get_neighbors):
        pass

    return node.depth


if __name__ == "__main__":
    regex = next(iter(fileinput.input())).strip()
    facility_map = FacilityMap(regex)
    print(p1(facility_map))
