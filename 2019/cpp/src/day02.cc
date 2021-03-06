#include <iostream>
#include <stdexcept>
#include <vector>

#include "intcode.h"
#include "strings.h"

using aoc::intcode::Op;
using aoc::intcode::VM;

long long always_zero() { return 0; }

long long p1(const std::vector<long long> &program) {
  VM vm = VM(program, &always_zero, 12, 2);
  Op op;
  do {
    op = vm.step();
  } while (op != Op::HALT);
  return vm.first_register();
}

long long p2(const std::vector<long long> &program) {
  for (long long noun = 0; noun < 100; ++noun) {
    for (long long verb = 0; verb < 100; ++verb) {
      VM vm = VM(program, &always_zero, noun, verb);
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
  try {
    std::string line;
    getline(std::cin, line);

    std::vector<long long> program;
    for (std::string s : aoc::strings::split(line, ',')) {
      program.push_back(std::stoi(s));
    }

    std::cout << "Part 1: " << p1(program) << std::endl;
    std::cout << "Part 2: " << p2(program) << std::endl;
  } catch (const std::exception &e) {
    std::cerr << e.what() << std::endl;
    return 1;
  }
}
