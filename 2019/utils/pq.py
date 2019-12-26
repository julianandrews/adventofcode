import heapq


class UpdateableQueue:
    def __init__(self):
        self.count = 0
        self.values = []
        self.entries = {}

    def push(self, value, priority):
        """Add a new value or update the priority of an existing value."""
        if value in self.entries:
            self.remove(value)
        entry = self.QueueEntry(value, priority, self.count)
        self.entries[value] = entry
        self.count += 1
        heapq.heappush(self.values, entry)

    def remove(self, value):
        """Remove a value from the queue."""
        entry = self.entries.pop(value)
        entry.removed = True

    def pop(self):
        """Remove and return the lowest priority value."""
        while self.values:
            entry = heapq.heappop(self.values)
            if not entry.removed:
                del self.entries[entry.value]
                return entry.value
        raise KeyError("Pop from empty queue")

    def __len__(self):
        return len(self.entries)


    class QueueEntry:
        def __init__(self, value, priority, counter):
            self.value = value
            self.priority = priority
            self.counter = counter
            self.removed = False

        def __lt__(self, other):
            return (self.priority, self.counter) < (other.priority, other.counter)
