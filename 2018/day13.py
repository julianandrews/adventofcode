import enum
import fileinput

DIRECTION_SYMBOLS = ["^", ">", "v", "<"]


class TurnDirection(enum.Enum):
    LEFT = 0
    NONE = 1
    RIGHT = 2


class Direction(enum.Enum):
    NORTH = 0
    EAST = 1
    SOUTH = 2
    WEST = 3

    @classmethod
    def from_symbol(cls, symbol):
        return cls(DIRECTION_SYMBOLS.index(symbol))

    @property
    def symbol(self):
        return DIRECTION_SYMBOLS[self.value]

    @property
    def velocity(self):
        return [(0, -1), (1, 0), (0, 1), (-1, 0)][self.value]

    def turn(self, turn_direction):
        if turn_direction == TurnDirection.RIGHT:
            return Direction((self.value + 1) % 4)
        elif turn_direction == TurnDirection.LEFT:
            return Direction((self.value - 1) % 4)
        else:
            return self


class CartMap:
    DIRECTION_TRACK_MAP = {"<": "-", ">": "-", "^": "|", "v": "|"}

    def __init__(self, lines):
        self.crashes = set()
        self.lines = []
        self.carts = {}

        for y, line in enumerate(lines):
            self.lines.append([])
            for x, c in enumerate(line):
                self.lines[y].append(self.DIRECTION_TRACK_MAP.get(c, c))
                if c in DIRECTION_SYMBOLS:
                    self.carts[(x, y)] = Cart(Direction.from_symbol(c))

    def tick(self):
        for x, y in sorted(self.carts.keys(), key=lambda pos: (pos[1], pos[0])):
            if (x, y) in self.carts:
                self.tick_cart(x, y)

    def tick_cart(self, x, y):
        cart = self.carts.pop((x, y))
        dx, dy = cart.velocity

        new_position = (x + dx, y + dy)
        if new_position in self.carts:
            del self.carts[new_position]
            self.crashes.add(new_position)
        else:
            track_segment = self.lines[new_position[1]][new_position[0]]
            cart.update_direction(track_segment)
            self.carts[new_position]  = cart

    def __str__(self):
        result = [line[:] for line in self.lines]
        for (x, y), cart in self.carts.items():
            result[y][x] = cart.direction.symbol

        return "\n".join(''.join(line) for line in result)


class Cart:
    def __init__(self, direction):
        self.direction = direction
        self.next_turn = TurnDirection.LEFT

    @property
    def velocity(self):
        return self.direction.velocity

    def update_direction(self, track_segment):
        if track_segment == "+":
            self.direction = self.direction.turn(self.next_turn)
            self.next_turn = TurnDirection((self.next_turn.value + 1) % 3)
        elif track_segment == "\\":
            if self.direction in (Direction.EAST, Direction.WEST):
                self.direction = self.direction.turn(TurnDirection.RIGHT)
            else:
                self.direction = self.direction.turn(TurnDirection.LEFT)
        elif track_segment == "/":
            if self.direction in (Direction.EAST, Direction.WEST):
                self.direction = self.direction.turn(TurnDirection.LEFT)
            else:
                self.direction = self.direction.turn(TurnDirection.RIGHT)


def p1(cart_map):
    while not cart_map.crashes:
        cart_map.tick()

    return cart_map.crashes.pop()


def p2(cart_map):
    while len(cart_map.carts) > 1:
        cart_map.tick()

    return list(cart_map.carts.keys())[0]


if __name__ == "__main__":
    cart_map = CartMap([line.strip("\n") for line in fileinput.input()])
    print("%s,%s" % p1(cart_map))
    cart_map = CartMap([line.strip("\n") for line in fileinput.input()])
    print("%s,%s" % p2(cart_map))
