#include <algorithm>
#include <iostream>
#include <optional>
#include <stdexcept>
#include <string>
#include <unordered_map>
#include <vector>

#include "direction.h"
#include "graphs.h"
#include "intcode.h"
#include "point.h"
#include "strings.h"

#include <limits.h>

using ::aoc::direction::Direction;

typedef ::aoc::point::Point<long long, 2> Coords;
typedef const ::std::vector<Coords> Neighbors;

enum class StatusCode { HIT_WALL = 0, MOVED = 1, FOUND_OXYGEN = 2 };

class Robot : public aoc::graphs::Graph<Coords, Neighbors> {
  aoc::intcode::VM vm_;
  std::unordered_map<Coords, StatusCode> ship_map_;
  Coords position_ = {0, 0};
  std::vector<Direction> route_;
  long long next_input_;
  bool explored_ = false;

  long long inputs() { return next_input_; }

  long long direction_input(Direction direction) {
    switch (direction) {
    case Direction::NORTH:
      return 1;
    case Direction::SOUTH:
      return 2;
    case Direction::WEST:
      return 3;
    case Direction::EAST:
      return 4;
    default:
      throw std::invalid_argument("Unexpected direction.");
    }
  }

  void try_move(Direction direction) {
    next_input_ = direction_input(direction);
    auto output = vm_.get_next_output();
    StatusCode status_code = static_cast<StatusCode>(*output);
    auto next_position = aoc::direction::step(position_, direction);
    ship_map_[next_position] = status_code;
    if (status_code != StatusCode::HIT_WALL) {
      position_ = next_position;
      route_.push_back(direction);
    }
  }

  void backtrack() {
    Direction direction = aoc::direction::reverse(route_.back());
    route_.pop_back();
    next_input_ = direction_input(direction);
    auto output = vm_.get_next_output();
    StatusCode status_code = static_cast<StatusCode>(*output);
    position_ = aoc::direction::step(position_, direction);
    if (status_code != ship_map_.at(position_)) {
      throw std::runtime_error("Inconsistent map data");
    }
  }

  std::optional<Direction> get_next_direction() {
    for (int i = 0; i < 4; ++i) {
      Direction direction = Direction(i);
      if (ship_map_.find(aoc::direction::step(position_, direction)) ==
          ship_map_.end()) {
        return direction;
      }
    }
    return std::nullopt;
  }

public:
  Robot(std::vector<long long> program)
      : vm_(program, [this] { return this->inputs(); }) {}

  void explore() {
    while (!explored_) {
      std::optional<Direction> direction = get_next_direction();
      if (direction.has_value()) {
        try_move(*direction);
      } else {
        if (!route_.empty()) {
          backtrack();
        } else {
          explored_ = true;
          break;
        }
      }
    }
  }

  StatusCode status_at(const Coords position) const {
    return ship_map_.at(position);
  }

  Neighbors neighbors(const Coords &position) const override {
    std::vector<Coords> neighbors;
    for (int i = 0; i < 4; ++i) {
      Direction direction = Direction(i);
      auto neighbor = aoc::direction::step(position, direction);
      auto search = ship_map_.find(neighbor);
      if (search != ship_map_.end() && search->second != StatusCode::HIT_WALL) {
        neighbors.push_back(neighbor);
      }
    }
    return neighbors;
  }
};

int p1(const Robot &robot) {
  auto bfs = aoc::graphs::BFS<Coords, Neighbors>(robot, {0, 0});
  for (const auto &node : bfs) {
    if (robot.status_at(node.value) == StatusCode::FOUND_OXYGEN) {
      return node.depth;
    }
  }
  return 0;
}

int p2(const Robot &robot) {
  // Find the oxygen.
  Coords oxygen_position;
  auto initial_bfs = aoc::graphs::BFS<Coords, Neighbors>(robot, {0, 0});
  for (const auto &node : initial_bfs) {
    if (robot.status_at(node.value) == StatusCode::FOUND_OXYGEN) {
      oxygen_position = node.value;
      break;
    }
  }

  // Find the farthest point from it.
  int depth;
  auto bfs = aoc::graphs::BFS<Coords, Neighbors>(robot, oxygen_position);
  for (const auto &node : bfs) {
    depth = node.depth;
  }
  return depth;
}

int main() {
  try {
    std::string line;
    getline(std::cin, line);

    std::vector<long long> program;
    for (std::string s : aoc::strings::split(line, ',')) {
      program.push_back(std::stoll(s));
    }
    Robot robot = Robot(program);
    robot.explore();

    std::cout << "Part 1: " << p1(robot) << std::endl;
    std::cout << "Part 2: " << p2(robot) << std::endl;
  } catch (const std::exception &e) {
    std::cerr << e.what() << std::endl;
    return 1;
  }
}
