from utils import read_data, lines, AStar

from copy import deepcopy
from itertools import chain, combinations


class FactoryState(object):
    def __init__(self, floor, items, moves):
        self.floor = floor
        self.items = items
        self.moves = moves
        microchips = {}
        generators = {}
        for i, floor in enumerate(self.items):
            for item in floor:
                if item[0] == 'M':
                    microchips[item[2:]] = i
                else:
                    generators[item[2:]] = i
        self.item_pairs = sorted((microchips[x], generators[x]) for x in microchips)

    def is_solved(self):
        return all(i == j == 3 for i, j in self.item_pairs)

    def is_valid(self):
        unmatched_microchip_floors = {i for i, j in self.item_pairs if i != j}
        generator_floors = {j for i, j in self.item_pairs}
        return not any(i in generator_floors for i in unmatched_microchip_floors)

    def neighbors(self):
        floor_items = self.items[self.floor]
        for direction in (1, -1):
            if 0 <= self.floor + direction <= 3:
                for subset in chain(combinations(floor_items, 2), combinations(floor_items, 1)):
                    new_state = self.make_move(subset, direction)
                    if new_state.is_valid():
                        yield new_state

    def make_move(self, items, direction):
        new_floor = self.floor + direction
        new_moves = self.moves + [(items, direction)]
        new_items = deepcopy(self.items)
        for item in items:
            new_items[self.floor].remove(item)
            new_items[new_floor].add(item)

        return FactoryState(new_floor, new_items, new_moves)

    def __eq__(self, other):
        return self.floor == other.floor and self.item_pairs == other.item_pairs

    def __hash__(self):
        return hash((self.floor, tuple(self.item_pairs)))

    def __str__(self):
        return "{} - {}".format(self.floor, self.item_pairs)


class FactorySolver(AStar):
    IRRELEVANT_WORDS = ['a', 'generator', 'microchip', 'and', 'nothing', 'relevant']

    def __init__(self, data, make_it_hard=False):
        items = []
        for line in lines(data):
            words = [s.strip(',.') for s in line.strip().split()]
            items.append(
                {self.tokenize(word) for word in words[4:] if word not in self.IRRELEVANT_WORDS}
            )
        if make_it_hard:
            items[0].update([
                'M-ELERIUM', 'G-ELERIUM', 'M-DILITHIUM', 'G-DILITHIUM'
            ])
        initial_state = FactoryState(0, items, [])
        super(FactorySolver, self).__init__(initial_state)

    @staticmethod
    def tokenize(word):
        if '-' in word:
            return "M-{}".format(word.split('-')[0].upper())
        else:
            return "G-{}".format(word.upper())

    def get_neighbors(self, state):
        return state.neighbors()

    def remaining_distance_heuristic(self, state):
        total = sum(3 - j for i, j in state.item_pairs)
        return total

    def get_edge_weight(self, node, neighbor):
        return 1

    def is_end(self, node):
        return node.is_solved()


def solution_length(data, make_it_hard=False):
    solution = FactorySolver(data, make_it_hard)()
    return len(solution)


if __name__ == '__main__':
    data = read_data(11)

    assert solution_length(
        """
        The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
        The second floor contains a hydrogen generator.
        The third floor contains a lithium generator.
        The fourth floor contains nothing relevant.
        """
    ) == 11
    print("All tests passed")

    print(solution_length(data))
    print(solution_length(data, make_it_hard=True))
