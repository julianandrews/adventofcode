#include <algorithm>
#include <iostream>
#include <vector>

#include "intcode.h"
#include "strings.h"

using ::aoc::intcode::Op;
using ::aoc::intcode::VM;

class TwoStepInputs {
  long long initial_;
  long long value_;
  bool returned_once = false;

public:
  TwoStepInputs(long long first, long long value)
      : initial_(first), value_(value) {}

  std::function<long long ()> next_input_function() {
    return [this] {
      if (!returned_once) {
        returned_once = true;
        return initial_;
      }
      return value_;
    };
  }

  void set_value(long long value) { value_ = value; }
};

long long p1(const std::vector<long long> &program) {
  long long best = 0;
  long long permutation[] = {0, 1, 2, 3, 4};
  do {
    long long signal = 0;
    for (long long phase : permutation) {
      auto inputs = TwoStepInputs(phase, signal).next_input_function();
      VM vm = VM(program, std::move(inputs));
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
        VM(program, std::move(inputs[0].next_input_function())),
        VM(program, std::move(inputs[1].next_input_function())),
        VM(program, std::move(inputs[2].next_input_function())),
        VM(program, std::move(inputs[3].next_input_function())),
        VM(program, std::move(inputs[4].next_input_function())),
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
  std::string line;
  getline(std::cin, line);

  std::vector<long long> program;
  for (std::string s : aoc::strings::split(line, ',')) {
    program.push_back(std::stoi(s));
  }

  std::cout << "Part 1: " << p1(program) << std::endl;
  std::cout << "Part 2: " << p2(program) << std::endl;
}
