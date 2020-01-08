#include <iostream>
#include <vector>

#include "intcode.h"
#include "utils.h"

using aoc::intcode::Op;
using aoc::intcode::VM;

long long always_one() { return 1; }
long long always_two() { return 2; }

long long p1(const std::vector<long long> &program) {
  VM vm = VM(program, &always_one);
  auto output = vm.get_next_output();

  return output.value();
}

long long p2(const std::vector<long long> &program) {
  VM vm = VM(program, &always_two);
  auto output = vm.get_next_output();

  return output.value();
}

int main() {
  std::string line;
  getline(std::cin, line);

  std::vector<long long> program;
  for (std::string s : aoc::utils::split(line, ',')) {
    program.push_back(std::stoi(s));
  }

  std::cout << "Part 1: " << p1(program) << std::endl;
  std::cout << "Part 2: " << p2(program) << std::endl;
}
