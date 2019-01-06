#include <iostream>
#include <numeric>
#include <string>
#include <unordered_set>
#include <vector>

#include "utils.h"

int p1(const std::vector<int> &deltas) {
  return std::accumulate(deltas.begin(), deltas.end(), 0);
}

int p2(const std::vector<int> &deltas) {
  std::unordered_set<int> seen = {};
  int i = 0;
  int frequency = 0;

  while (true) {
    if (seen.count(frequency)) {
      return frequency;
    }
    seen.insert(frequency);
    frequency += deltas[i];
    i = (i + 1) % deltas.size();
  }
}

int main() {
  std::vector<std::string> lines = aoc::utils::getlines();

  std::vector<int> deltas;
  for (std::string line : lines) {
    deltas.push_back(std::stoi(line));
  }

  std::cout << "Part 1: " << p1(deltas) << std::endl;
  std::cout << "Part 2: " << p2(deltas) << std::endl;
}
