#ifndef AOC_GRAPHS_H
#define AOC_GRAPHS_H

#include <iostream>
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
    to_visit_.push(std::make_shared<TraversalNode<T>>(start, 0, 0));
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

template <class T> class TopologicalTraversal {
  const Graph<T> &graph_;
  std::unordered_map<T, int> indegrees_;
  std::vector<T> to_visit_;

public:
  TopologicalTraversal(const Graph<T> &graph,
                       const std::unordered_set<T> &values)
      : graph_(graph) {
    for (const auto &value : values) {
      for (const auto &neighbor : graph_.neighbors(value)) {
        ++indegrees_[neighbor];
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
    for (const auto &neighbor : graph_.neighbors(value)) {
      --indegrees_[neighbor];
      if (indegrees_[neighbor] == 0) {
        auto search = indegrees_.find(neighbor);
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
