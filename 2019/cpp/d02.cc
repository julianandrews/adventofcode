#include <iostream>
#include <vector>

#include "intcode.h"
#include "utils.h"

using intcode::VM;

int p1(const std::vector<int> &program) {
  VM vm = VM(program, 12, 2);
  vm.run();
  return vm.output();
}

int p2(const std::vector<int> &program) {
  for (int noun = 0; noun < 100; ++noun) {
    for (int verb = 0; verb < 100; ++verb) {
      VM vm = VM(program, noun, verb);
      vm.run();
      if (vm.output() == 19690720) {
        return 100 * noun + verb;
      }
    }
  }

  return -1;
}

int main() {
  std::string line;
  getline(std::cin, line);

  std::vector<int> program;
  for (std::string s : aoc::utils::split(line, ',')) {
    program.push_back(std::stoi(s));
  }

  std::cout << "Part 1: " << p1(program) << std::endl;
  std::cout << "Part 2: " << p2(program) << std::endl;
}
