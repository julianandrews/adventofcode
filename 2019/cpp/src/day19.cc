#include <iostream>
#include <stdexcept>
#include <vector>

#include "intcode.h"
#include "strings.h"

class TractorBeam {
public:
  explicit TractorBeam(const std::vector<long long> &program)
      : program_(program) {}

  bool is_active(long long x, long long y) {
    if (x < 0 || y < 0) {
      return false;
    }

    int i = 0;
    aoc::intcode::VM vm(program_,
                        [&i, x, y]() { return i++ % 2 == 0 ? x : y; });
    return vm.get_next_output().value();
  }

private:
  const std::vector<long long> program_;
};

int p1(const std::vector<long long> &program) {
  TractorBeam tractor_beam(program);
  int active_squares = 0;
  for (int x = 0; x < 50; ++x) {
    for (int y = 0; y < 50; ++y) {
      if (tractor_beam.is_active(x, y)) {
        ++active_squares;
      }
    }
  }
  return active_squares;
}
int p2(const std::vector<long long> &program) {
  TractorBeam tractor_beam(program);
  int x = 0, y = 0;
  while (true) {
    if (tractor_beam.is_active(x + 99, y - 99)) {
      return 10000 * x + y - 99;
    }
    if (tractor_beam.is_active(x, y + 1)) {
      ++y;
    } else if (tractor_beam.is_active(x + 1, y)) {
      ++x;
    } else {
      x = 0;
      ++y;
      while (!tractor_beam.is_active(x, y)) {
        ++x;
        if (x > y) {
          x = 0;
          ++y;
        }
      }
    }
  }
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
