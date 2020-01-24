#include <iostream>
#include <vector>

#include "intcode.h"
#include "strings.h"

using aoc::intcode::Op;
using aoc::intcode::VM;

long long always_one() { return 1; };
long long always_five() { return 5; };

long long p1(const std::vector<long long> &program) {
  VM vm = VM(program, &always_one);
  long long output = 0;
  do {
    output = vm.get_next_output().value();
  } while (output == 0);

  return output;
}

long long p2(const std::vector<long long> &program) {
  VM vm = VM(program, &always_five);
  long long output = 0;
  do {
    output = vm.get_next_output().value();
  } while (output == 0);

  return output;
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
