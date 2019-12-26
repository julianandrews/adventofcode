import collections

from utils import read_data
from utils.graphs import bfs
from utils.pq import UpdateableQueue


MazeState = collections.namedtuple("MazeState", ["robot_locations", "collected_keys"])


class KeyMaze:
    def __init__(self, maze_string):
        self.map = [
            [c for c in line.strip()]
            for line in maze_string.strip().split("\n")
        ]

        # A mapping from keys or starting point indices to coordinates.
        waypoints = {}
        self.entry_point_count = 0
        for y, line in enumerate(self.map):
            if len(line) != len(self.map[0]):
                raise ValueError("Non-rectangular map")
            for x, tile in enumerate(line):
                if tile == '@':
                    self.map[y][x] = str(self.entry_point_count)
                    tile = str(self.entry_point_count)
                    self.entry_point_count += 1
                if tile.islower() or tile.isdigit():
                    waypoints[tile] = (x, y)

        self.waypoint_distances = collections.defaultdict(dict)
        self.waypoint_doors = collections.defaultdict(set)
        for from_tile, position in waypoints.items():
            for node in bfs(position, self.neighbors):
                to_tile = self.at(*node.value)
                if to_tile.islower():
                    self.waypoint_distances[from_tile][to_tile] = node.depth
                    inbetween_tiles = [self.at(*p) for p in node.get_path()]
                    self.waypoint_doors[(from_tile, to_tile)] = {
                        t.lower() for t in inbetween_tiles if t.isupper()
                    }

    @property
    def height(self):
        return len(self.map)

    @property
    def width(self):
        return len(self.map[0]) if self.map else 0

    @property
    def key_count(self):
        return len(self.waypoint_distances) - self.entry_point_count

    def at(self, x, y):
        if x < 0 or x >= self.width or y < 0 or y >= self.height:
            return "#"
        else:
            return self.map[y][x]

    def neighbors(self, position):
        x, y = position
        for dx, dy in ((0, 1), (1, 0), (0, -1), (-1, 0)):
            new_x = x + dx
            new_y = y + dy
            if self.at(new_x, new_y) != "#":
                yield new_x, new_y

    def state_neighbors(self, state):
        for from_tile in state.robot_locations:
            for to_tile in self.waypoint_distances[from_tile]:
                already_visited = to_tile in state.collected_keys
                blocking_doors = self.waypoint_doors[(from_tile, to_tile)] - state.collected_keys
                if not already_visited and not blocking_doors:
                    new_locations = (state.robot_locations - {from_tile}) ^ {to_tile}
                    new_keys = state.collected_keys ^ {to_tile}
                    yield (from_tile, to_tile), MazeState(new_locations, new_keys)

    def steps(self):
        entry_points = frozenset(str(d) for d in range(self.entry_point_count))
        starting_state = MazeState(entry_points, frozenset())
        to_process = UpdateableQueue()
        to_process.push(starting_state, 0)
        distances = {starting_state: 0}

        while to_process:
            maze_state = to_process.pop()
            for (from_tile, to_tile), new_state in self.state_neighbors(maze_state):
                if to_tile in self.waypoint_distances[from_tile]:
                    extra_distance = self.waypoint_distances[from_tile][to_tile]
                    new_distance = distances[maze_state] + extra_distance
                    if new_state not in distances or new_distance < distances[new_state]:
                        to_process.push(new_state, new_distance)
                        distances[new_state] = new_distance

        return min(
            distance
            for maze_state, distance in distances.items()
            if len(maze_state.collected_keys) == self.key_count
        )


def tweak_maze_string(maze_string):
    lines = [line.strip() for line in maze_string.strip().split("\n")]
    for y, line in enumerate(lines):
        for x, c in enumerate(line):
            if c == "@":
                lines[y - 1] = lines[y - 1][:x - 1] + "@#@" + lines[y - 1][x + 2:]
                lines[y] = lines[y][:x - 1] + "###" + lines[y][x + 2:]
                lines[y + 1] = lines[y + 1][:x - 1] + "@#@" + lines[y + 1][x + 2:]
                return "\n".join(lines)


def p1(data):
    key_maze = KeyMaze(data.strip())
    return key_maze.steps()


def p2(data):
    maze_string = tweak_maze_string(data)
    key_maze = KeyMaze(maze_string)
    return key_maze.steps()


def run_tests():
    maze_1 = KeyMaze("""
        #########
        #b.A.@.a#
        #########
    """)
    assert maze_1.steps() == 8

    maze_2 = KeyMaze("""
        ########################
        #f.D.E.e.C.b.A.@.a.B.c.#
        ######################.#
        #d.....................#
        ########################
    """)
    assert maze_2.steps() == 86

    maze_3 = KeyMaze("""
        ########################
        #...............b.C.D.f#
        #.######################
        #.....@.a.B.c.d.A.e.F.g#
        ########################
    """)
    assert maze_3.steps() == 132

    maze_4 = KeyMaze("""
        #################
        #i.G..c...e..H.p#
        ########.########
        #j.A..b...f..D.o#
        ########@########
        #k.E..a...g..B.n#
        ########.########
        #l.F..d...h..C.m#
        #################
    """)
    assert maze_4.steps() == 136

    maze_5 = KeyMaze("""
        ########################
        #@..............ac.GI.b#
        ###d#e#f################
        ###A#B#C################
        ###g#h#i################
        ########################
    """)
    assert maze_5.steps() == 81

    maze_6 = KeyMaze("""
        #######
        #a.#Cd#
        ##@#@##
        #######
        ##@#@##
        #cB#Ab#
        #######
    """)
    assert maze_6.steps() == 8

    maze_7 = KeyMaze("""
        ###############
        #d.ABC.#.....a#
        ######@#@######
        ###############
        ######@#@######
        #b.....#.....c#
        ###############
    """)
    assert maze_7.steps() == 24

    maze_8 = KeyMaze("""
        #############
        #DcBa.#.GhKl#
        #.###@#@#I###
        #e#d#####j#k#
        ###C#@#@###J#
        #fEbA.#.FgHi#
        #############
    """)
    assert maze_8.steps() == 32


if __name__ == "__main__":
    run_tests()
    print("All tests passed")

    data = read_data(18)
    print("Part 1: {}".format(p1(data)))
    print("Part 2: {}".format(p2(data)))
