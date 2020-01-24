#ifndef AOC_GRAPHS_H
#define AOC_GRAPHS_H

#include <iostream>
#include <iterator>
#include <memory>
#include <queue>
#include <stdexcept>
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
 *
 * Neighbors must implement `begin` and `end` which return iterators.
 */
template <class T, class Neighbors> class Graph {
public:
  virtual Neighbors neighbors(const T &value) const = 0;
};

template <class T, class Neighbors> class BFSIterator {
  const Graph<T, Neighbors> &graph_;
  std::unordered_set<T> visited_;
  std::queue<std::shared_ptr<TraversalNode<T>>> to_visit_;
  int index_ = 0;

public:
  using iterator_category = std::input_iterator_tag;
  using value_type = TraversalNode<T>;
  using reference = TraversalNode<T> &;
  using pointer = std::shared_ptr<TraversalNode<T>>;
  using diference_type = void;

  BFSIterator(const Graph<T, Neighbors> &graph, const T start) : graph_(graph) {
    to_visit_.push(std::make_shared<TraversalNode<T>>(start, 0, 0));
  }

  BFSIterator(const BFSIterator &other)
      : graph_(other.graph_), visited_(other.visited_),
        to_visit_(other.to_visit_), index_(other.index_) {}

  bool empty() const { return to_visit_.empty(); }

  BFSIterator<T, Neighbors> &operator++() {
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
    return *this;
  }

  TraversalNode<T> &operator*() { return *to_visit_.front(); }
};

struct BFSIteratorEnd {};

template <class T, class Neighbors>
bool operator==(const BFSIterator<T, Neighbors> &lhs,
                const BFSIterator<T, Neighbors> &rhs) {
  return lhs.index_ == rhs.index_;
}

template <class T, class Neighbors>
bool operator==(const BFSIterator<T, Neighbors> &lhs,
                const BFSIteratorEnd &rhs) {
  return lhs.empty();
}

template <class T, class Neighbors>
bool operator==(const BFSIteratorEnd &lhs,
                const BFSIterator<T, Neighbors> &rhs) {
  return rhs.empty();
}

template <class T, class Neighbors>
bool operator!=(const BFSIterator<T, Neighbors> &lhs,
                const BFSIteratorEnd &rhs) {
  return !(lhs == rhs);
}

template <class T, class Neighbors>
bool operator!=(const BFSIteratorEnd &lhs,
                const BFSIterator<T, Neighbors> &rhs) {
  return !(lhs == rhs);
}

template <class T, class Neighbors>
bool operator!=(const BFSIterator<T, Neighbors> &lhs,
                const BFSIterator<T, Neighbors> &rhs) {
  return !(lhs == rhs);
}

template <class T, class Neighbors> class BFS {
  const Graph<T, Neighbors> &graph_;
  const T &start_;

public:
  BFS(const Graph<T, Neighbors> &graph, const T &start)
      : graph_(graph), start_(start) {}

  BFSIterator<T, Neighbors> begin() {
    return BFSIterator<T, Neighbors>(graph_, start_);
  }

  BFSIteratorEnd end() { return BFSIteratorEnd(); }
};

template <class T, class Neighbors> class TopoIterator {
  using iterator_category = std::input_iterator_tag;
  using value_type = T;
  using reference = T &;
  using pointer = T *;
  using diference_type = void;

  const Graph<T, Neighbors> &graph_;
  std::unordered_map<T, int> indegrees_;
  std::vector<T> to_visit_;
  int index_ = 0;

public:
  TopoIterator(const Graph<T, Neighbors> &graph,
               const std::unordered_set<T> &values)
      : graph_(graph) {
    for (const auto &value : values) {
      for (auto neighbor : graph_.neighbors(value)) {
        ++indegrees_[neighbor];
      }
    }
    for (const auto &value : values) {
      if (indegrees_[value] == 0) {
        to_visit_.push_back(value);
      }
    }
  }

  TopoIterator(const TopoIterator &other)
      : graph_(other.graph_), indegrees_(other.indegrees_),
        to_visit_(other.to_visit_), index_(other.index_) {}

  bool empty() const { return to_visit_.empty(); }

  TopoIterator<T, Neighbors> &operator++() {
    T value = std::move(to_visit_.back());
    to_visit_.pop_back();
    for (auto neighbor : graph_.neighbors(value)) {
      --indegrees_[neighbor];
      if (indegrees_[neighbor] == 0) {
        auto search = indegrees_.find(neighbor);
        if (search == indegrees_.end()) {
          throw std::runtime_error("Failed to find neighbor in values");
        }
        to_visit_.push_back(search->first);
      }
    }

    ++index_;
    return *this;
  }

  T &operator*() { return to_visit_.back(); }
};

struct TopoIteratorEnd {};

template <class T, class Neighbors> class Toposort {
  const Graph<T, Neighbors> &graph_;
  const std::unordered_set<T> &values_;

public:
  Toposort(const Graph<T, Neighbors> &graph,
           const std::unordered_set<T> &values)
      : graph_(graph), values_(values) {}

  TopoIterator<T, Neighbors> begin() {
    return TopoIterator<T, Neighbors>(graph_, values_);
  }

  TopoIteratorEnd end() { return TopoIteratorEnd(); }
};

template <class T, class Neighbors>
bool operator==(const TopoIterator<T, Neighbors> &lhs,
                const TopoIterator<T, Neighbors> &rhs) {
  return lhs.index_ == rhs.index_;
}

template <class T, class Neighbors>
bool operator==(const TopoIterator<T, Neighbors> &lhs,
                const TopoIteratorEnd &rhs) {
  return lhs.empty();
}

template <class T, class Neighbors>
bool operator==(const TopoIteratorEnd &lhs,
                const TopoIterator<T, Neighbors> &rhs) {
  return rhs.empty();
}

template <class T, class Neighbors>
bool operator!=(const TopoIterator<T, Neighbors> &lhs,
                const TopoIteratorEnd &rhs) {
  return !(lhs == rhs);
}

template <class T, class Neighbors>
bool operator!=(const TopoIteratorEnd &lhs,
                const TopoIterator<T, Neighbors> &rhs) {
  return !(lhs == rhs);
}

template <class T, class Neighbors>
bool operator!=(const TopoIterator<T, Neighbors> &lhs,
                const TopoIterator<T, Neighbors> &rhs) {
  return !(lhs == rhs);
}

} // namespace graphs
} // namespace aoc

#endif
