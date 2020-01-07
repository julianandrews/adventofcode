#ifndef AOC_POINT_H
#define AOC_POINT_H

#include <functional>

namespace aoc {
namespace point {

template <class T> struct Point {
  T x;
  T y;

  bool operator==(const Point &other) const {
    return x == other.x and y == other.y;
  }

  Point(T x, T y) : x(x), y(y) {}
};

} // namespace point
} // namespace aoc

namespace std {

template <class T> struct hash<aoc::point::Point<T>> {
  std::size_t operator()(const aoc::point::Point<T> &p) const {
    return std::hash<T>()(p.x) ^ std::hash<T>()(p.y);
  }
};

} // namespace std

#endif
