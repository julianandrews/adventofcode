import copy

from utils import read_data
from utils.math import modinverse


class Deck:
    def __init__(self, size, stride=None, offset=None):
        self.size = size
        self.stride = 1 if stride is None else stride
        self.offset = 0 if offset is None else offset

    def __repr__(self):
        return f"Deck({self.size}, {self.stride}, {self.offset})"

    def __iter__(self):
        for i in range(self.size):
            yield (self.offset + self.stride * i) % self.size

    def __copy__(self):
        return Deck(self.size, self.stride, self.offset)

    def __eq__(self, other):
        return (self.size == other.size and
                self.stride == other.stride and
                self.offset == other.offset)

    def __add__(self, other):
        if self.size != other.size:
            raise ValueError("Inconsistent deck sizes")
        deck = copy.copy(self)
        deck.cut(other.offset)
        deck.stride = (deck.stride * other.stride) % self.size
        return deck

    def __mul__(self, n):
        deck = Deck(self.size)
        one_shuffle = copy.copy(self)

        while n:
            count = 1
            new_deck = copy.copy(one_shuffle)
            while count < n // 2:
                new_deck = new_deck + new_deck
                count *= 2
            n -= count
            deck += new_deck
        return deck

    @classmethod
    def from_moves(cls, size, moves):
        deck = Deck(size)
        deck.shuffle(moves)
        return deck

    def cut(self, n):
        self.offset = (self.offset + self.stride * n) % self.size

    def increment(self, n):
        self.stride = (self.stride * modinverse(n, self.size)) % self.size

    def deal_new_stack(self):
        self.increment(self.size - 1)
        self.cut(1)

    def shuffle(self, moves):
        for move in moves:
            if move.startswith("deal with increment"):
                self.increment(int(move.split(" ")[-1]))
            elif move.startswith("deal into new stack"):
                self.deal_new_stack()
            elif move.startswith("cut"):
                self.cut(int(move.split(" ")[-1]))


def run_tests():
    deck = Deck.from_moves(10, [
        "deal with increment 9",
    ])
    assert list(deck) == [0, 9, 8, 7, 6, 5, 4, 3, 2, 1]

    deck = Deck.from_moves(10, [
        "deal with increment 7",
        "deal into new stack",
        "deal into new stack",
    ])
    assert list(deck) == [0, 3, 6, 9, 2, 5, 8, 1, 4, 7]

    deck = Deck.from_moves(10, [
        "cut 6",
        "deal with increment 7",
        "deal into new stack",
    ])
    assert list(deck) == [3, 0, 7, 4, 1, 8, 5, 2, 9, 6]

    deck = Deck.from_moves(10, [
        "deal with increment 7",
        "deal with increment 9",
        "cut -2",
    ])
    assert list(deck) == [6, 3, 0, 7, 4, 1, 8, 5, 2, 9]

    deck = Deck.from_moves(10, [
        "deal into new stack",
        "cut -2",
        "deal with increment 7",
        "cut 8",
        "cut -4",
        "deal with increment 7",
        "cut 3",
        "deal with increment 9",
        "deal with increment 3",
        "cut -1",
    ])
    assert list(deck) == [9, 2, 5, 8, 1, 4, 7, 0, 3, 6]

    moves_1 = [
        "deal with increment 7",
        "deal into new stack",
        "cut -2",
    ]
    moves_2 = [
        "cut 8",
        "cut 3",
        "deal with increment 9",
    ]
    deck_1 = deck.from_moves(10, moves_1)
    deck_2 = deck.from_moves(10, moves_2)
    assert deck_1 + deck_2 == deck.from_moves(10, moves_1 + moves_2)
    assert deck_2 + deck_1 == deck.from_moves(10, moves_2 + moves_1)

    moves = [
        "deal into new stack",
        "cut -2",
        "deal with increment 7",
        "cut 8",
        "cut -4",
        "deal with increment 7",
        "cut 3",
        "deal with increment 9",
        "deal with increment 3",
        "cut -1",
    ]
    deck = Deck.from_moves(10, moves)
    for i in range(20):
        assert deck * i == Deck.from_moves(10, moves * i)


def p1(moves):
    deck = Deck.from_moves(10007, moves)
    for i, c in enumerate(deck):
        if c == 2019:
            return i


def p2(moves):
    deck = Deck.from_moves(119315717514047, moves)
    deck = deck * 101741582076661

    for i, c in enumerate(deck):
        if i == 2020:
            return c


if __name__ == "__main__":
    run_tests()
    print("All tests passed")

    data = read_data()
    moves = data.strip().split("\n")

    print("Part 1: {}".format(p1(moves)))
    print("Part 2: {}".format(p2(moves)))
