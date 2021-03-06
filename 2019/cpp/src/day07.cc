#include <algorithm>
#include <iostream>
#include <stdexcept>
#include <vector>

#include "intcode.h"
#include "strings.h"

using ::aoc::intcode::Op;
using ::aoc::intcode::VM;

class TwoStepInputs {
  long long initial_;
  long long value_;
  bool returned_once_ = false;

public:
  TwoStepInputs(long long first, long long value)
      : initial_(first), value_(value) {}

  long long operator()() {
    if (!returned_once_) {
      returned_once_ = true;
      return initial_;
    }
    return value_;
  }

  void set_value(long long value) { value_ = value; }
};

long long p1(const std::vector<long long> &program) {
  long long best = 0;
  long long permutation[] = {0, 1, 2, 3, 4};
  do {
    long long signal = 0;
    for (long long phase : permutation) {
      VM vm = VM(program, TwoStepInputs(phase, signal));
      signal = vm.get_next_output().value();
    }
    best = std::max(best, signal);
  } while (std::next_permutation(permutation, permutation + 5));

  return best;
}

long long p2(const std::vector<long long> &program) {
  long long best = 0;
  long long permutation[] = {5, 6, 7, 8, 9};
  do {
    TwoStepInputs inputs[5] = {
        TwoStepInputs(permutation[0], 0), TwoStepInputs(permutation[1], 0),
        TwoStepInputs(permutation[2], 0), TwoStepInputs(permutation[3], 0),
        TwoStepInputs(permutation[4], 0)};
    VM vms[5] = {
        VM(program, [&inputs] { return inputs[0](); }),
        VM(program, [&inputs] { return inputs[1](); }),
        VM(program, [&inputs] { return inputs[2](); }),
        VM(program, [&inputs] { return inputs[3](); }),
        VM(program, [&inputs] { return inputs[4](); }),
    };

    int i = 0;
    while (true) {
      auto opt = vms[i].get_next_output();
      if (!opt.has_value()) {
        break;
      }
      i = (i + 1) % 5;
      inputs[i].set_value(opt.value());
    }
    best = std::max(best, vms[4].diagnostic_code());
  } while (std::next_permutation(permutation, permutation + 5));

  return best;
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
