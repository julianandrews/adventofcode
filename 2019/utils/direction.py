import enum


class Direction(enum.Enum):
    NORTH = 0
    EAST = 1
    SOUTH = 2
    WEST = 3

    @property
    def offset(self):
        if self == Direction.NORTH:
            return (0, -1)
        elif self == Direction.SOUTH:
            return (0, 1)
        elif self == Direction.EAST:
            return (1, 0)
        elif self == Direction.WEST:
            return (-1, 0)

    @staticmethod
    def from_offset(dx, dy):
        if (dx, dy) == (0, -1):
            return Direction.NORTH
        elif (dx, dy) == (0, 1):
            return Direction.SOUTH
        elif (dx, dy) == (1, 0):
            return Direction.EAST
        elif (dx, dy) == (-1, 0):
            return Direction.WEST

    def right_turn(self):
        return Direction((self.value + 1) % 4)

    def left_turn(self):
        return Direction((self.value - 1) % 4)
