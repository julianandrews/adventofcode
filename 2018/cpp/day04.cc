#include <algorithm>
#include <iostream>
#include <map>
#include <string>
#include <unordered_map>
#include <utility>
#include <vector>

#include "utils.h"

std::multimap<int, std::pair<int, int>>
get_sleep_times(const std::vector<std::string> &lines) {
  std::multimap<int, std::pair<int, int>> sleep_times;
  int current_guard;
  int sleep_start;

  std::cout << std::endl;
  for (const auto &line : lines) {
    auto minute = std::stoi(line.substr(15, 2));
    if (line.find("begins shift") != std::string::npos) {
      current_guard = std::stoi(line.substr(26, line.size() - 39));
    } else if (line.find("falls asleep") != std::string::npos) {
      sleep_start = minute;
    } else if (line.find("wakes up") != std::string::npos) {
      sleep_times.insert({current_guard, {sleep_start, minute}});
    }
  }

  return sleep_times;
}

int p1(const std::vector<std::string> &lines) {
  const auto sleep_times = get_sleep_times(lines);

  std::unordered_map<int, int> total_sleep_times;
  for (const auto &entry : sleep_times) {
    const auto guard_id = entry.first;
    const auto start_time = entry.second.first;
    const auto end_time = entry.second.second;
    total_sleep_times[guard_id] += end_time - start_time;
  }

  auto sleepiest_guard =
      std::max_element(total_sleep_times.begin(), total_sleep_times.end(),
                       [](auto &a, auto &b) { return a.second < b.second; })
          ->first;

  auto sleepiest_guard_times = sleep_times.equal_range(sleepiest_guard);
  std::unordered_map<int, int> sleep_counts;
  for (auto it = sleepiest_guard_times.first;
       it != sleepiest_guard_times.second; ++it) {
    auto start_time = it->second.first;
    auto end_time = it->second.second;
    for (auto i = start_time; i < end_time; i++) {
      sleep_counts[i]++;
    }
  }
  auto sleepiest_minute =
      std::max_element(sleep_counts.begin(), sleep_counts.end(), [](auto &a,
                                                                    auto &b) {
        return a.second < b.second;
      })->first;

  return sleepiest_guard * sleepiest_minute;
}

int p2(const std::vector<std::string> &lines) {
  const auto sleep_times = get_sleep_times(lines);

  std::map<std::pair<int, int>, int> sleep_counts;
  for (const auto &entry : sleep_times) {
    const auto guard_id = entry.first;
    const auto start_time = entry.second.first;
    const auto end_time = entry.second.second;
    for (auto minute = start_time; minute < end_time; minute++) {
      sleep_counts[{guard_id, minute}]++;
    }
  }

  auto sleepiest_guard_and_minute =
      std::max_element(sleep_counts.begin(), sleep_counts.end(), [](auto &a,
                                                                    auto &b) {
        return a.second < b.second;
      })->first;

  return sleepiest_guard_and_minute.first * sleepiest_guard_and_minute.second;
  return 0;
}

int main() {
  std::vector<std::string> lines = aoc::utils::getlines();
  std::sort(lines.begin(), lines.end());

  std::cout << "Part 1: " << p1(lines) << std::endl;
  std::cout << "Part 2: " << p2(lines) << std::endl;
}
