class DisjointSet:
    def __init__(self, elements):
        self.elements = {e: i for i, e in enumerate(elements)}
        self.ids = list(range(len(elements)))

    def union(self, a, b):
        i = self.root(a)
        j = self.root(b)
        self.ids[j] = i

    def find(self, a, b):
        return self.root(a) == self.root(b)

    def root(self, a):
        i = self.elements[a]

        root = i
        while root != self.ids[root]:
            root = self.ids[root]

        while i != root:
            parent = self.ids[i]
            self.ids[i] = root
            i = parent

        return root
