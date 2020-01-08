#ifndef AOC_DIRECTION_H_
#define AOC_DIRECTION_H_

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

aoc::point::Point<int> offset(Direction d) {
  using ::aoc::point::Point;
  switch (d) {
    case Direction::NORTH:
      return Point<int>(0, 1);
    case Direction::SOUTH:
      return Point<int>(0, -1);
    case Direction::EAST:
      return Point<int>(1, 0);
    case Direction::WEST:
      return Point<int>(-1, 0);
    default:
      throw "Unexpected direction!";
  }
}

}  // namespace direction
}  // namespace aoc

#endif
