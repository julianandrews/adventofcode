#include <algorithm>
#include <iostream>
#include <vector>

#include "intcode.h"
#include "utils.h"

using intcode::Op;
using intcode::VM;

class TwoStepInputs : public intcode::Inputs {
  int initial_;
  int value_;
  bool returned_once = false;

public:
  TwoStepInputs(int first, int value) : initial_(first), value_(value) {}

  int next() {
    if (!returned_once) {
      returned_once = true;
      return initial_;
    }
    return value_;
  }

  void set_value(int value) { value_ = value; }
};

int p1(const std::vector<int> &program) {
  int best = 0;
  int permutation[] = {0, 1, 2, 3, 4};
  do {
    int signal = 0;
    for (int phase : permutation) {
      auto inputs = TwoStepInputs(phase, signal);
      VM vm = VM(program, &inputs);
      signal = vm.get_next_output().value();
    }
    best = std::max(best, signal);
  } while (std::next_permutation(permutation, permutation + 5));

  return best;
}

int p2(const std::vector<int> &program) {
  int best = 0;
  int permutation[] = {5, 6, 7, 8, 9};
  do {
    TwoStepInputs inputs[5] = {
        TwoStepInputs(permutation[0], 0), TwoStepInputs(permutation[1], 0),
        TwoStepInputs(permutation[2], 0), TwoStepInputs(permutation[3], 0),
        TwoStepInputs(permutation[4], 0)};
    VM vms[5] = {
        VM(program, &inputs[0]), VM(program, &inputs[1]),
        VM(program, &inputs[2]), VM(program, &inputs[3]),
        VM(program, &inputs[4]),
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

  std::vector<int> program;
  for (std::string s : aoc::utils::split(line, ',')) {
    program.push_back(std::stoi(s));
  }

  std::cout << "Part 1: " << p1(program) << std::endl;
  std::cout << "Part 2: " << p2(program) << std::endl;
}
