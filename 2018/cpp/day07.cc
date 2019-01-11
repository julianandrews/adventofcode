#include <string>
#include <unordered_set>
#include <utility>
#include <vector>

#include "graphs.h"
#include "utils.h"

int MAX_WORKERS = 5;
int BASE_STEP_TIME = 60;

namespace {
using std::pair;
using std::string;
using std::vector;
}  // namespace

string p1(vector<pair<char, char>> pairs) {
  std::unordered_set<char> values;
  std::unordered_map<char, std::vector<char>> edges;
  for (const auto& p : pairs) {
    values.insert(p.first);
    values.insert(p.second);
    edges[p.first].push_back(p.second);
  }

  std::function<vector<char>(const char&)> neighbors = [&edges](const char& v) {
    return edges[v];
  };

  const auto result =
      aoc::graphs::toposort({values.begin(), values.end()}, neighbors).value();

  return string(result.begin(), result.end());
}

int p2(vector<pair<char, char>> pairs) {
  std::unordered_map<char, std::vector<char>> edges;
  std::unordered_map<char, int> indegrees;
  for (const auto& p : pairs) {
    edges[p.first].push_back(p.second);
    ++indegrees[p.second];
  }

  std::priority_queue<char, std::vector<char>, std::greater<char>>
      working_values;
  for (const auto& p : edges) {
    if (indegrees[p.first] == 0) {
      working_values.push(p.first);
    }
  }

  std::unordered_map<char, int> tasks;
  int t = 0;
  while (working_values.size() || tasks.size()) {
    for (auto it = tasks.begin(); it != tasks.end();) {
      char value = it->first;
      int end_time = it->second;
      if (t >= end_time) {
        tasks.erase(it++);
        for (auto edge : edges[value]) {
          --indegrees[edge];
          if (indegrees[edge] == 0) {
            working_values.push(edge);
          }
        }
      } else {
        ++it;
      }
    }
    while (working_values.size() && tasks.size() < MAX_WORKERS) {
      auto value = working_values.top();
      working_values.pop();
      tasks[value] = t + BASE_STEP_TIME + 1 + value - 'A';
    }
    ++t;
  }

  return t - 1;
}

int main() {
  vector<string> lines = aoc::utils::getlines();

  vector<pair<char, char>> pairs;
  std::transform(
      lines.begin(), lines.end(), std::back_inserter(pairs),
      [](const string& line) { return pair<char, char>(line[5], line[36]); });

  std::cout << "Part 1: " << p1(pairs) << std::endl;
  std::cout << "Part 2: " << p2(pairs) << std::endl;
}
