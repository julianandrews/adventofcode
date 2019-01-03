class SummedAreaTable:
    def __init__(self, generator, width, height):
        self.table = [[None] * width for y in range(height)]
        for x in range(width):
            self.table[0][x] = generator(x, 0)
        for y in range(height):
            self.table[y][0] = generator(0, y)
        for y in range(1, height):
            for x in range(1, width):
                self.table[y][x] = (
                    generator(x, y)
                    + self.table[y - 1][x]
                    + self.table[y][x - 1]
                    - self.table[y - 1][x - 1]
                )

    def intensity(self, x, y, width, height):
        x0, y0 = x - 1, y - 1
        x1, y1 = x + width - 1, y + height - 1
        value = self.table[y1][x1]
        if y0 >= 0:
            value -= self.table[y0][x1]
        if x0 >= 0:
            value -= self.table[y1][x0]
        if x0 >= 0 and y0 >= 0:
            value += self.table[y0][x0]

        return value
