from copy import deepcopy
from itertools import chain, combinations
try:
    from queue import Queue
except ImportError:
    from Queue import Queue


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

    def __eq__(self, other):
        return self.floor == other.floor and self.item_pairs == other.item_pairs

    def __hash__(self):
        return hash((self.floor, tuple(self.item_pairs)))

    def __repr__(self):
        return 'FactoryState({})'.format(self)

    def __str__(self):
        return "{} - {}".format(self.floor, self.item_pairs)


class FactorySolver:
    IRRELEVANT_WORDS = ['a', 'generator', 'microchip', 'and', 'nothing', 'relevant']

    def __init__(self, data):
        self.seen = set()
        self.queue = Queue()

        items = []
        for line in data.strip().split('\n'):
            words = [s.strip(',.') for s in line.strip().split()]
            items.append(
                {self.tokenize(word) for word in words[4:] if word not in self.IRRELEVANT_WORDS}
            )
        items[0].update(('M-ELERIUM', 'G-ELERIUM', 'M-DILITHIUM', 'G-DILITHIUM'))
        initial_state = FactoryState(0, items, [])
        self.queue.put(initial_state)
        self.seen.add(initial_state)

    @staticmethod
    def tokenize(word):
        if '-' in word:
            return "M-{}".format(word.split('-')[0].upper())
        else:
            return "G-{}".format(word.upper())

    def solve(self):
        solution = None

        while not self.queue.empty():
            state = self.queue.get()
            if state.is_solved():
                solution = state
            if solution is None or len(solution.moves) > len(state.moves) + 1:
                self.queue_children(state)

        return solution

    def queue_children(self, old_state):
        # must be valid, and never seen
        floor_items = old_state.items[old_state.floor]
        for direction in (1, -1):
            if 0 <= old_state.floor + direction <= 3:
                for subset in chain(combinations(floor_items, 2), combinations(floor_items, 1)):
                    new_state = self.make_move(old_state, subset, direction)
                    if new_state not in self.seen:
                        self.seen.add(new_state)
                        if new_state.is_valid():
                            self.queue.put(new_state)

    def make_move(self, old_state, items, direction):
        new_floor = old_state.floor + direction
        new_moves = old_state.moves + [(items, direction)]
        new_items = deepcopy(old_state.items)
        for item in items:
            new_items[old_state.floor].remove(item)
            new_items[new_floor].add(item)

        return FactoryState(new_floor, new_items, new_moves)


def doit(data):
    solver = FactorySolver(data)
    solution = solver.solve()
    return len(solution.moves)


if __name__ == '__main__':
    with open('data/d11.txt') as f:
        data = f.read()
    print(doit(data))
