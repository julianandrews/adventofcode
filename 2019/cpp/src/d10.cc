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

bool direction_cmp(const Point<int> &a, const Point<int> &b) {
  auto key_func = [](const Point<int> &p) {
    return fmod(-atan2(p.x, p.y) + M_PI, 2 * M_PI);
  };
  return key_func(a) < key_func(b);
};

typedef std::set<Point<int>, decltype(&direction_cmp)> DirectionSet;

class AsteroidField {
  const int height_;
  const int width_;

  std::unordered_set<Point<int>> asteroids_;

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
          asteroids_.insert(Point<int>(x, y));
        }
      }
    }
  }

  bool asteroid_at(const Point<int> &p) const {
    return asteroids_.find(p) != asteroids_.end();
  }

  DirectionSet directions(const Point<int> &p) const {
    DirectionSet directions(&direction_cmp);

    for (int x = 0; x < width_; ++x) {
      int dx = x - p.x;
      for (int y = 0; y < height_; ++y) {
        int dy = y - p.y;
        if (dx || dy) {
          int denom = std::gcd(dx, dy);
          directions.insert(Point<int>(dx / denom, dy / denom));
        }
      }
    }

    return directions;
  }

  std::optional<Point<int>>
  first_visible_asteroid(const Point<int> &location,
                         const Point<int> &direction) const {
    Point<int> p = Point(location.x, location.y);
    while (p.y >= 0 && p.y < height_ && p.x >= 0 && p.x < width_) {
      p.x += direction.x;
      p.y += direction.y;
      if (asteroid_at(p)) {
        return p;
      }
    }
    return std::nullopt;
  }

  int visible_count(const Point<int> &p) const {
    int count = 0;
    for (const auto &d : directions(p)) {
      if (first_visible_asteroid(p, d).has_value()) {
        ++count;
      }
    }

    return count;
  }

  Point<int> monitoring_station() const {
    int best = -1;
    Point<int> best_point = Point<int>(0, 0);
    for (const auto &p : asteroids_) {
      int count = visible_count(p);
      if (count > best) {
        best = count;
        best_point = p;
      }
    }

    return best_point;
  }

  Point<int> destroy_n_asteroids(const Point<int> &p, int n) {
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
  Point<int> p = field.destroy_n_asteroids(field.monitoring_station(), 200);
  return 100 * p.x + p.y;
}

int main() {
  std::vector<std::string> lines = aoc::utils::getlines();

  std::cout << "Part 1: " << p1(lines) << std::endl;
  std::cout << "Part 2: " << p2(lines) << std::endl;
}
