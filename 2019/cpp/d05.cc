#include <iostream>
#include <vector>

#include "intcode.h"
#include "utils.h"

using intcode::VM, intcode::Op;

int p1(const std::vector<int> &program) {
  VM vm = VM(program, 1);
  do {
    vm.step();
  } while (!vm.diagnostic_code());

  return vm.diagnostic_code();
}

int p2(const std::vector<int> &program) {
  VM vm = VM(program, 5);
  do {
    vm.step();
  } while (!vm.diagnostic_code());

  return vm.diagnostic_code();
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
