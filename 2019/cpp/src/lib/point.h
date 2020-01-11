#ifndef AOC_POINT_H
#define AOC_POINT_H

#include <functional>

namespace aoc {
namespace point {

template <class T, std::size_t size> struct Point {
  T values[size];

  bool operator==(const Point &other) const {
    for (std::size_t i = 0; i < size; ++i) {
      if (values[i] != other.values[i])
        return false;
    }
    return true;
  }

  const T &operator[](int index) const { return values[index]; }
  T &operator[](int index) { return values[index]; }
};

} // namespace point
} // namespace aoc

namespace std {

template <class T, std::size_t size> struct hash<aoc::point::Point<T, size>> {
  std::size_t operator()(const aoc::point::Point<T, size> &p) const {
    std::size_t hash_value = 0;
    for (std::size_t i = 0; i < size; ++i) {
      hash_value ^= std::hash<T>()(p.values[i]);
    }
    return hash_value;
  }
};

} // namespace std

#endif
