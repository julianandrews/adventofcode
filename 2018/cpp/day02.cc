#include <experimental/optional>
#include <iostream>
#include <string>
#include <unordered_map>
#include <vector>

#include "utils.h"

int p1(const std::vector<std::string> &lines) {
  int two_counts = 0;
  int three_counts = 0;

  for (auto &line : lines) {
    std::unordered_map<char, int> counter;
    for (const auto c : line) {
      counter[c]++;
    }
    if (std::any_of(counter.begin(), counter.end(),
                    [](auto &pair) { return pair.second == 2; })) {
      two_counts++;
    }
    if (std::any_of(counter.begin(), counter.end(),
                    [](auto &pair) { return pair.second == 3; })) {
      three_counts++;
    }
  }

  return two_counts * three_counts;
}

std::experimental::optional<std::string>
p2(const std::vector<std::string> &lines) {
  std::vector<std::string> matches;
  for (int i = 0; i < lines.size(); i++) {
    for (int j = i + 1; j < lines.size(); j++) {
      std::string matching;
      for (int k = 0; k < lines[i].size(); k++) {
        if (lines[i][k] == lines[j][k]) {
          matching.push_back(lines[i][k]);
        }
      }
      if (matching.size() == lines[i].size() - 1) {
        matches.emplace_back(matching);
      }
    }
  }

  return matches.size() == 1
             ? std::experimental::optional<std::string>(matches[0])
             : std::experimental::nullopt;
}

int main() {
  std::vector<std::string> lines = aoc::utils::getlines();

  std::cout << "Part 1: " << p1(lines) << std::endl;
  std::cout << "Part 2: " << p2(lines).value_or("Failed to find unique box.")
            << std::endl;
}
