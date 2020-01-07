#include <iostream>

#include "utils.h"

int simple_fuel(int mass) { return std::max(0, mass / 3 - 2); }

int fuel(int mass) {
  int total_mass = 0;
  int new_mass = mass;
  do {
    new_mass = simple_fuel(new_mass);
    total_mass += new_mass;
  } while (new_mass > 0);

  return total_mass;
}

int p1(const std::vector<int> &masses) {
  int total_fuel = 0;
  for (int mass : masses) {
    total_fuel += simple_fuel(mass);
  }

  return total_fuel;
}

int p2(const std::vector<int> &masses) {
  int total_fuel = 0;
  for (int mass : masses) {
    total_fuel += fuel(mass);
  }

  return total_fuel;
}

int main() {
  std::vector<std::string> lines = aoc::utils::getlines();
  std::vector<int> masses;

  for (std::string s : lines) {
    masses.push_back(std::stoi(s));
  }

  std::cout << "Part 1: " << p1(masses) << std::endl;
  std::cout << "Part 2: " << p2(masses) << std::endl;
}
