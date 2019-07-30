#include <algorithm>
#include <string>
#include <vector>

#include "point.h"
#include "utils.h"

using aoc::point::Point;

enum Action { TURN_ON, TURN_OFF, TOGGLE };

class Instruction {
public:
  static Instruction make_instruction(const std::string &s) {
    auto words = aoc::utils::split(s, ' ');
    Action action =
        words[0] == "toggle"
            ? Action::TOGGLE
            : (words[1] == "on" ? Action::TURN_ON : Action::TURN_OFF);

    return Instruction(action, parse_point(words[words.size() - 3]),
                       parse_point(words[words.size() - 1]));
  }

  Action action;
  Point lower_left;
  Point upper_right;

private:
  Instruction(Action action, Point lower_left, Point upper_right)
      : action(action), lower_left(lower_left), upper_right(upper_right) {}

  static Point parse_point(std::string &s) {
    auto parts = aoc::utils::split(s, ',');
    return Point(std::stoi(parts[0]), std::stoi(parts[1]));
  }
};

typedef int (*ActionHandler)(Action, int);

int get_brightness(const std::vector<Instruction> &instructions,
                   ActionHandler do_action) {
  int lights[1000][1000] = {{0}};

  for (const auto &i : instructions) {
    for (int x = i.lower_left.x; x <= i.upper_right.x; ++x) {
      for (int y = i.lower_left.y; y <= i.upper_right.y; ++y) {
        lights[x][y] = do_action(i.action, lights[x][y]);
      }
    }
  }

  int brightness = 0;
  for (int x = 0; x < 1000; ++x) {
    for (int y = 0; y < 1000; ++y) {
      brightness += lights[x][y];
    }
  }

  return brightness;
}

int simple_action(Action action, int value) {
  switch (action) {
  case Action::TURN_ON:
    return 1;
  case Action::TURN_OFF:
    return 0;
  default:
    return !value;
  }
}

int p1(const std::vector<Instruction> &instructions) {
  return get_brightness(instructions, *simple_action);
}

int fancy_action(Action action, int value) {
  switch (action) {
  case Action::TURN_ON:
    return value + 1;
  case Action::TURN_OFF:
    return std::max(0, value - 1);
  default:
    return value + 2;
  }
}

int p2(const std::vector<Instruction> &instructions) {
  return get_brightness(instructions, *fancy_action);
}

int main() {
  std::vector<std::string> lines = aoc::utils::get_trimmed_lines();
  std::vector<Instruction> instructions;
  std::transform(lines.begin(), lines.end(), std::back_inserter(instructions),
                 Instruction::make_instruction);

  try {
    std::cout << "Part 1: " << p1(instructions) << std::endl;
    std::cout << "Part 2: " << p2(instructions) << std::endl;
  } catch (const std::exception &exception) {
    std::cerr << exception.what() << std::endl;
    return 1;
  }
}
