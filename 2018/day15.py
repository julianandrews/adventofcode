import fileinput

from utils.graphs import bfs


INITIAL_HP = 200


class Creature:
    def __init__(self, hp, attack_power, creature_type):
        self.hp = hp
        self.attack_power = attack_power
        self.creature_type = creature_type

    def __str__(self):
        return "%s(%s)" % (self.creature_type, self.hp)


class BattleMap:
    def __init__(self, lines, elf_power, goblin_power):
        self.round = 0
        self.done = False
        self.grid = [list(line) for line in lines]
        self.creatures = {
            (x, y): Creature(INITIAL_HP, elf_power if c == "E" else goblin_power, c)
            for (y, line) in enumerate(self.grid)
            for (x, c) in enumerate(line)
            if c in "EG"}

    def run(self):
        while not self.done:
            self.tick()

    def tick(self):
        sorted_creatures = sorted(
            self.creatures.items(),
            key=lambda pair: sort_key(pair[0])
        )
        for point, creature in sorted_creatures:
            if creature is self.creatures.get(point):
                new_point = self.move(point)
                self.fight(new_point)
                if self.done:
                    return

        self.round += 1

    def move(self, origin):
        destination = self.find_move(origin)

        if destination is not None:
            creature = self.creatures.pop(origin)
            self.clear_point(origin)
            self.grid[destination[1]][destination[0]] = creature.creature_type
            self.creatures[destination] = creature
        else:
            destination = origin

        return destination

    def fight(self, point):
        creature = self.creatures[point]
        enemy_type = "G" if creature.creature_type == "E" else "E"
        candidates = [p for p in self.get_neighbors(point, enemy_type)]
        if candidates:
            target_point = min(
                candidates,
                key=lambda p: (self.creatures[p].hp, sort_key(p))
            )

            self.creatures[target_point].hp -= creature.attack_power
            if self.creatures[target_point].hp <= 0:
                del self.creatures[target_point]
                self.clear_point(target_point)
                if not self.creature_count(enemy_type):
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

        for node in bfs(origin, self.get_neighbors, lambda x: (x in destinations, sort_key(x))):
            if node.value in destinations:
                return list(node.get_path())[-2]

        return None

    def clear_point(self, point):
        x, y = point
        self.grid[y][x] = "."

    def get_neighbors(self, point, allowed_tiles="."):
        x, y = point
        for (v, w) in ((x, y + 1), (x, y - 1), (x + 1, y), (x - 1, y)):
            if 0 <= w <= len(self.grid) and 0 <= v <= len(self.grid[0]):
                if self.grid[w][v] in allowed_tiles:
                    yield (v, w)

    def creature_count(self, creature_type):
        return len([
            c for c in self.creatures.values()
            if c.creature_type == creature_type
        ])

    def outcome(self):
        return self.round * sum(c.hp for c in self.creatures.values())

    def __str__(self):
        creatures_by_line = []
        for y, row in enumerate(self.grid):
            creatures_by_line.append([
                self.creatures[(x, y)]
                for (x, c) in enumerate(self.grid[y])
                if c in "EG"
            ])
        lines = [
            "%s   %s" % (
                "".join(row),
                ", ".join(str(creature) for creature in creatures)
            )
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
    bottom = 0
    top = INITIAL_HP
    while True:
        elf_power = (bottom + top) // 2
        battle_map = BattleMap(lines, elf_power, 3)
        initial_elves = battle_map.creature_count("E")
        battle_map.run()
        if battle_map.creature_count("E") == initial_elves:
            if elf_power == top:
                break
            top = elf_power
        else:
            bottom = elf_power + 1

    return battle_map.outcome()


if __name__ == "__main__":
    lines = [line.strip() for line in fileinput.input()]
    print(p1(lines))
    print(p2(lines))
