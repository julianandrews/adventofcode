#include <iostream>
#include <vector>

#include "intcode.h"
#include "utils.h"

using intcode::ConstantInputs;
using intcode::Op;
using intcode::VM;

long long p1(const std::vector<long long> &program) {
  auto inputs = ConstantInputs(1);
  VM vm = VM(program, &inputs);
  long long output = 0;
  do {
    output = vm.get_next_output().value();
  } while (output == 0);

  return output;
}

long long p2(const std::vector<long long> &program) {
  auto inputs = ConstantInputs(5);
  VM vm = VM(program, &inputs);
  long long output = 0;
  do {
    output = vm.get_next_output().value();
  } while (output == 0);

  return output;
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
