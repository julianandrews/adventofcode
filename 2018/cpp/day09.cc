#include <deque>
#include <string>
#include <unordered_map>
#include <vector>

#include "utils.h"

namespace {

using std::string;
using std::vector;

}  // namespace

long p1(int num_players, int max_marble) {
  std::vector<long> scores(num_players);
  std::deque<int> circle = {0};

  for (int marble = 1; marble <= max_marble; ++marble) {
    if (marble % 23) {
      if (circle.size() > 1) {
        circle.push_back(circle.front());
        circle.pop_front();
      }
      circle.push_back(marble);
    } else {
      for (int i = 0; i < 7; ++i) {
        circle.push_front(circle.back());
        circle.pop_back();
      }
      scores[marble % num_players] += marble + circle.back();
      circle.pop_back();
      circle.push_back(circle.front());
      circle.pop_front();
    }
  }

  return *std::max_element(scores.begin(), scores.end());
}

int main() {
  vector<string> lines = aoc::utils::getlines();
  vector<string> words = aoc::utils::split(lines[0], ' ');
  int num_players = std::stoi(words[0]);
  int max_marble = std::stoi(words[6]);

  std::cout << "Part 1: " << p1(num_players, max_marble) << std::endl;
  std::cout << "Part 2: " << p1(num_players, max_marble * 100) << std::endl;
}
