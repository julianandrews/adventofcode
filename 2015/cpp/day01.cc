#include <string>

#include "utils.h"

int p1(const std::string &instructions) {
  int floor = 0;

  for (char const &c : instructions) {
    if (c == '(') {
      floor++;
    } else if (c == ')') {
      floor--;
    } else {
      throw std::runtime_error("Unexpected instruction: " + std::string(1, c));
    }
  }

  return floor;
}

int p2(const std::string &instructions) {
  int floor = 0;
  int position = 0;

  for (char const &c : instructions) {
    position += 1;
    if (c == '(') {
      floor++;
    } else if (c == ')') {
      floor--;
    } else {
      throw std::runtime_error("Unexpected instruction: " + std::string(1, c));
    }
    if (floor < 0) {
      return position;
    }
  }
  throw std::runtime_error("Failed to enter basement.");
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
