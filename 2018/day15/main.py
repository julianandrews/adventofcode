import fileinput

from bfs import bfs


class Creature:
    def __init__(self, hp, attack_power, creature_type):
        self.hp = hp
        self.attack_power = attack_power
        self.creature_type = creature_type

    def __repr__(self):
        return "Creature(%s, %s, %s)" % (self.hp, self.attack_power, self.creature_type)

    def __str__(self):
        return "%s(%s)" % (self.creature_type, self.hp)


class BattleMap:
    def __init__(self, lines, elf_power, goblin_power):
        self.round = 0
        self.done = False
        self.grid = [[c for c in line] for line in lines]
        self.creatures = {
            (x, y): Creature(200, elf_power if c == "E" else goblin_power, c)
            for (y, line) in enumerate(self.grid)
            for (x, c) in enumerate(line)
            if c in "EG"}

    def run(self):
        while not self.done:
            self.tick()

    def tick(self):
        for point, creature in sorted(self.creatures.items(), key=lambda pair: sort_key(pair[0])):
            if self.done:
                return
            if creature is self.creatures.get(point):
                point = self.move(point)
                self.fight(point)

        self.round += 1

    def move(self, origin):
        destination = self.find_move(origin)

        if destination is not None:
            creature = self.creatures.pop(origin)
            self.clear_point(origin)
            self.grid[destination[1]][destination[0]] = creature.creature_type
            self.creatures[destination] = creature

        return destination if destination is not None else origin

    def fight(self, point):
        creature = self.creatures[point]
        enemy_type = "G" if creature.creature_type == "E" else "E"
        candidates = [p for p in self.get_neighbors(point, enemy_type)]
        if candidates:
            target_point = min(candidates, key = lambda p: (self.creatures[p].hp, sort_key(p)))

            self.creatures[target_point].hp -= creature.attack_power
            if self.creatures[target_point].hp <= 0:
                del self.creatures[target_point]
                self.clear_point(target_point)
                if not [c for c in self.creatures.values() if c.creature_type == enemy_type]:
                    self.done = True

    def find_move(self, origin):
        creature_type = self.grid[origin[1]][origin[0]]
        enemy_type = "G" if creature_type == "E" else "E"
        if list(self.get_neighbors(origin, enemy_type)):
            return None

        destinations = {
            neighbor for point in self.creatures.keys()
            for neighbor in self.get_neighbors(point)
            if self.grid[point[1]][point[0]] == enemy_type
        }

        best_depth = None
        good_nodes = []
        for node in bfs(origin, self.get_neighbors, sort_key):
            if node.value in destinations:
                if best_depth is None:
                    best_depth = node.depth
                elif node.depth > best_depth:
                    break
                good_nodes.append(node)

        if good_nodes:
            best_node = min(good_nodes, key=lambda node: sort_key(node.value))
            path = list(best_node.get_path())
            return path[-2]

        return None

    def clear_point(self, point):
        x, y = point
        self.grid[y][x] = "."

    def get_neighbors(self, point, allowed_tiles="."):
        x, y = point
        for (v, w) in ((x, y + 1), (x, y - 1), (x + 1, y), (x - 1, y)):
            if 0 <= w <= len(self.grid) and 0 <= v <= len(self.grid[0]) and self.grid[w][v] in allowed_tiles:
                yield (v, w)

    def elf_count(self):
        return len([c for c in self.creatures.values() if c.creature_type == "E"])

    def outcome(self):
        return self.round * sum(c.hp for c in self.creatures.values())

    def __str__(self):
        creatures_by_line = []
        for y, row in enumerate(self.grid):
            creatures_by_line.append(
                [self.creatures[(x, y)] for (x, c) in enumerate(self.grid[y]) if c in "EG"]
            )
        lines = [
            "%s   %s" % ("".join(row), ", ".join(str(creature) for creature in creatures))
            for (row, creatures) in zip(self.grid, creatures_by_line)
        ]

        return "Round: %s\n%s" % (self.round, "\n".join(lines))


def sort_key(point):
    return (point[1], point[0])


def p1(lines):
    battle_map = BattleMap(lines, 3, 3)
    battle_map.run()

    return battle_map.outcome()


def p2(lines):
    elf_power = 0
    while True:
        battle_map = BattleMap(lines, elf_power, 3)
        initial_elves = battle_map.elf_count()
        battle_map.run()
        if battle_map.elf_count() == initial_elves:
            break

        elf_power += 1

    return battle_map.outcome()


if __name__ == "__main__":
    lines = [line.strip() for line in fileinput.input()]
    print(p1(lines))
    print(p2(lines))
