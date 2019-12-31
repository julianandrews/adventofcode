import collections

from utils import read_data
from utils.graphs import bfs


class DonutMaze:
    def __init__(self, lines):
        for i, line in enumerate(lines):
            if len(line) != len(lines[0]):
                raise ValueError("Non rectangular maze error")

        self.grid = [
            [" " if c.isupper() else c for c in line[2:-2]]
            for line in lines[2:-2]
        ]

        hole_start = None
        hole_end = None
        for y in range(len(self.grid)):
            for x in range(len(self.grid[0])):
                if lines[y + 2][x + 2] not in "#.":
                    if hole_start is None:
                        hole_start = (x, y)
                    else:
                        hole_end = (x, y)

        labels = collections.defaultdict(lambda: [None, None])
        for y in range(len(self.grid)):
            if lines[y + 2][1].isupper():
                label = lines[y + 2][0] + lines[y + 2][1]
                labels[label][0] = (0, y)
            if lines[y + 2][-2].isupper():
                label = lines[y + 2][-2] + lines[y + 2][-1]
                labels[label][0] = (len(self.grid[0]) - 1, y)
            if lines[y + 2][hole_start[0] + 2].isupper():
                label = lines[y + 2][hole_start[0] + 2] + lines[y + 2][hole_start[0] + 3]
                labels[label][1] = (hole_start[0] - 1, y)
            if lines[y + 2][hole_end[0] + 2].isupper():
                label = lines[y + 2][hole_end[0] + 1] + lines[y + 2][hole_end[0] + 2]
                labels[label][1] = (hole_end[0] + 1, y)
        for x in range(len(self.grid[0])):
            if lines[1][x + 2].isupper():
                label = lines[0][x + 2] + lines[1][x + 2]
                labels[label][0] = (x, 0)
            if lines[-2][x + 2].isupper():
                label = lines[-2][x + 2] + lines[-1][x + 2]
                labels[label][0] = (x, len(self.grid) - 1)
            if lines[hole_start[1] + 2][x + 2].isupper():
                label = lines[hole_start[1] + 2][x + 2] + lines[hole_start[1] + 3][x + 2]
                labels[label][1] = (x, hole_start[1] - 1)
            if lines[hole_end[1] + 2][x + 2].isupper():
                label = lines[hole_end[1] + 1][x + 2] + lines[hole_end[1] + 2][x + 2]
                labels[label][1] = (x, hole_end[1] + 1)

        self.start = labels["AA"][0]
        self.end = labels["ZZ"][0]
        del labels["AA"]
        del labels["ZZ"]
        self.down_portals = {}
        self.up_portals = {}
        for up, down in labels.values():
            self.down_portals[down] = up
            self.up_portals[up] = down

    def simple_neighbors(self, point):
        for dx, dy in ((1, 0), (-1, 0), (0, 1), (0, -1)):
            x, y = point[0] + dx, point[1] + dy
            if 0 <= y and y < len(self.grid) and 0 <= x and x < len(self.grid[0]):
                if self.grid[y][x] == '.':
                    yield (x, y)

    def portal_neighbors(self, point):
        yield from self.simple_neighbors(point)
        if point in self.down_portals:
            yield self.down_portals[point]
        if point in self.up_portals:
            yield self.up_portals[point]

    def recursive_neighbors(self, point):
        x, y, depth = point
        for (new_x, new_y) in self.simple_neighbors((x, y)):
            yield (new_x, new_y, depth)
        if (x, y) in self.down_portals:
            yield (*self.down_portals[(x, y)], depth + 1)
        if (x, y) in self.up_portals and depth > 0:
            yield (*self.up_portals[(x, y)], depth - 1)

    def solution_length(self):
        for node in bfs(self.start, self.portal_neighbors):
            if node.value == self.end:
                return node.depth

    def recursive_solution_length(self):
        for node in bfs((*self.start, 0), self.recursive_neighbors):
            if node.value == (*self.end, 0):
                return node.depth


def p1(lines):
    maze = DonutMaze(lines)
    return maze.solution_length()


def p2(lines):
    maze = DonutMaze(lines)
    return maze.recursive_solution_length()


