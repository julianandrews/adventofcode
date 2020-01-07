#include <unordered_map>
#include <unordered_set>
#include <vector>

#include "utils.h"

namespace std {
template <> struct hash<std::pair<int, int>> {
    inline size_t operator()(const std::pair<int, int> &v) const {
        std::hash<int> int_hasher;
        return int_hasher(v.first) ^ int_hasher(v.second);
    }
};
}  // namespace std

typedef std::pair<int, int> Point;

enum class Direction { UP = 'U', DOWN = 'D', LEFT = 'L', RIGHT= 'R' };

class Wire {
  std::unordered_map<Point, int> signal_distances_;

 public:
  explicit Wire(const std::vector<std::string> &instructions) {
    int current_distance = 0;
    int x = 0;
    int y = 0;

    for (const auto &instruction : instructions) {
      Direction d = Direction(instruction.at(0));
      int distance = std::stoi(instruction.substr(1));

      for (int i = 0; i < distance; ++i) {
        switch (d) {
          case Direction::UP:
            ++y;
            break;
          case Direction::DOWN:
            --y;
            break;
          case Direction::RIGHT:
            ++x;
            break;
          case Direction::LEFT:
            --x;
            break;
        }
        ++current_distance;
        Point coords = {x, y};
        if (!contains(coords)) {
          signal_distances_[coords] = current_distance;
        }
      }
    }
  }

  bool contains(const Point &point) const {
    return signal_distances_.find(point) != signal_distances_.end();
  }

  int signal_distance(const Point &point) const {
    return signal_distances_.at(point);
  }

  std::unordered_set<Point> intersections(const Wire &other) const {
    std::unordered_set<Point> points;

    for (const auto &key_value_pair : signal_distances_) {
      const auto& point = key_value_pair.first;
      if (other.contains(point)) {
        points.insert(point);
      }
    }

    return points;
  }
};

int p1(const Wire &first_wire, const Wire &second_wire) {
  const auto intersections = first_wire.intersections(second_wire);
  int smallest =
      abs(intersections.begin()->first) + abs(intersections.begin()->second);
  for (const auto &point : intersections) {
    int value = abs(point.first) + abs(point.second);
    if (value < smallest) smallest = value;
  }

  return smallest;
}

int p2(const Wire &first_wire, const Wire &second_wire) {
  const auto intersections = first_wire.intersections(second_wire);
  int smallest = first_wire.signal_distance(*intersections.begin()) + second_wire.signal_distance(*intersections.begin());
  for (const auto &point : intersections) {
    int value = first_wire.signal_distance(point) + second_wire.signal_distance(point);
    if (value < smallest) smallest = value;
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
