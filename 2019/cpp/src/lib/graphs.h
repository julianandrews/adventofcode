#ifndef AOC_GRAPHS_H
#define AOC_GRAPHS_H

#include <iostream>
#include <iterator>
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

  TraversalNode(T value, int depth, int index)
      : value(value), depth(depth), index(index) {}
};

/**
 * Abstract graph class.
 *
 * T must implement `operator==()` and must be hashable.
 * T is assumed to be cheap/fast to copy.
 */
template <class T, class NeighborIterator,
          class NeighborIteratorEnd = NeighborIterator>
class Graph {
public:
  virtual NeighborIterator neighbors_begin(const T &value) const = 0;

  virtual NeighborIteratorEnd neighbors_end(const T &value) const = 0;
};

template <class T, class NeighborIterator,
          class NeighborIteratorEnd = NeighborIterator>
class BFSIterator {
  const Graph<T, NeighborIterator, NeighborIteratorEnd> &graph_;
  std::unordered_set<T> visited_;
  std::queue<std::shared_ptr<TraversalNode<T>>> to_visit_;
  int index_ = 0;

public:
  using iterator_category = std::input_iterator_tag;
  using value_type = TraversalNode<T>;
  using reference = TraversalNode<T> &;
  using pointer = std::shared_ptr<TraversalNode<T>>;
  using diference_type = void;

  BFSIterator(const Graph<T, NeighborIterator, NeighborIteratorEnd> &graph,
              const T start)
      : graph_(graph) {
    to_visit_.push(std::make_shared<TraversalNode<T>>(start, 0, 0));
  }

  BFSIterator(const BFSIterator &other)
      : graph_(other.graph_), visited_(other.visited_),
        to_visit_(other.to_visit_), index_(other.index_) {}

  bool empty() const { return to_visit_.empty(); }

  BFSIterator<T, NeighborIterator, NeighborIteratorEnd> &operator++() {
    auto node = to_visit_.front();
    to_visit_.pop();
    for (auto neighbor = graph_.neighbors_begin(node->value);
         neighbor != graph_.neighbors_end(node->value); ++neighbor) {
      if (visited_.find(*neighbor) == visited_.end()) {
        ++index_;
        to_visit_.push(std::make_shared<TraversalNode<T>>(
            *neighbor, node->depth + 1, index_, node));
      }
      visited_.insert(node->value);
    }
    return *this;
  }

  TraversalNode<T> &operator*() { return *to_visit_.front(); }
};

struct BFSIteratorEnd {};

template <class T, class NeighborIterator,
          class NeighborIteratorEnd = NeighborIterator>
bool operator==(
    const BFSIterator<T, NeighborIterator, NeighborIteratorEnd> &lhs,
    const BFSIterator<T, NeighborIterator, NeighborIteratorEnd> &rhs) {
  return lhs.index_ == rhs.index_;
}

template <class T, class NeighborIterator,
          class NeighborIteratorEnd = NeighborIterator>
bool operator==(
    const BFSIterator<T, NeighborIterator, NeighborIteratorEnd> &lhs,
    const BFSIteratorEnd &rhs) {
  return lhs.empty();
}

template <class T, class NeighborIterator,
          class NeighborIteratorEnd = NeighborIterator>
bool operator==(
    const BFSIteratorEnd &lhs,
    const BFSIterator<T, NeighborIterator, NeighborIteratorEnd> &rhs) {
  return rhs.empty();
}

template <class T, class NeighborIterator,
          class NeighborIteratorEnd = NeighborIterator>
bool operator!=(
    const BFSIterator<T, NeighborIterator, NeighborIteratorEnd> &lhs,
    const BFSIteratorEnd &rhs) {
  return !(lhs == rhs);
}

template <class T, class NeighborIterator,
          class NeighborIteratorEnd = NeighborIterator>
bool operator!=(
    const BFSIteratorEnd &lhs,
    const BFSIterator<T, NeighborIterator, NeighborIteratorEnd> &rhs) {
  return !(lhs == rhs);
}

template <class T, class NeighborIterator,
          class NeighborIteratorEnd = NeighborIterator>
bool operator!=(
    const BFSIterator<T, NeighborIterator, NeighborIteratorEnd> &lhs,
    const BFSIterator<T, NeighborIterator, NeighborIteratorEnd> &rhs) {
  return !(lhs == rhs);
}

template <class T, class NeighborIterator,
          class NeighborIteratorEnd = NeighborIterator>
class BFS {
  const Graph<T, NeighborIterator, NeighborIteratorEnd> &graph_;
  const T &start_;

public:
  BFS(const Graph<T, NeighborIterator, NeighborIteratorEnd> &graph,
      const T &start)
      : graph_(graph), start_(start) {}

  BFSIterator<T, NeighborIterator, NeighborIteratorEnd> begin() {
    return BFSIterator<T, NeighborIterator, NeighborIteratorEnd>(graph_,
                                                                 start_);
  }

  BFSIteratorEnd end() { return BFSIteratorEnd(); }
};

template <class T, class NeighborIterator,
          class NeighborIteratorEnd = NeighborIterator>
class TopologicalTraversal {
  const Graph<T, NeighborIterator, NeighborIteratorEnd> &graph_;
  std::unordered_map<T, int> indegrees_;
  std::vector<T> to_visit_;

public:
  TopologicalTraversal(
      const Graph<T, NeighborIterator, NeighborIteratorEnd> &graph,
      const std::unordered_set<T> &values)
      : graph_(graph) {
    for (const auto &value : values) {
      for (auto neighbor = graph_.neighbors_begin(value);
           neighbor != graph_.neighbors_end(value); ++neighbor) {
        ++indegrees_[*neighbor];
      }
    }
    for (const auto &value : values) {
      if (indegrees_[value] == 0) {
        to_visit_.push_back(value);
      }
    }
  }

  bool hasnext() const { return !to_visit_.empty(); }

  const T next() {
    T value = std::move(to_visit_.back());
    to_visit_.pop_back();
    for (auto neighbor = graph_.neighbors_begin(value);
         neighbor != graph_.neighbors_end(value); ++neighbor) {
      --indegrees_[*neighbor];
      if (indegrees_[*neighbor] == 0) {
        auto search = indegrees_.find(*neighbor);
        if (search == indegrees_.end()) {
          throw "Failed to find neighbor in values";
        }
        to_visit_.push_back(search->first);
      }
    }

    return value;
  }
};

} // namespace graphs
} // namespace aoc

#endif
