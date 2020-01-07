#ifndef AOC_INTCODE_H
#define AOC_INTCODE_H

#include <memory>
#include <optional>
#include <vector>

namespace intcode {

enum class ValueMode { POSITION = 0, IMMEDIATE = 1, RELATIVE = 2 };

enum class Op {
  ADD = 1,
  MULTIPLY = 2,
  STORE = 3,
  OUTPUT = 4,
  JUMP_IF_TRUE = 5,
  JUMP_IF_FALSE = 6,
  LESS_THAN = 7,
  EQUALS = 8,
  ADJUST_REL_OFFSET = 9,
  HALT = 99
};

class Inputs {
public:
  virtual long long next() = 0;
};

class ConstantInputs : public Inputs {
  const long long n_;

public:
  ConstantInputs(long long n) : n_(n) {}

  long long next() override { return n_; }
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
  } else if (op == Op::STORE || op == Op::OUTPUT ||
             op == Op::ADJUST_REL_OFFSET) {
    return 1;
  } else if (op == Op::HALT) {
    return 0;
  } else {
    throw "Unexpected operation!";
  }
}

class VM {
  class VMMemory {
    std::vector<long long> memory_;

  public:
    VMMemory(std::vector<long long> memory) : memory_(memory) {}

    long long at(const size_t index) const {
      return index < memory_.size() ? memory_.at(index) : 0;
    }

    long long &operator[](const size_t index) {
      if (index >= memory_.size()) {
        memory_.resize(index + 1);
      }
      return memory_[index];
    }
  };

  VMMemory memory_;
  long long ip_ = 0;
  long long relative_base_ = 0;
  long long output_ = 0;
  Inputs *inputs_;

  long long get_value(long long value, ValueMode mode) const {
    switch (mode) {
    case ValueMode::POSITION:
      return memory_.at(value);
    case ValueMode::RELATIVE:
      return memory_.at(relative_base_ + value);
    default:
      return value;
    }
  }

  long long get_address(long long base_address, ValueMode mode) const {
    switch (mode) {
    case ValueMode::POSITION:
      return base_address;
    case ValueMode::RELATIVE:
      return base_address + relative_base_;
    default:
      throw "Unexpected Mode";
    }
  }

public:
  VM(std::vector<long long> memory, Inputs *inputs)
      : memory_(std::move(memory)), inputs_(inputs) {}

  VM(std::vector<long long> memory, Inputs *inputs, long long noun,
     long long verb)
      : memory_(std::move(memory)), inputs_(inputs) {
    memory_[1] = noun;
    memory_[2] = verb;
  }

  Op step() {
    long long instruction = memory_.at(ip_);
    Op op = Op(instruction % 100);
    instruction /= 100;
    std::vector<long long> params;
    std::vector<ValueMode> modes;
    for (int i = 0; i < num_params(op); ++i) {
      params.push_back(memory_.at(ip_ + i + 1));
      modes.push_back(ValueMode(instruction % 10));
      instruction /= 10;
    }
    long long ip_offset = num_params(op) + 1;

    if (is_binary_op(op)) {
      long long a = get_value(params.at(0), modes.at(0));
      long long b = get_value(params.at(1), modes.at(1));
      long long address = get_address(params.at(2), modes.at(2));
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
        throw "Unexpected operation!";
      }
    } else if (is_jump(op)) {
      long long value = get_value(params.at(0), modes.at(0));
      long long address = get_value(params.at(1), modes.at(1));
      if ((op == Op::JUMP_IF_TRUE && value) ||
          (op == Op::JUMP_IF_FALSE && !value)) {
        ip_ = address;
        ip_offset = 0;
      }
    } else if (op == Op::STORE) {
      long long address = get_address(params.at(0), modes.at(0));
      memory_[address] = inputs_->next();
    } else if (op == Op::OUTPUT) {
      output_ = get_value(params.at(0), modes.at(0));
    } else if (op == Op::ADJUST_REL_OFFSET) {
      int value = get_value(params.at(0), modes.at(0));
      relative_base_ += value;
    } else if (op == Op::HALT) {
      ip_offset = 0;
    } else {
      throw "Unexpected operation!";
    }

    ip_ += ip_offset;
    return op;
  }

  std::optional<long long> get_next_output() {
    Op op;
    do {
      op = step();
    } while (op != Op::HALT && op != Op::OUTPUT);

    return op == Op::OUTPUT ? std::optional<long long>(output_) : std::nullopt;
  }

  long long first_register() const { return memory_.at(0); }

  long long diagnostic_code() const { return output_; }
};

} // namespace intcode

#endif
