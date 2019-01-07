#include <iostream>
#include <string>
#include <vector>

#include "utils.h"

std::vector<char> collapse(const std::vector<char> &polymer) {
  std::vector<char> result;
  for (auto c : polymer) {
    while (result.size() and std::abs(result.back() - c) == 32) {
      result.pop_back();
      c = result.back();
      result.pop_back();
    }
    result.push_back(c);
  }

  return result;
}

int p1(const std::string &polymer) {
  auto collapsed = collapse({polymer.begin(), polymer.end()});
  return collapsed.size();
}

int p2(const std::string &polymer) {
  auto collapsed = collapse({polymer.begin(), polymer.end()});
  std::vector<int> lengths;

  for (char c = 'A'; c <= 'Z'; c++) {
    std::vector<char> candidate = collapsed;
    for (auto it = candidate.begin(); it != candidate.end();) {
      if (*it == c || *it == std::tolower(c)) {
        it = candidate.erase(it);
      } else {
        ++it;
      }
    }
    lengths.push_back(collapse(candidate).size());
  }

  return *std::min_element(lengths.begin(), lengths.end());
}

int main() {
  std::vector<std::string> lines = aoc::utils::getlines();
  std::sort(lines.begin(), lines.end());

  std::cout << "Part 1: " << p1(lines[0]) << std::endl;
  std::cout << "Part 2: " << p2(lines[0]) << std::endl;
}
