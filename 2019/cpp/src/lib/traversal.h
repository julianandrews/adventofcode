#ifndef AOC_GRAPHS_TRAVERSAL_H
#define AOC_GRAPHS_TRAVERSAL_H

#include <iostream>
#include <iterator>
#include <memory>
#include <queue>
#include <unordered_set>

#include "graph.h"

namespace aoc {
namespace graphs {
namespace traversal {

template <class T> struct TraversalNode {
  const T value;
  const int depth;
  const int index;
  const std::shared_ptr<TraversalNode<T>> parent;

  TraversalNode(T value, int depth, int index,
                std::shared_ptr<TraversalNode<T>> parent)
      : value(value), depth(depth), index(index), parent(parent) {}

  TraversalNode(T value, int depth, int index)
      : value(value), depth(depth), index(index) {}
};

template <class T, class Neighbors, class Container> class GraphIterator {
  const Graph<T, Neighbors> &graph_;
  std::unordered_set<T> visited_;
  int index_ = 0;
  Container to_visit_;

public:
  using iterator_category = std::input_iterator_tag;
  using value_type = TraversalNode<T>;
  using reference = TraversalNode<T> &;
  using pointer = std::shared_ptr<TraversalNode<T>>;
  using diference_type = void;

  GraphIterator(const Graph<T, Neighbors> &graph, const T &start)
      : graph_(graph) {
    to_visit_.push(std::make_shared<TraversalNode<T>>(start, 0, 0));
  }

  GraphIterator(const GraphIterator<T, Neighbors, Container> &other)
      : graph_(other.graph_), visited_(other.visited_), index_(other.index_),
        to_visit_(other.to_visit_) {}

  GraphIterator<T, Neighbors, Container> &operator++() {
    auto node = to_visit_.peek();
    to_visit_.pop();
    if (visited_.find(node->value) == visited_.end()) {
      visited_.insert(node->value);
      for (const auto &neighbor : graph_.neighbors(node->value)) {
        if (visited_.find(neighbor) == visited_.end()) {
          ++index_;
          to_visit_.push(std::make_shared<TraversalNode<T>>(
              neighbor, node->depth + 1, index_, node));
        }
      }
    }
    return *this;
  }

  TraversalNode<T> &operator*() { return *to_visit_.peek(); }

  bool empty() const { return to_visit_.empty(); }
};

struct GraphIteratorEnd {};

template <class T, class Neighbors, class Container>
bool operator==(const GraphIterator<T, Neighbors, Container> &lhs,
                const GraphIterator<T, Neighbors, Container> &rhs) {
  return lhs.index_ == rhs.index_;
}

template <class T, class Neighbors, class Container>
bool operator==(const GraphIterator<T, Neighbors, Container> &lhs,
                const GraphIteratorEnd &rhs) {
  return lhs.empty();
}

template <class T, class Neighbors, class Container>
bool operator==(const GraphIteratorEnd &lhs,
                const GraphIterator<T, Neighbors, Container> &rhs) {
  return rhs.empty();
}

template <class T, class Neighbors, class Container>
bool operator!=(const GraphIterator<T, Neighbors, Container> &lhs,
                const GraphIteratorEnd &rhs) {
  return !(lhs == rhs);
}

template <class T, class Neighbors, class Container>
bool operator!=(const GraphIteratorEnd &lhs,
                const GraphIterator<T, Neighbors, Container> &rhs) {
  return !(lhs == rhs);
}

template <class T, class Neighbors, class Container>
bool operator!=(const GraphIterator<T, Neighbors, Container> &lhs,
                const GraphIterator<T, Neighbors, Container> &rhs) {
  return !(lhs == rhs);
}

template <class T, class Neighbors, class Container> class GraphTraversal {
  const Graph<T, Neighbors> &graph_;
  const T &start_;

public:
  GraphTraversal(const Graph<T, Neighbors> &graph, const T &start)
      : graph_(graph), start_(start) {}

  GraphIterator<T, Neighbors, Container> begin() {
    return GraphIterator<T, Neighbors, Container>(graph_, start_);
  }

  GraphIteratorEnd end() { return GraphIteratorEnd(); }
};

template <class T> class BFSQ {
  std::queue<std::shared_ptr<TraversalNode<T>>> values_;

public:
  BFSQ() = default;

  explicit BFSQ(const BFSQ &other) : values_(other.values_) {}

  void push(std::shared_ptr<TraversalNode<T>> node) { values_.push(node); }

  void pop() { values_.pop(); }

  std::shared_ptr<TraversalNode<T>> peek() { return values_.front(); }

  bool empty() const { return values_.empty(); }
};

template <class T> class DFSQ {
  std::vector<std::shared_ptr<TraversalNode<T>>> values_;

public:
  DFSQ() = default;

  explicit DFSQ(const DFSQ &other) : values_(other.values_) {}

  void push(std::shared_ptr<TraversalNode<T>> node) { values_.push_back(node); }

  void pop() { values_.pop_back(); }

  std::shared_ptr<TraversalNode<T>> peek() { return values_.back(); }

  bool empty() const { return values_.empty(); }
};

} // namespace traversal
} // namespace graphs
} // namespace aoc

#endif
