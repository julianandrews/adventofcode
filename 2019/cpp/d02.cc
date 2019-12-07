#include <iostream>
#include <vector>

#include "intcode.h"
#include "utils.h"

using intcode::ConstantInputs;
using intcode::Inputs;
using intcode::Op;
using intcode::VM;

int p1(const std::vector<int> &program) {
  auto inputs = ConstantInputs(0);
  VM vm = VM(program, &inputs, 12, 2);
  Op op;
  do {
    op = vm.step();
  } while (op != Op::HALT);
  return vm.first_register();
}

int p2(const std::vector<int> &program) {
  auto inputs = ConstantInputs(0);
  for (int noun = 0; noun < 100; ++noun) {
    for (int verb = 0; verb < 100; ++verb) {
      VM vm = VM(program, &inputs, noun, verb);
      Op op;
      do {
        op = vm.step();
      } while (op != Op::HALT);
      if (vm.first_register() == 19690720) {
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