def run_tests():
    maze_1 = DonutMaze([
        "         A           ",
        "         A           ",
        "  #######.#########  ",
        "  #######.........#  ",
        "  #######.#######.#  ",
        "  #######.#######.#  ",
        "  #######.#######.#  ",
        "  #####  B    ###.#  ",
        "BC...##  C    ###.#  ",
        "  ##.##       ###.#  ",
        "  ##...DE  F  ###.#  ",
        "  #####    G  ###.#  ",
        "  #########.#####.#  ",
        "DE..#######...###.#  ",
        "  #.#########.###.#  ",
        "FG..#########.....#  ",
        "  ###########.#####  ",
        "             Z       ",
        "             Z       ",
    ])
    assert maze_1.solution_length() == 23

    maze_2 = DonutMaze([
        "                   A               ",
        "                   A               ",
        "  #################.#############  ",
        "  #.#...#...................#.#.#  ",
        "  #.#.#.###.###.###.#########.#.#  ",
        "  #.#.#.......#...#.....#.#.#...#  ",
        "  #.#########.###.#####.#.#.###.#  ",
        "  #.............#.#.....#.......#  ",
        "  ###.###########.###.#####.#.#.#  ",
        "  #.....#        A   C    #.#.#.#  ",
        "  #######        S   P    #####.#  ",
        "  #.#...#                 #......VT",
        "  #.#.#.#                 #.#####  ",
        "  #...#.#               YN....#.#  ",
        "  #.###.#                 #####.#  ",
        "DI....#.#                 #.....#  ",
        "  #####.#                 #.###.#  ",
        "ZZ......#               QG....#..AS",
        "  ###.###                 #######  ",
        "JO..#.#.#                 #.....#  ",
        "  #.#.#.#                 ###.#.#  ",
        "  #...#..DI             BU....#..LF",
        "  #####.#                 #.#####  ",
        "YN......#               VT..#....QG",
        "  #.###.#                 #.###.#  ",
        "  #.#...#                 #.....#  ",
        "  ###.###    J L     J    #.#.###  ",
        "  #.....#    O F     P    #.#...#  ",
        "  #.###.#####.#.#####.#####.###.#  ",
        "  #...#.#.#...#.....#.....#.#...#  ",
        "  #.#####.###.###.#.#.#########.#  ",
        "  #...#.#.....#...#.#.#.#.....#.#  ",
        "  #.###.#####.###.###.#.#.#######  ",
        "  #.#.........#...#.............#  ",
        "  #########.###.###.#############  ",
        "           B   J   C               ",
        "           U   P   P               ",
    ])
    assert maze_2.solution_length() == 58

    maze_3 = DonutMaze([
        "             Z L X W       C                 ",
        "             Z P Q B       K                 ",
        "  ###########.#.#.#.#######.###############  ",
        "  #...#.......#.#.......#.#.......#.#.#...#  ",
        "  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  ",
        "  #.#...#.#.#...#.#.#...#...#...#.#.......#  ",
        "  #.###.#######.###.###.#.###.###.#.#######  ",
        "  #...#.......#.#...#...#.............#...#  ",
        "  #.#########.#######.#.#######.#######.###  ",
        "  #...#.#    F       R I       Z    #.#.#.#  ",
        "  #.###.#    D       E C       H    #.#.#.#  ",
        "  #.#...#                           #...#.#  ",
        "  #.###.#                           #.###.#  ",
        "  #.#....OA                       WB..#.#..ZH",
        "  #.###.#                           #.#.#.#  ",
        "CJ......#                           #.....#  ",
        "  #######                           #######  ",
        "  #.#....CK                         #......IC",
        "  #.###.#                           #.###.#  ",
        "  #.....#                           #...#.#  ",
        "  ###.###                           #.#.#.#  ",
        "XF....#.#                         RF..#.#.#  ",
        "  #####.#                           #######  ",
        "  #......CJ                       NM..#...#  ",
        "  ###.#.#                           #.###.#  ",
        "RE....#.#                           #......RF",
        "  ###.###        X   X       L      #.#.#.#  ",
        "  #.....#        F   Q       P      #.#.#.#  ",
        "  ###.###########.###.#######.#########.###  ",
        "  #.....#...#.....#.......#...#.....#.#...#  ",
        "  #####.#.###.#######.#######.###.###.#.#.#  ",
        "  #.......#.......#.#.#.#.#...#...#...#.#.#  ",
        "  #####.###.#####.#.#.#.#.###.###.#.###.###  ",
        "  #.......#.....#.#...#...............#...#  ",
        "  #############.#.#.###.###################  ",
        "               A O F   N                     ",
        "               A A D   M                     ",
    ])
    assert maze_3.recursive_solution_length() == 396


if __name__ == "__main__":
    run_tests()
    print("All tests passed")

    data = read_data(20)
    lines = data.strip("\n").split("\n")
    print("Part 1: {}".format(p1(lines)))
    print("Part 2: {}".format(p2(lines)))
