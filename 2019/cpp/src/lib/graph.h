#ifndef AOC_GRAPH_H
#define AOC_GRAPH_H

#include <memory>

namespace aoc {
namespace graphs {

template <class T, class Neighbors> class Graph {
public:
  virtual Neighbors neighbors(const T &value) const = 0;
};

} // namespace graphs
} // namespace aoc

#endif
