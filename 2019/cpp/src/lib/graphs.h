#ifndef AOC_GRAPHS_H
#define AOC_GRAPHS_H

#include <memory>
#include <queue>
#include <unordered_map>
#include <unordered_set>

namespace aoc {
namespace graphs {

template <class T> struct TraversalNode {
  const T value;
  const int depth;
  const int index;
  const std::shared_ptr<TraversalNode<T>> parent;

  TraversalNode(T value, int depth, int index,
                std::shared_ptr<TraversalNode<T>> parent)
      : value(value), depth(depth), index(index), parent(parent) {}
};

template <class T> class Graph {
public:
  virtual const std::unordered_set<T> &neighbors(const T &value) const = 0;
};

template <class T> class BFSTraversal {
  const Graph<T> &graph_;
  std::unordered_set<T> visited_;
  std::queue<std::shared_ptr<TraversalNode<T>>> to_visit_;
  int index_ = 0;

public:
  BFSTraversal(const Graph<T> &graph, const T start) : graph_(graph) {
    std::shared_ptr<TraversalNode<T>> root;
    to_visit_.push(std::make_shared<TraversalNode<T>>(start, 0, index_, root));
  }

  bool hasnext() const { return !to_visit_.empty(); }

  const std::shared_ptr<TraversalNode<T>> next() {
    auto node = to_visit_.front();
    to_visit_.pop();
    for (const auto &neighbor : graph_.neighbors(node->value)) {
      if (visited_.find(neighbor) == visited_.end()) {
        ++index_;
        to_visit_.push(std::make_shared<TraversalNode<T>>(
            neighbor, node->depth + 1, index_, node));
      }
      visited_.insert(node->value);
    }

    return node;
  }
};

} // namespace graphs
} // namespace aoc

#endif
