# -*- coding: UTF-8 -*-

def doit(s, width=50, height=6):
    data = [[0 for x in range(width)] for y in range(height)]
    commands = [line.strip() for line in s.strip().split('\n')]
    for command in commands:
        words = command.split()
        if words[0] == 'rect':
            rect_width, rect_height = list(map(int, words[1].split('x')))
            for x in range(rect_width):
                for y in range(rect_height):
                    data[y][x] = 1
        else:  # rotate
            val = int(words[2].split('=')[1])
            offset = int(words[4])
            if words[1] == 'row':
                offset = offset % width
                data[val] = data[val][-offset:] + data[val][:-offset]
            else:  # column
                old_column = [data[y][val] for y in range(height)]
                for y in range(height):
                    data[y][val] = old_column[(y - offset) % height]
    return '\n'.join(''.join('█' if val else ' ' for val in row) for row in data)


if __name__ == '__main__':
    with open('data/d8.txt') as f:
        s = f.read()
    print(doit(s))
