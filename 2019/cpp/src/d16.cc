#include <iostream>
#include <memory>
#include <stdexcept>
#include <string>
#include <vector>

#include "utils.h"

int first_n_digits(const std::vector<int> &digits, int n) {
  int result = 0;
  for (int i = 0; i < n; ++i) {
    result *= 10;
    result += digits[i];
  }
  return result;
}

std::vector<int> fft(const std::vector<int> &input_list, int num_phases) {
  static int BASE_PATTERN[4] = {0, 1, 0, -1};

  if (input_list.size() < 8) {
    throw std::invalid_argument("Not enough inputs");
  }
  auto source = std::make_unique<std::vector<int>>(input_list.size());
  auto destination = std::make_unique<std::vector<int>>();
  std::copy(input_list.begin(), input_list.end(),
            std::back_inserter(*destination));

  for (int phase = 0; phase < num_phases; ++phase) {
    std::swap(source, destination);
    for (int pos = 0; pos < destination->size(); ++pos) {
      int value = 0;
      for (int i = 0; i < source->size(); ++i) {
        int pattern = BASE_PATTERN[((i + 1) / (pos + 1)) % 4];
        value += source->at(i) * pattern;
      }
      (*destination)[pos] = std::abs(value) % 10;
    }
  }

  return *destination;
}

std::string p1(const std::vector<int> &input_list) {
  return aoc::utils::zero_pad(first_n_digits(fft(input_list, 100), 8), 8);
}

std::string p2(const std::vector<int> &input_list) {
  int new_size = input_list.size() * 10000;
  int message_offset = first_n_digits(input_list, 7);
  if (message_offset <= new_size / 2) {
    throw std::invalid_argument("Message offset too close to start of list");
  }

  int n = new_size - message_offset;

  auto last_n = std::vector<int>(n);
  for (int i = 0; i < n; ++i) {
    last_n[i] = input_list[(i + message_offset) % input_list.size()];
  }
  for (int i = 0; i < 100; ++i) {
    for (int j = n - 2; j >= 0; --j) {
      last_n[j] = (last_n[j + 1] + last_n[j]) % 10;
    }
  }
  return aoc::utils::zero_pad(first_n_digits(last_n, 8), 8);
}

int main() {
  std::string line;
  getline(std::cin, line);
  std::vector<int> input_list;
  for (char c : line) {
    input_list.push_back(c - '0');
  }

  try {
    std::cout << "Part 1: " << p1(input_list) << std::endl;
    std::cout << "Part 2: " << p2(input_list) << std::endl;
  } catch (const std::exception &e) {
    std::cerr << e.what() << std::endl;
    return 1;
  }
  return 0;
}
