#ifndef AOC_DIRECTION_H_
#define AOC_DIRECTION_H_

#include <stdexcept>

#include "point.h"

namespace aoc {
namespace direction {

enum class Direction {
  NORTH = 0,
  EAST = 1,
  SOUTH = 2,
  WEST = 3,
};

Direction reverse(Direction d) {
  return static_cast<Direction>((static_cast<int>(d) + 2) % 4);
}

Direction right_turn(Direction d) {
  return static_cast<Direction>((static_cast<int>(d) + 1) % 4);
}

Direction left_turn(Direction d) {
  return static_cast<Direction>((static_cast<int>(d) + 3) % 4);
}

template <class T> Direction from_offset(aoc::point::Point<T, 2> offset) {
  if (offset == aoc::point::Point<T, 2>({0, 1})) {
    return Direction::NORTH;
  } else if (offset == aoc::point::Point<T, 2>({0, -1})) {
    return Direction::SOUTH;
  } else if (offset == aoc::point::Point<T, 2>({1, 0})) {
    return Direction::EAST;
  } else if (offset == aoc::point::Point<T, 2>({-1, 0})) {
    return Direction::WEST;
  } else {
    throw std::invalid_argument("Invalid offset for direction.");
  }
}

template <class T> aoc::point::Point<T, 2> offset(Direction d) {
  switch (d) {
  case Direction::NORTH:
    return {0, 1};
  case Direction::SOUTH:
    return {0, -1};
  case Direction::EAST:
    return {1, 0};
  case Direction::WEST:
    return {-1, 0};
  default:
    throw std::invalid_argument("Unexpected direction!");
  }
}

template <class T>
aoc::point::Point<T, 2> step(aoc::point::Point<T, 2> p, Direction d) {
  auto off = offset<T>(d);
  return {p[0] + off[0], p[1] + off[1]};
}

} // namespace direction
} // namespace aoc

#endif
