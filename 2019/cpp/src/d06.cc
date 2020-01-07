#include <iostream>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <vector>

#include "graphs.h"
#include "utils.h"

class OrbitGraph : public aoc::graphs::Graph<std::string> {
  std::unordered_map<std::string, std::unordered_set<std::string>> orbits_;

public:
  OrbitGraph(std::vector<std::string> lines) {
    for (const auto &line : lines) {
      const auto parts = aoc::utils::split(line, ')');
      if (parts.size() != 2) {
        throw "Failed to parse line";
      }
      orbits_[parts[0]].insert(parts[1]);
      orbits_[parts[1]].insert(parts[0]);
    }
  }

  const std::unordered_set<std::string> &
  neighbors(const std::string &body) const override {
    return orbits_.at(body);
  }
};

int p1(const OrbitGraph &orbit_graph) {
  auto traversal = aoc::graphs::BFSTraversal<std::string>(orbit_graph, "COM");
  int total = 0;
  while (traversal.hasnext()) {
    total += traversal.next()->depth;
  }

  return total;
}

int p2(const OrbitGraph &orbit_graph) {
  auto traversal = aoc::graphs::BFSTraversal<std::string>(orbit_graph, "YOU");
  while (traversal.hasnext()) {
    auto node = traversal.next();
    if (node->value == "SAN") {
      return node->depth - 2;
    }
  }

  throw "Failed to find SAN!";
}

int main() {
  std::vector<std::string> lines = aoc::utils::getlines();
  const OrbitGraph orbits = OrbitGraph(lines);

  std::cout << "Part 1: " << p1(orbits) << std::endl;
  std::cout << "Part 2: " << p2(orbits) << std::endl;
}
