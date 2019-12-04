#ifndef AOC_INTCODE_H
#define AOC_INTCODE_H

#include <vector>

namespace intcode {

enum class Op { ADD = 1, MULTIPLY = 2, HALT = 99 };

int num_params(Op op) {
  switch (op) {
    case Op::HALT :
      return 0;
    default:
      return 3;
  }
}

class VM {
  std::vector<int> memory_;
  int ip_ = 0;

 public:
  VM(std::vector<int> memory) : memory_(memory){};

  VM(std::vector<int> memory, int noun, int verb) : memory_(memory) {
    memory_[1] = noun;
    memory_[2] = verb;
  }

  Op step() {
    Op op = Op(memory_.at(ip_));
    std::vector<int> params;
    for (int i = 0; i < num_params(op); ++i) {
      params.push_back(memory_.at(ip_ + i + 1));
    }
    switch (op) {
      case Op::ADD:
        memory_[params.at(2)] =
            memory_.at(params.at(0)) + memory_.at(params.at(1));
        break;
      case Op::MULTIPLY:
        memory_[params.at(2)] =
            memory_.at(params.at(0)) * memory_.at(params.at(1));
        break;
      default:
        break;
    }

    ip_ += num_params(op) + 1;
    return op;
  }

  void run() {
    Op op;
    do {
      op = step();
    } while (op != Op::HALT);
  }

  int output() { return memory_.at(0); }
};

} // namespace intcode

#endif
