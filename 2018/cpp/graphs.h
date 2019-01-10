#include <algorithm>
#include <experimental/optional>
#include <functional>
#include <queue>
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

}  // namespace graphs
}  // namespace aoc
