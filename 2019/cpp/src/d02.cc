#include <iostream>
#include <vector>

#include "intcode.h"
#include "utils.h"

using aoc::intcode::ConstantInputs;
using aoc::intcode::Inputs;
using aoc::intcode::Op;
using aoc::intcode::VM;

long long p1(const std::vector<long long> &program) {
  auto inputs = ConstantInputs(0);
  VM vm = VM(program, &inputs, 12, 2);
  Op op;
  do {
    op = vm.step();
  } while (op != Op::HALT);
  return vm.first_register();
}

long long p2(const std::vector<long long> &program) {
  auto inputs = ConstantInputs(0);
  for (long long noun = 0; noun < 100; ++noun) {
    for (long long verb = 0; verb < 100; ++verb) {
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

  std::vector<long long> program;
  for (std::string s : aoc::utils::split(line, ',')) {
    program.push_back(std::stoi(s));
  }

  std::cout << "Part 1: " << p1(program) << std::endl;
  std::cout << "Part 2: " << p2(program) << std::endl;
}
