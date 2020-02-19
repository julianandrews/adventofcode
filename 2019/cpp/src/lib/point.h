#ifndef AOC_POINT_H
#define AOC_POINT_H

#include <functional>
#include <iostream>

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

  bool operator!=(const Point &other) const { return !(operator==(other)); }

  const T &operator[](int index) const { return values[index]; }
  T &operator[](int index) { return values[index]; }
};

namespace details {

std::size_t cantor_pairing(std::size_t a, std::size_t b) {
  return (a + b) * (a + b + 1) / 2 + b;
}

} // namespace details

} // namespace point
} // namespace aoc

namespace std {

template <class T, std::size_t size> struct hash<aoc::point::Point<T, size>> {
  std::size_t operator()(const aoc::point::Point<T, size> &point) const {
    std::size_t hash_value = 0;
    for (std::size_t i = 0; i < size; ++i) {
      hash_value = aoc::point::details::cantor_pairing(
          hash_value, std::hash<T>()(point.values[i]));
    }
    return hash_value;
  }
};

} // namespace std

template <class T, std::size_t size>
std::ostream &operator<<(std::ostream &os,
                         const aoc::point::Point<T, size> &point) {
  os << '(';
  for (std::size_t i = 0; i < size; ++i) {
    os << point.values[i];
    if (i != size - 1) {
      os << ", ";
    }
  }
  os << ')';
  return os;
}

#endif
