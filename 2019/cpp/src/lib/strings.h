#ifndef AOC_UTILS_H
#define AOC_UTILS_H

#include <algorithm>
#include <iomanip>
#include <iostream>
#include <sstream>
#include <string>
#include <string_view>
#include <vector>

namespace aoc {
namespace strings {

std::vector<std::string> split(const std::string &s, const char delim) {
  std::vector<std::string> items;
  std::stringstream ss(s);
  std::string item;
  while (std::getline(ss, item, delim)) {
    items.push_back(item);
  }

  return items;
}

std::string join(const std::vector<std::string> &strings,
                 const std::string &delim) {
  std::string joined;
  for (const auto &s : strings) {
    if (!joined.empty())
      joined += delim;
    joined += s;
  }
  return joined;
}

std::string replace_substring(std::string subject,
                              const std::string_view search,
                              const std::string_view replace) {
  std::size_t pos = 0;
  while ((pos = subject.find(search, pos)) != std::string::npos) {
    subject.replace(pos, search.length(), replace);
    pos += replace.length();
  }
  return subject;
}

std::string trim(const std::string &s) {
  auto start = std::find_if_not(s.begin(), s.end(),
                                [](int c) { return std::isspace(c); });
  auto end = std::find_if_not(s.rbegin(), s.rend(),
                              [](int c) { return std::isspace(c); })
                 .base();

  return start < end ? std::string(start, end) : std::string();
}

std::vector<std::string> getlines() {
  std::vector<std::string> lines;

  std::string buffer;
  while (getline(std::cin, buffer)) {
    lines.push_back(buffer);
  }

  return lines;
}

template <class T> std::string zero_pad(T value, int width) {
  std::stringstream ss;
  ss << std::setw(width) << std::setfill('0') << value;
  return ss.str();
}

} // namespace strings
} // namespace aoc

#endif
