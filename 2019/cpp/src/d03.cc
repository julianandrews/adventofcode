#include <unordered_map>
#include <unordered_set>
#include <vector>

#include "direction.h"
#include "point.h"
#include "utils.h"

typedef ::aoc::point::Point<int, 2> Coords;
using ::aoc::direction::Direction;

Direction parse_direction(char c) {
  switch (c) {
  case 'U':
    return Direction::NORTH;
  case 'D':
    return Direction::SOUTH;
  case 'R':
    return Direction::EAST;
  case 'L':
    return Direction::WEST;
  default:
    throw "Unexpected Direction";
  }
}

class Wire {
  std::unordered_map<Coords, int> signal_distances_;

public:
  explicit Wire(const std::vector<std::string> &instructions) {
    int current_distance = 0;
    int x = 0;
    int y = 0;

    for (const auto &instruction : instructions) {
      Direction d = parse_direction(instruction.at(0));
      int distance = std::stoi(instruction.substr(1));

      Coords offset = aoc::direction::offset(d);
      for (int i = 0; i < distance; ++i) {
        x += offset.values[0];
        y += offset.values[1];
        ++current_distance;
        Coords coords = {x, y};
        if (!contains(coords)) {
          signal_distances_[coords] = current_distance;
        }
      }
    }
  }

  bool contains(const Coords &point) const {
    return signal_distances_.find(point) != signal_distances_.end();
  }

  int signal_distance(const Coords &point) const {
    return signal_distances_.at(point);
  }

  std::unordered_set<Coords> intersections(const Wire &other) const {
    std::unordered_set<Coords> points;

    for (const auto &key_value_pair : signal_distances_) {
      const auto &point = key_value_pair.first;
      if (other.contains(point)) {
        points.insert(point);
      }
    }

    return points;
  }
};

int p1(const Wire &first_wire, const Wire &second_wire) {
  const auto intersections = first_wire.intersections(second_wire);
  int smallest = abs(intersections.begin()->values[0]) +
                 abs(intersections.begin()->values[1]);
  for (const auto &point : intersections) {
    int value = abs(point.values[0]) + abs(point.values[1]);
    if (value < smallest)
      smallest = value;
  }

  return smallest;
}

int p2(const Wire &first_wire, const Wire &second_wire) {
  const auto intersections = first_wire.intersections(second_wire);
  int smallest = first_wire.signal_distance(*intersections.begin()) +
                 second_wire.signal_distance(*intersections.begin());
  for (const auto &point : intersections) {
    int value =
        first_wire.signal_distance(point) + second_wire.signal_distance(point);
    if (value < smallest)
      smallest = value;
  }

  return smallest;
}

int main() {
  std::vector<std::string> lines = aoc::utils::getlines();
  Wire first_wire = Wire(aoc::utils::split(lines.at(0), ','));
  Wire second_wire = Wire(aoc::utils::split(lines.at(1), ','));

  std::cout << "Part 1: " << p1(first_wire, second_wire) << std::endl;
  std::cout << "Part 2: " << p2(first_wire, second_wire) << std::endl;

  return 0;
}
