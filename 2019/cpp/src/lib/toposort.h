#include <iterator>
#include <stdexcept>
#include <unordered_map>
#include <unordered_set>
#include <vector>

#include "graph.h"

namespace aoc {
namespace graphs {
namespace toposort {

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

} // namespace toposort
} // namespace graphs
} // namespace aoc
