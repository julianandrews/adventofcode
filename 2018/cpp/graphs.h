#include <algorithm>
#include <experimental/optional>
#include <functional>
#include <queue>
#include <unordered_map>
#include <unordered_set>
#include <vector>

namespace aoc {
namespace graphs {

template <class T>
class TraversalNode {
 public:
  const T value;
  const ulong index;
  const ulong depth;
  const std::experimental::optional<TraversalNode<T> *> parent;

  TraversalNode(T value, ulong depth, ulong index,
                std::experimental::optional<TraversalNode<T> *> parent)
      : value(value), index(index), depth(depth), parent(parent){};
};

template <class T>
class BFSIterator {
 private:
  std::queue<TraversalNode<T>> queue;
  std::unordered_set<T> seen;
  std::function<std::vector<T>(const T &)> neighbors;
  ulong index;

 public:
  BFSIterator(const T start, std::function<std::vector<T>(const T &)> neighbors)
      : neighbors(neighbors) {
    queue.push(TraversalNode<T>(start, index, 0, std::experimental::nullopt));
    seen.insert(start);
  };

  const std::experimental::optional<TraversalNode<T>> next() {
    if (!queue.size()) {
      return std::experimental::nullopt;
    }

    auto node = queue.front();
    queue.pop();
    for (const auto &neighbor : neighbors(node.value)) {
      if (!seen.count(neighbor)) {
        index++;
        seen.insert(neighbor);
        queue.push({neighbor, index, node.depth + 1, &node});
      }
    }

    return std::experimental::optional<TraversalNode<T>>(node);
  }

  BFSIterator(const BFSIterator<T> &) = delete;
  BFSIterator &operator=(const BFSIterator<T> &) = delete;
};

template <class T>
std::experimental::optional<std::vector<T>> toposort(
    const std::vector<T> &values,
    std::function<std::vector<T>(const T &)> neighbors) {
  std::unordered_map<T, int> indegrees;
  for (const auto &value : values) {
    for (const auto &neighbor : neighbors(value)) {
      ++indegrees[neighbor];
    }
  }

  std::priority_queue<T, std::vector<T>, std::greater<char>> working_values;
  for (const auto &value : values) {
    if (indegrees.count(value) == 0) {
      working_values.push(value);
    }
  }

  std::vector<T> sorted_values;

  while (working_values.size()) {
    auto value = working_values.top();
    working_values.pop();
    sorted_values.push_back(value);
    for (const auto &neighbor : neighbors(value)) {
      --indegrees[neighbor];
      if (indegrees[neighbor] == 0) {
        working_values.push(neighbor);
      }
    }
  }

  return sorted_values.size() == values.size()
             ? std::experimental::optional<std::vector<T>>(sorted_values)
             : std::experimental::nullopt;
}

}  // namespace graphs
}  // namespace aoc
