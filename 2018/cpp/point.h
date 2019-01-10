#include <cstdlib>
#include <tuple>
#include <vector>

namespace aoc {
namespace point {

class Point {
 public:
  long x, y;

  Point(long x, long y) : x(x), y(y) {}

  ulong manhattan_distance(const Point &other) const {
    return std::abs(x - other.x) + std::abs(y - other.y);
  }

  friend bool operator==(const Point &lhs, const Point &rhs) {
    return lhs.x == rhs.x && lhs.y == rhs.y;
  }

  friend bool operator<(const Point &lhs, const Point &rhs) {
    return std::tie(lhs.x, lhs.y) < std::tie(rhs.x, rhs.y);
  }
};

}  // namespace point
}  // namespace aoc

namespace std {
template <>
struct hash<aoc::point::Point> {
  size_t operator()(const aoc::point::Point &p) const {
    const size_t prime = 257;
    size_t h = std::hash<ulong>()(p.x);
    h = h * prime + std::hash<ulong>()(p.y);
    return h;
  }
};
}  // namespace std
