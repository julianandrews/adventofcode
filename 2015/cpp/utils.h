#ifndef AOC_UTILS
#define AOC_UTILS

#include <algorithm>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

namespace aoc {
namespace utils {

std::vector<std::string> split(const std::string &s, const char delim) {
  std::vector<std::string> items;
  std::stringstream ss(s);
  std::string item;
  while (std::getline(ss, item, delim)) {
    items.push_back(item);
  }

  return items;
}

std::string trim(const std::string &s) {
  auto start = std::find_if_not(s.begin(), s.end(),
                                [](int c) { return std::isspace(c); });
  auto end = std::find_if_not(s.rbegin(), s.rend(),
                              [](int c) { return std::isspace(c); })
                 .base();

  return start < end ? std::string(start, end) : std::string();
}

std::string get_trimmed_line() {
  std::string buffer;
  std::getline(std::cin, buffer);
  return trim(buffer);
}

std::vector<std::string> get_trimmed_lines() {
  std::vector<std::string> lines;

  std::string buffer;
  while (getline(std::cin, buffer)) {
    lines.push_back(trim(buffer));
  }

  return lines;
}

} // namespace utils
} // namespace aoc

#endif
