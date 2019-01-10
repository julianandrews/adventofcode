#include <algorithm>
#include <experimental/optional>
#include <iostream>
#include <string>
#include <unordered_map>
#include <vector>

#include "graphs.h"
#include "point.h"
#include "utils.h"

namespace {
using aoc::graphs::BFSIterator;
using aoc::point::Point;
using std::vector;
using std::experimental::optional;
}  // namespace

ulong total_manhattan_distance(const Point &point,
                               const vector<Point> &points) {
  ulong total = 0;
  for (auto &other : points) {
    total += point.manhattan_distance(other);
  }

  return total;
}

vector<Point> manhattan_neighbors(const Point &point) {
  return {{point.x - 1, point.y},
          {point.x + 1, point.y},
          {point.x, point.y - 1},
          {point.x, point.y + 1}};
}

int p1(const vector<Point> &points) {
  long min_x =
      std::min_element(points.begin(), points.end(), [](auto &a, auto &b) {
        return a.x < b.x;
      })->x;
  long max_x =
      std::max_element(points.begin(), points.end(), [](auto &a, auto &b) {
        return a.x < b.x;
      })->x;
  long min_y =
      std::min_element(points.begin(), points.end(), [](auto &a, auto &b) {
        return a.y < b.y;
      })->y;
  long max_y =
      std::max_element(points.begin(), points.end(), [](auto &a, auto &b) {
        return a.y < b.y;
      })->y;

  std::unordered_map<Point, int> assigned_points;
  vector<std::unordered_set<Point>> working_points;
  for (auto &point : points) {
    working_points.push_back({point});
  }

  while (std::any_of(working_points.begin(), working_points.end(),
                     [](const auto &ps) { return ps.size(); })) {
    vector<std::unordered_set<Point>> new_points(working_points.size());
    for (int i = 0; i < working_points.size(); i++) {
      for (auto &point : working_points[i]) {
        const auto neighbors = manhattan_neighbors(point);
        new_points[i].insert(neighbors.begin(), neighbors.end());
      }
      working_points[i] = {};
    }

    std::unordered_map<Point, int> counts;
    for (int i = 0; i < working_points.size(); i++) {
      for (auto &p : new_points[i]) {
        ++counts[p];
      }
    }
    for (int i = 0; i < new_points.size(); i++) {
      for (auto &p : new_points[i]) {
        if (min_x <= p.x && p.x <= max_x && min_y <= p.y && p.y <= max_y &&
            assigned_points.count(p) == 0) {
          if (counts[p] == 1) {
            assigned_points[p] = i;
            working_points[i].insert(p);
          } else {
            assigned_points[p] = -1;
          }
        }
      }
    }
  }

  std::unordered_set<int> edge_values;
  for (long x = min_x; x <= max_x; x++) {
    edge_values.insert(assigned_points.at({x, min_y}));
    edge_values.insert(assigned_points.at({x, max_y}));
  }
  for (long y = min_y; y <= max_y; y++) {
    edge_values.insert(assigned_points.at({min_x, y}));
    edge_values.insert(assigned_points.at({max_x, y}));
  }

  std::vector<int> areas(points.size());
  for (auto &pair : assigned_points) {
    auto index = pair.second;
    if (index != -1 && !edge_values.count(index)) {
      areas[index]++;
    }
  }

  return *std::max_element(areas.begin(), areas.end());
}

int p2(const vector<Point> &points) {
  long x = 0, y = 0;
  for (auto &point : points) {
    x += point.x;
    y += point.y;
  }
  auto center = Point(x / points.size(), y / points.size());

  BFSIterator<Point> iterator(center, [&points](const Point &point) {
    auto all_points = manhattan_neighbors(point);

    vector<Point> result;
    std::copy_if(all_points.begin(), all_points.end(),
                 std::back_inserter(result), [&points](auto p) {
                   return total_manhattan_distance(p, points) < 10000;
                 });

    return result;
  });

  int count = 0;
  while (iterator.next()) {
    count++;
  }

  return count;
}

int main() {
  vector<std::string> lines = aoc::utils::getlines();
  vector<Point> points;
  std::transform(lines.begin(), lines.end(), std::back_inserter(points),
                 [](auto &s) {
                   auto values = aoc::utils::split(s, ',');
                   return Point(std::stol(values[0]), std::stol(values[1]));
                 });

  std::cout << "Part 1: " << p1(points) << std::endl;
  std::cout << "Part 2: " << p2(points) << std::endl;
}
