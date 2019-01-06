import fileinput


class TreeNode:
    def __init__(self, numbers):
        num_children = next(numbers)
        num_metadata = next(numbers)
        self.children = [TreeNode(numbers) for i in range(num_children)]
        self.metadata = [next(numbers) for i in range(num_metadata)]

    def metadata_sum(self):
        return sum(self.metadata) + sum(
            child.metadata_sum() for child in self.children
        )

    def value(self):
        if self.children:
            return sum(self.children[index - 1].value()
                       for index in self.metadata
                       if 0 < index <= len(self.children))
        else:
           return sum(self.metadata)


def p1(data):
    return TreeNode(map(int, data.split())).metadata_sum()


def p2(data):
    return TreeNode(map(int, data.split())).value()


if __name__ == "__main__":
    data = fileinput.input()[0]
    print("Part 1: %s" % p1(data))
    print("Part 2: %s" % p2(data))
