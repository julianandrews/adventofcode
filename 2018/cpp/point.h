#include <tuple>
#include <vector>

namespace aoc {
namespace point {

class Point {
public:
  const long x, y;

  Point(long x, long y) : x(x), y(y) {}

  friend bool operator==(const Point &lhs, const Point &rhs) {
    return lhs.x == rhs.x && lhs.y == rhs.y;
  }

  friend bool operator<(const Point &lhs, const Point &rhs) {
    return std::tie(lhs.x, lhs.y) < std::tie(rhs.x, rhs.y);
  }
};

} // namespace aoc
} // namespace point

namespace std {
template <> struct hash<aoc::point::Point> {
  size_t operator()(const aoc::point::Point &p) const {
    const size_t prime = 257;
    size_t h = std::hash<ulong>()(p.x);
    h = h * prime + std::hash<ulong>()(p.y);
    return h;
  }
};
}
