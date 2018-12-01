# -*- coding: UTF-8 -*-
from utils import read_data, lines


def get_pixels(data, width=50, height=6):
    commands = lines(data)
    pixels = [[0 for x in range(width)] for y in range(height)]
    for command in commands:
        words = command.split()
        if words[0] == 'rect':
            rect_width, rect_height = list(map(int, words[1].split('x')))
            for x in range(rect_width):
                for y in range(rect_height):
                    pixels[y][x] = 1
        else:  # rotate
            val = int(words[2].split('=')[1])
            offset = int(words[4])
            if words[1] == 'row':
                offset = offset % width
                pixels[val] = pixels[val][-offset:] + pixels[val][:-offset]
            else:  # column
                old_column = [pixels[y][val] for y in range(height)]
                for y in range(height):
                    pixels[y][val] = old_column[(y - offset) % height]
    return pixels


def pixel_count(pixels):
    return sum(sum(pixels, []))


def draw_pixels(pixels):
    return '\n'.join(''.join('█' if val else ' ' for val in row) for row in pixels)


if __name__ == '__main__':
    data = read_data(8)

    assert pixel_count(get_pixels(
        """
        rect 3x2
        rotate column x=1 by 1
        rotate row y=0 by 4
        rotate column x=1 by 1
        """, 7, 3
    )) == 6
    print('All tests passed')

    pixels = get_pixels(data)
    print(pixel_count(pixels))
    print(draw_pixels(pixels))
