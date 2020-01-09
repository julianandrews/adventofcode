#include <iostream>
#include <math.h>
#include <numeric>
#include <optional>
#include <set>
#include <unordered_set>
#include <vector>

#include "point.h"
#include "utils.h"

using ::aoc::point::Point;

bool direction_cmp(const Point<int, 2> &a, const Point<int, 2> &b) {
  auto key_func = [](const Point<int, 2> &p) {
    return fmod(-atan2(p.values[0], p.values[1]) + M_PI, 2 * M_PI);
  };
  return key_func(a) < key_func(b);
};

typedef std::set<Point<int, 2>, decltype(&direction_cmp)> DirectionSet;

class AsteroidField {
  const int height_;
  const int width_;

  std::unordered_set<Point<int, 2>> asteroids_;

public:
  explicit AsteroidField(const std::vector<std::string> &lines)
      : height_(lines.size()), width_(lines.size() ? lines.at(0).length() : 0) {
    for (const std::string &line : lines) {
      if ((int)line.length() != width_) {
        throw "Inconsistent grid size!";
      }
    }

    for (int y = 0; y < height_; ++y) {
      for (int x = 0; x < width_; ++x) {
        if (lines.at(y).at(x) == '#') {
          asteroids_.insert({x, y});
        }
      }
    }
  }

  bool asteroid_at(const Point<int, 2> &p) const {
    return asteroids_.find(p) != asteroids_.end();
  }

  DirectionSet directions(const Point<int, 2> &p) const {
    DirectionSet directions(&direction_cmp);

    for (int x = 0; x < width_; ++x) {
      int dx = x - p.values[0];
      for (int y = 0; y < height_; ++y) {
        int dy = y - p.values[1];
        if (dx || dy) {
          int denom = std::gcd(dx, dy);
          directions.insert({dx / denom, dy / denom});
        }
      }
    }

    return directions;
  }

  std::optional<Point<int, 2>>
  first_visible_asteroid(const Point<int, 2> &location,
                         const Point<int, 2> &direction) const {
    Point<int, 2> p = {location.values[0], location.values[1]};
    while (p.values[1] >= 0 && p.values[1] < height_ && p.values[0] >= 0 &&
           p.values[0] < width_) {
      p.values[0] += direction.values[0];
      p.values[1] += direction.values[1];
      if (asteroid_at(p)) {
        return p;
      }
    }
    return std::nullopt;
  }

  int visible_count(const Point<int, 2> &p) const {
    int count = 0;
    for (const auto &d : directions(p)) {
      if (first_visible_asteroid(p, d).has_value()) {
        ++count;
      }
    }

    return count;
  }

  Point<int, 2> monitoring_station() const {
    int best = -1;
    Point<int, 2> best_point = {0, 0};
    for (const auto &p : asteroids_) {
      int count = visible_count(p);
      if (count > best) {
        best = count;
        best_point = p;
      }
    }

    return best_point;
  }

  Point<int, 2> destroy_n_asteroids(const Point<int, 2> &p, int n) {
    int count = 0;
    auto dirs = directions(p);
    auto it = dirs.begin();
    for (auto it = dirs.begin(); true; ++it) {
      if (it == dirs.end())
        it = dirs.begin();
      auto asteroid_coords = first_visible_asteroid(p, *it);
      if (asteroid_coords.has_value()) {
        asteroids_.erase(asteroid_coords.value());
        ++count;
        if (count == n) {
          return asteroid_coords.value();
        }
      }
    }
  }
};

int p1(const std::vector<std::string> lines) {
  auto field = AsteroidField(lines);
  return field.visible_count(field.monitoring_station());
}

int p2(const std::vector<std::string> lines) {
  auto field = AsteroidField(lines);
  Point<int, 2> p = field.destroy_n_asteroids(field.monitoring_station(), 200);
  return 100 * p.values[0] + p.values[1];
}

int main() {
  std::vector<std::string> lines = aoc::utils::getlines();

  std::cout << "Part 1: " << p1(lines) << std::endl;
  std::cout << "Part 2: " << p2(lines) << std::endl;
}
