#ifndef AOC_GRAPHS_H
#define AOC_GRAPHS_H

#include <graph.h>
#include <toposort.h>
#include <traversal.h>

namespace aoc {
namespace graphs {

template <class T, class Neighbors>
using Toposort = toposort::Toposort<T, Neighbors>;

template <class T, class Neighbors>
using BFS = traversal::GraphTraversal<T, Neighbors, traversal::BFSQ<T>>;

template <class T, class Neighbors>
using DFS = traversal::GraphTraversal<T, Neighbors, traversal::DFSQ<T>>;

} // namespace graphs
} // namespace aoc

#endif
