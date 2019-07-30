#include <string>
#include <unordered_set>
#include <vector>

#include "utils.h"

bool is_nice(const std::string &word) {
  static const std::unordered_set<char> vowels = {'a', 'e', 'i', 'o', 'u'};
  int vowel_count = 0;
  bool has_double_letter = false;
  bool has_bad_pair = false;

  for (std::size_t i = 0; i < word.size(); ++i) {
    char c = word[i];
    if (vowels.count(c)) {
      vowel_count++;
    }
    if (i + 1 < word.size()) {
      char d = word[i + 1];
      if (c == d) {
        has_double_letter = true;
      }
      if ((c == 'a' && d == 'b') || (c == 'c' && d == 'd') ||
          (c == 'p' && d == 'q') || (c == 'x' && d == 'y')) {
        has_bad_pair = true;
      }
    }
  }

  return has_double_letter && !has_bad_pair && vowel_count >= 3;
}

bool has_double_pair(const std::string &word) {
  for (std::size_t i = 0; i < word.size() - 2; ++i) {
    for (std::size_t j = i + 2; j < word.size() - 1; ++j) {
      if (word[j] == word[i] && word[j + 1] == word[i + 1]) {
        return true;
      }
    }
  }
  return false;
}

bool has_skip_repeat(const std::string &word) {
  for (std::size_t i = 0; i < word.size() - 2; ++i) {
    if (word[i] == word[i + 2]) {
      return true;
    }
  }
  return false;
}

bool is_really_nice(const std::string &word) {
  return has_double_pair(word) && has_skip_repeat(word);
}

int p1(const std::vector<std::string> &words) {
  return std::count_if(words.begin(), words.end(), is_nice);
}

int p2(const std::vector<std::string> &words) {
  return std::count_if(words.begin(), words.end(), is_really_nice);
}

int main() {
  std::vector<std::string> lines = aoc::utils::get_trimmed_lines();

  try {
    std::cout << "Part 1: " << p1(lines) << std::endl;
    std::cout << "Part 2: " << p2(lines) << std::endl;
  } catch (const std::exception &exception) {
    std::cerr << exception.what() << std::endl;
    return 1;
  }
}
