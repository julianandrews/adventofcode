#include <algorithm>
#include <iostream>
#include <string>
#include <vector>

namespace aoc {
namespace utils {

std::string trim(std::string &s) {
  auto start = std::find_if_not(s.begin(), s.end(),
                                [](int c) { return std::isspace(c); });
  auto end = std::find_if_not(s.rbegin(), s.rend(), [](int c) {
               return std::isspace(c);
             }).base();

  return start < end ? std::string(start, end) : std::string();
}

std::vector<std::string> getlines() {
  std::vector<std::string> lines;

  std::string buffer;
  while (getline(std::cin, buffer)) {
    lines.push_back(trim(buffer));
  }

  return lines;
}

} // namespace utils
} // namespace aoc
