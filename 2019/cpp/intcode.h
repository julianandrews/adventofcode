#ifndef AOC_INTCODE_H
#define AOC_INTCODE_H

#include <memory>
#include <optional>
#include <vector>

namespace intcode {

enum class ValueMode { POSITION = 0, IMMEDIATE = 1 };

enum class Op {
  ADD = 1,
  MULTIPLY = 2,
  STORE = 3,
  OUTPUT = 4,
  JUMP_IF_TRUE = 5,
  JUMP_IF_FALSE = 6,
  LESS_THAN = 7,
  EQUALS = 8,
  HALT = 99
};

class Inputs {
public:
  virtual int next() = 0;
};

class ConstantInputs : public Inputs {
  const int n_;

public:
  ConstantInputs(int n) : n_(n) {}

  int next() override { return n_; }
};

bool is_binary_op(Op op) {
  return op == Op::ADD || op == Op::MULTIPLY || op == Op::LESS_THAN ||
         op == Op::EQUALS;
}

bool is_jump(Op op) {
  return op == Op::JUMP_IF_TRUE || op == Op::JUMP_IF_FALSE;
}

int num_params(Op op) {
  if (is_binary_op(op)) {
    return 3;
  } else if (is_jump(op)) {
    return 2;
  } else if (op == Op::STORE || op == Op::OUTPUT) {
    return 1;
  } else if (op == Op::HALT) {
    return 0;
  } else {
    throw "Unexpected operation!";
  }
}

class VM {
  std::vector<int> memory_;
  int ip_ = 0;
  int output_ = 0;
  Inputs *inputs_;

  int get_value(int value, ValueMode mode) {
    switch (mode) {
    case ValueMode::POSITION:
      return memory_.at(value);
    default:
      return value;
    }
  }

public:
  VM(std::vector<int> memory, Inputs *inputs)
      : memory_(memory), inputs_(inputs) {}

  VM(std::vector<int> memory, Inputs *inputs, int noun, int verb)
      : memory_(memory), inputs_(inputs) {
    memory_[1] = noun;
    memory_[2] = verb;
  }

  Op step() {
    int instruction = memory_.at(ip_);
    Op op = Op(instruction % 100);
    instruction /= 100;
    std::vector<int> params;
    std::vector<ValueMode> modes;
    for (int i = 0; i < num_params(op); ++i) {
      params.push_back(memory_.at(ip_ + i + 1));
      modes.push_back(ValueMode(instruction % 10));
      instruction /= 10;
    }
    int ip_offset = num_params(op) + 1;

    if (is_binary_op(op)) {
      int a = get_value(params.at(0), modes.at(0));
      int b = get_value(params.at(1), modes.at(1));
      int address = params.at(2);
      switch (op) {
        case Op::ADD:
          memory_[address] = a + b;
          break;
        case Op::MULTIPLY:
          memory_[address] = a * b;
          break;
        case Op::LESS_THAN:
          memory_[address] = a < b ? 1 : 0;
          break;
        case Op::EQUALS:
          memory_[address] = a == b ? 1 : 0;
          break;
        default:
          throw "Unexpecetd operation!";
      }
    } else if (is_jump(op)) {
      int value = get_value(params.at(0), modes.at(0));
      int address = get_value(params.at(1), modes.at(1));
      if ((op == Op::JUMP_IF_TRUE && value) ||
          (op == Op::JUMP_IF_FALSE && !value)) {
        ip_ = address;
        ip_offset = 0;
      }
    } else if (op == Op::STORE) {
      int address = params.at(0);
      memory_[address] = inputs_->next();
    } else if (op == Op::OUTPUT) {
      output_ = get_value(params.at(0), modes.at(0));
    } else if (op == Op::HALT) {
      ip_offset = 0;
    } else {
      throw "Unexpected operation!";
    }

    ip_ += ip_offset;
    return op;
  }

  std::optional<int> get_next_output() {
    Op op;
    do {
      op = step();
    } while (op != Op::HALT && op != Op::OUTPUT);

    return op == Op::OUTPUT ? std::optional<int>(output_) : std::nullopt;
  }

  int first_register() const { return memory_.at(0); }

  int diagnostic_code() const { return output_; }
};

} // namespace intcode

#endif
