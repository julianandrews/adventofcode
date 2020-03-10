from utils import read_data


class Image:
    PIXEL_MAP = {
        0: " ",
        1: "█",
        2: "░",
    }

    def __init__(self, data, width, height):
        self.data = data
        self.width = width
        self.height = height

    @property
    def layer_size(self):
        return self.width * self.height

    @property
    def num_layers(self):
        return len(self.data) // self.layer_size

    def get_pixel(self, x, y):
        for layer in range(self.num_layers):
            pixel = self.data[layer * self.layer_size + y * self.width + x]
            if pixel != 2:
                break
        return pixel

    def count_digit(self, layer, digit):
        layer_range = range(layer * self.layer_size, (layer + 1) * self.layer_size)
        return sum(1 for i in layer_range if self.data[i] == digit)

    def __str__(self):
        return "\n".join(
            "".join(self.PIXEL_MAP[self.get_pixel(x, y)] for x in range(self.width))
            for y in range(self.height))


def p1(image):
    layer = min(range(image.num_layers), key=lambda layer: image.count_digit(layer, 0))
    return image.count_digit(layer, 1) * image.count_digit(layer, 2)


def p2(image):
    return str(image)


def run_tests():
    image1 = Image([int(c) for c in "123456789012"], 3, 2)
    assert p1(image1) == 1
    image2 = Image([int(c) for c in "0222112222120000"], 2, 2)
    assert p2(image2) == " █\n█ "


if __name__ == "__main__":
    run_tests()
    print("All tests passed")

    data = read_data(8)
    image = Image([int(c) for c in data.strip()], 25, 6)

    print("Part 1: {}".format(p1(image)))
    print("Part 2: \n{}".format(p2(image)))
