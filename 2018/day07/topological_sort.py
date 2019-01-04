import collections
import heapq


def topological_sort(nodes, get_neighbors, get_indegree):
    working_nodes = [node for node in nodes if get_indegree(node) == 0]
    heapq.heapify(working_nodes)
    sorted_nodes = []
    indegree_deltas = collections.defaultdict(int)

    while working_nodes:
        node = heapq.heappop(working_nodes)
        sorted_nodes.append(node)
        for neighbor in get_neighbors(node):
            indegree_deltas[neighbor] -= 1
            if get_indegree(neighbor) + indegree_deltas[neighbor] == 0:
                heapq.heappush(working_nodes, neighbor)

    return sorted_nodes if len(sorted_nodes) == len(nodes) else None
