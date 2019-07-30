#include <unordered_set>

#include "point.h"
#include "utils.h"

using ::aoc::point::Point;

enum Direction { NORTH, EAST, SOUTH, WEST };

class Santa {
public:
  Santa() : location(0, 0) {}

  void move(const Direction direction) {
    switch (direction) {
    case NORTH:
      location.y++;
      break;
    case EAST:
      location.x++;
      break;
    case SOUTH:
      location.y--;
      break;
    case WEST:
      location.x--;
      break;
    }
  }

  const Point get_location() const { return location; }

private:
  Point location;
};

Direction dir_from_char(const char c) {
  switch (c) {
  case '^':
    return Direction::NORTH;
  case '>':
    return Direction::EAST;
  case 'v':
    return Direction::SOUTH;
  case '<':
    return Direction::WEST;
  default:
    throw std::runtime_error("Unexpected instruction: " + std::string(1, c));
  }
}

int p1(const std::string &instructions) {
  Santa santa;
  std::unordered_set<Point> seen = {santa.get_location()};
  for (const char instruction : instructions) {
    santa.move(dir_from_char(instruction));
    seen.insert(santa.get_location());
  }

  return seen.size();
}

int p2(const std::string &instructions) {
  std::vector<Santa> santas(2);
  std::unordered_set<Point> seen;
  for (const auto &santa : santas) {
    seen.insert(santa.get_location());
  }

  for (std::size_t i = 0; i < instructions.size(); ++i) {
    Santa &santa = santas[i % santas.size()];
    santa.move(dir_from_char(instructions[i]));
    seen.insert(santa.get_location());
  }

  return seen.size();
}

int main() {
  std::string instructions = aoc::utils::get_trimmed_line();

  try {
    std::cout << "Part 1: " << p1(instructions) << std::endl;
    std::cout << "Part 2: " << p2(instructions) << std::endl;
  } catch (const std::exception &exception) {
    std::cerr << exception.what() << std::endl;
    return 1;
  }
}
