#include <cassert>

#include "strings.h"

bool is_candidate(int n) {
  std::vector<int> digits;
  while (n > 0) {
    digits.push_back(n % 10);
    n /= 10;
  }
  std::reverse(digits.begin(), digits.end());
  bool adjacent_pair = false;
  int run_length = 1;

  for (unsigned int i = 0; i < digits.size() - 1; ++i) {
    int d1 = digits[i];
    int d2 = digits[i + 1];
    if (d2 < d1) {
      return false;
    } else if (d2 == d1) {
      ++run_length;
    } else {
      if (run_length == 2) {
        adjacent_pair = true;
      }
      run_length = 1;
    }
  }
  adjacent_pair = adjacent_pair || run_length == 2;

  return adjacent_pair;
}

bool is_simple_candidate(int n) {
  std::vector<int> digits;
  while (n > 0) {
    digits.push_back(n % 10);
    n /= 10;
  }
  std::reverse(digits.begin(), digits.end());
  bool adjacent_pair = false;

  for (unsigned int i = 0; i < digits.size() - 1; ++i) {
    int d1 = digits[i];
    int d2 = digits[i + 1];
    if (d2 < d1) {
      return false;
    } else if (d2 == d1) {
      adjacent_pair = true;
    }
  }

  return adjacent_pair;
}

int p1(int start, int end) {
  int count = 0;
  for (int i = start; i <= end; ++i) {
    if (is_simple_candidate(i)) {
      ++count;
    }
  }
  return count;
}

int p2(int start, int end) {
  int count = 0;
  for (int i = start; i <= end; ++i) {
    if (is_candidate(i)) {
      ++count;
    }
  }
  return count;
}

void run_tests() {
  assert(is_simple_candidate(111111));
  assert(!is_simple_candidate(23450));
  assert(!is_simple_candidate(123789));
  assert(is_candidate(112233));
  assert(!is_candidate(123444));
  assert(is_candidate(111122));
  std::cout << "All tests passed!" << std::endl;
}

int main() {
  run_tests();
  std::string line;
  getline(std::cin, line);

  auto bits = aoc::strings::split(line, '-');
  int start = std::stoi(bits.at(0));
  int end = std::stoi(bits.at(1));

  std::cout << "Part 1: " << p1(start, end) << std::endl;
  std::cout << "Part 2: " << p2(start, end) << std::endl;

  return 0;
}
