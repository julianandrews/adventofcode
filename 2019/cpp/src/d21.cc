#include <iostream>
#include <optional>
#include <stdexcept>
#include <vector>

#include "intcode.h"
#include "strings.h"

long long run_commands(const std::vector<long long> &program,
                       const std::vector<std::string> &commands) {
  int i = 0;
  std::string inputs = aoc::strings::join(commands, "\n") + '\n';
  aoc::intcode::VM vm(program, [&inputs, &i]() { return inputs.at(i++); });
  std::vector<long long> outputs;
  for (auto output = vm.get_next_output(); output.has_value();
       output = vm.get_next_output()) {
    outputs.push_back(output.value());
  }
  if (outputs.back() <= 128) {
    for (auto x : outputs) {
      std::cerr << (char)x;
    }
    throw std::runtime_error("Run failed.");
  }
  return outputs.back();
}

int p1(const std::vector<long long> &program) {
  auto commands = std::vector<std::string>({
      "OR C J",
      "AND A J",
      "NOT J J",
      "AND D J",
      "WALK",
  });

  return run_commands(program, commands);
}

int p2(const std::vector<long long> &program) {
  auto commands = std::vector<std::string>({
      "NOT A J",
      "NOT J J",
      "AND B J",
      "AND C J",
      "NOT J J",
      "AND D J",
      "NOT E T",
      "NOT T T",
      "OR H T",
      "AND T J",
      "RUN",
  });

  return run_commands(program, commands);
}

int main() {
  try {
    std::string line;
    getline(std::cin, line);

    std::vector<long long> program;
    for (std::string s : aoc::strings::split(line, ',')) {
      program.push_back(std::stoll(s));
    }

    std::cout << "Part 1: " << p1(program) << std::endl;
    std::cout << "Part 2: " << p2(program) << std::endl;
  } catch (const std::exception &e) {
    std::cerr << e.what() << std::endl;
    return 1;
  }
}
