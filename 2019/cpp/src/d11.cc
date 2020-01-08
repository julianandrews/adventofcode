#include <iostream>
#include <limits.h>
#include <optional>
#include <unordered_set>
#include <vector>

#include "direction.h"
#include "point.h"
#include "intcode.h"
#include "utils.h"

using ::aoc::direction::Direction;
using ::aoc::intcode::VM;
using ::aoc::point::Point;

struct PaintInstruction {
  Point<int> paint_location;
  Point<int> move_location;
  bool paint_white;
};

class Robot {
  VM vm_;
  Point<int> location_ = Point<int>(0, 0);
  Direction direction_ = Direction::NORTH;
  std::unordered_set<Point<int>> painted_panels_;

  bool panel_painted(const Point<int> p) const {
    return painted_panels_.find(p) != painted_panels_.end();
  }

 public:
  Robot(const std::vector<long long> &program)
      : vm_(program, [this] { return this->panel_painted(this->location_); }) {}

  void paint(const Point<int> p) {
    painted_panels_.insert(p);
  }

  std::optional<PaintInstruction> next_paint_instruction() {
    auto maybe_paint_white = vm_.get_next_output();
    auto maybe_turn_right = vm_.get_next_output();
    if (!maybe_paint_white.has_value() || !maybe_turn_right.has_value()) {
      return std::nullopt;
    }
    bool paint_white = maybe_paint_white.value();
    bool turn_right = maybe_turn_right.value();
    if (paint_white) {
      painted_panels_.insert(location_);
    } else {
      painted_panels_.erase(location_);
    }
    if (turn_right) {
      direction_ = aoc::direction::right_turn(direction_);
    } else {
      direction_ = aoc::direction::left_turn(direction_);
    }
    Point<int> paint_location = location_;
    Point<int> offset = aoc::direction::offset(direction_);
    location_.x += offset.x;
    location_.y += offset.y;
    return {{paint_location, location_, paint_white}};
  }

  std::string panel_string() {
    int min_x = INT_MAX;
    int max_x = INT_MIN;
    int min_y = INT_MAX;
    int max_y = INT_MIN;
    for (const auto &p : painted_panels_) {
      min_x = std::min(min_x, p.x);
      max_x = std::max(max_x, p.x);
      min_y = std::min(min_y, p.y);
      max_y = std::max(max_y, p.y);
    }

    std::stringstream s;
    for (int y = min_y; y <= max_y; ++y) {
      for (int x = min_x; x <= max_x; ++x) {
        s << (panel_painted(Point<int>(x, y)) ? "█" : " ");
      }
      s << "\n";
    }

    return s.str();
  }
};

int p1(const std::vector<long long> program) {
  Robot robot = Robot(program);
  std::unordered_set<Point<int>> painted_locations;
  std::optional<PaintInstruction> instruction;
  do {
    instruction = robot.next_paint_instruction();
    if (instruction.has_value() && instruction.value().paint_white) {
      painted_locations.insert(instruction.value().paint_location);
    }
  } while (instruction.has_value());

  return painted_locations.size();
}

std::string p2(const std::vector<long long> program) {
  Robot robot = Robot(program);
  robot.paint(Point(0, 0));
  std::optional<PaintInstruction> instruction;
  do {
    instruction = robot.next_paint_instruction();
  } while (instruction.has_value());

  std::vector<std::string> lines =
      aoc::utils::split(robot.panel_string(), '\n');
  std::stringstream ss;
  for (auto rit = lines.rbegin(); rit != lines.rend(); ++rit) {
    ss << *rit << std::endl;
  }

  return ss.str();
}

int main() {
  std::string line;
  getline(std::cin, line);

  std::vector<long long> program;
  for (std::string s : aoc::utils::split(line, ',')) {
    program.push_back(std::stoll(s));
  }

  std::cout << "Part 1: " << p1(program) << std::endl;
  std::cout << "Part 2:" << std::endl << p2(program) << std::endl;
}
