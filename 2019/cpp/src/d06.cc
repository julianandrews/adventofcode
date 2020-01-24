#include <iostream>
#include <stdexcept>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <vector>

#include "graphs.h"
#include "strings.h"

typedef const ::std::unordered_set<std::string> &Neighbors;
typedef ::aoc::graphs::BFS<std::string, Neighbors> OrbitGraphBFS;

class OrbitGraph : public aoc::graphs::Graph<std::string, Neighbors> {
  std::unordered_map<std::string, std::unordered_set<std::string>> orbits_;

public:
  OrbitGraph(std::vector<std::string> lines) {
    for (const auto &line : lines) {
      const auto parts = aoc::strings::split(line, ')');
      if (parts.size() != 2) {
        throw std::invalid_argument("Failed to parse line");
      }
      orbits_[parts[0]].insert(parts[1]);
      orbits_[parts[1]].insert(parts[0]);
    }
  }

  Neighbors neighbors(const std::string &body) const override {
    return orbits_.at(body);
  }
};

int p1(const OrbitGraph &orbit_graph) {
  auto bfs = OrbitGraphBFS(orbit_graph, "COM");
  int total = 0;
  for (const auto &node : bfs) {
    total += node.depth;
  }

  return total;
}

int p2(const OrbitGraph &orbit_graph) {
  auto bfs = OrbitGraphBFS(orbit_graph, "YOU");
  for (const auto &node : bfs) {
    if (node.value == "SAN") {
      return node.depth - 2;
    }
  }

  throw std::runtime_error("Failed to find SAN!");
}

int main() {
  try {
    std::vector<std::string> lines = aoc::strings::getlines();
    const OrbitGraph orbits = OrbitGraph(lines);

    std::cout << "Part 1: " << p1(orbits) << std::endl;
    std::cout << "Part 2: " << p2(orbits) << std::endl;
  } catch (const std::exception &e) {
    std::cerr << e.what() << std::endl;
    return 1;
  }
}
