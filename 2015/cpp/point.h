#include <functional>

namespace aoc {
namespace point {

struct Point {
  int x;
  int y;

  bool operator==(const Point &p) const { return x == p.x && y == p.y; }
};

} // namespace point
} // namespace aoc

namespace std {
template <> struct hash<aoc::point::Point> {
  size_t operator()(const aoc::point::Point &p) const {
    return std::hash<int>()(p.x) ^ std::hash<int>()(p.y);
  }
};
} // namespace std
