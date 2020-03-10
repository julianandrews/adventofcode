class DisjointSet:
    """A set of elements grouped into disjoint subsets."""
    def __init__(self, elements):
        self.elements = {e: i for i, e in enumerate(elements)}
        self.ids = list(range(len(elements)))

    def union(self, a, b):
        """Joins the subsets containing `a` and `b`."""
        i = self.find(a)
        j = self.find(b)
        self.ids[j] = i

    def find(self, a):
        """Returns the root element representing the subset containing `a`.

        `ds.find(a) == ds.find(b)` checks if two elements share their subsets.
        """
        i = self.elements[a]

        root = i
        while root != self.ids[root]:
            root = self.ids[root]

        while i != root:
            parent = self.ids[i]
            self.ids[i] = root
            i = parent

        return root
