#include <algorithm>
#include <algorithm>
#include <string>

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

} // namespace utils
} // namespace aoc
