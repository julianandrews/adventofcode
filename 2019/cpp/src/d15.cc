#include <algorithm>
#include <iostream>
#include <optional>
#include <string>
#include <unordered_map>
#include <vector>

#include "direction.h"
#include "graphs.h"
#include "intcode.h"
#include "point.h"
#include "strings.h"

using ::aoc::direction::Direction;
using ::aoc::graphs::BFSTraversal;

typedef ::aoc::point::Point<long long, 2> Coords;

enum class StatusCode { HIT_WALL = 0, MOVED = 1, FOUND_OXYGEN = 2 };

class NeighborIterator {
  size_t index_;
  std::vector<Coords> neighbors_;

public:
  NeighborIterator(std::vector<Coords> neighbors, size_t index = 0)
      : neighbors_(neighbors), index_(index) {}

  NeighborIterator &operator++() {
    ++index_;
    return *this;
  }

  Coords &operator*() { return neighbors_.at(index_); }

  friend bool operator==(const NeighborIterator &, const NeighborIterator &);
  friend bool operator!=(const NeighborIterator &, const NeighborIterator &);
};

bool operator==(const NeighborIterator &lhs, const NeighborIterator &rhs) {
  for (const auto &neighbor : lhs.neighbors_) {
    if (std::find(rhs.neighbors_.begin(), rhs.neighbors_.end(), neighbor) ==
        rhs.neighbors_.end()) {
      return false;
    }
  }

  return lhs.neighbors_.size() == rhs.neighbors_.size() &&
         lhs.index_ == rhs.index_;
}

bool operator!=(const NeighborIterator &lhs, const NeighborIterator &rhs) {
  return !(lhs == rhs);
}

class Robot : public aoc::graphs::Graph<Coords, NeighborIterator> {
  aoc::intcode::VM vm_;
  std::unordered_map<Coords, StatusCode> ship_map_;
  Coords position_ = {0, 0};
  std::vector<Direction> route_;
  long long next_input_;
  bool explored_;

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
      throw "Unexpected direction.";
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
      throw "Inconsistent map data";
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

  std::vector<Coords> get_neighbors(const Coords &position) const {
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

  NeighborIterator neighbors_begin(const Coords &position) const override {
    if (!explored_) {
      throw "Must explore first!";
    }
    return NeighborIterator(get_neighbors(position));
  }

  NeighborIterator neighbors_end(const Coords &position) const override {
    if (!explored_) {
      throw "Must explore first!";
    }
    auto neighbors = get_neighbors(position);
    return NeighborIterator(neighbors, neighbors.size());
  }
};

int p1(const Robot &robot) {
  auto traversal = BFSTraversal<Coords, NeighborIterator>(robot, {0, 0});
  while (traversal.hasnext()) {
    auto node = traversal.next();
    if (robot.status_at(node->value) == StatusCode::FOUND_OXYGEN) {
      return node->depth;
    }
  }
  return 0;
}

int p2(const Robot &robot) {
  // Find the oxygen.
  Coords start_position;
  auto initial_traversal =
      BFSTraversal<Coords, NeighborIterator>(robot, {0, 0});
  while (initial_traversal.hasnext()) {
    auto node = initial_traversal.next();
    if (robot.status_at(node->value) == StatusCode::FOUND_OXYGEN) {
      start_position = node->value;
      break;
    }
  }

  // Find the farthest point from it.
  int depth;
  auto traversal =
      BFSTraversal<Coords, NeighborIterator>(robot, start_position);
  while (traversal.hasnext()) {
    depth = traversal.next()->depth;
  }
  return depth;
}

int main() {
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
}
