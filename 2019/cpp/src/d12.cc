#include <numeric>
#include <vector>

#include "point.h"
#include "strings.h"

typedef ::aoc::point::Point<int, 3> Coords;

Coords parse_position(std::string s) {
  auto parts = aoc::strings::split(s.substr(1, s.length() - 2), ',');
  int x = std::stoi(aoc::strings::trim(parts[0]).substr(2));
  int y = std::stoi(aoc::strings::trim(parts[1]).substr(2));
  int z = std::stoi(aoc::strings::trim(parts[2]).substr(2));

  return {x, y, z};
}

class Moon {
  Coords position_;
  Coords velocity_ = Coords({0, 0, 0});

  void update_velocity(const std::vector<Moon> &moons) {
    for (const auto &moon : moons) {
      for (int i = 0; i < 3; ++i) {
        int offset = position_[i] - moon.position_[i];
        if (offset < 0) {
          velocity_[i] += 1;
        } else if (offset > 0) {
          velocity_[i] -= 1;
        }
      }
    }
  }

  void update_position() {
    for (int i = 0; i < 3; ++i) {
      position_[i] += velocity_[i];
    }
  }

  friend class PlanetarySystem;

public:
  Moon(Coords position) : position_(position) {}

  int energy() const {
    int p = 0;
    int v = 0;
    for (int i = 0; i < 3; ++i) {
      p += std::abs(position_[i]);
      v += std::abs(velocity_[i]);
    }
    return p * v;
  }
};

class PlanetarySystem {
public:
  std::vector<Moon> moons_;

  PlanetarySystem(std::vector<Moon> moons) : moons_(moons) {}

  void step() {
    for (auto &moon : moons_) {
      moon.update_velocity(moons_);
    }
    for (auto &moon : moons_) {
      moon.update_position();
    }
  }

  int energy() const {
    return std::accumulate(
        moons_.begin(), moons_.end(), 0,
        [](int total, const Moon &moon) { return total + moon.energy(); });
  }

  std::array<long, 3> dimension_periods() {
    std::vector<Coords> initial_positions;
    for (const auto &moon : moons_) {
      initial_positions.push_back(moon.position_);
    }
    std::array<long, 3> periods = {0, 0, 0};
    int steps = 0;
    while (periods[0] == 0 || periods[1] == 0 || periods[2] == 0) {
      step();
      ++steps;
      for (int i = 0; i < 3; ++i) {
        if (periods.at(i) == 0) {
          auto is_cycle = [&](int m) {
            const auto &moon = moons_.at(m);
            return moon.position_[i] == initial_positions[m][i] &&
                   moon.velocity_[i] == 0;
          };
          std::vector<int> moon_indices;
          for (int m = 0; m < moons_.size(); ++m)
            moon_indices.push_back(m);
          if (std::all_of(moon_indices.begin(), moon_indices.end(), is_cycle)) {
            periods[i] = steps;
          }
        }
      }
    }
    return periods;
  }
};

int p1(PlanetarySystem system) {
  for (int i = 0; i < 1000; ++i) {
    system.step();
  }
  return system.energy();
}

long p2(PlanetarySystem system) {
  auto periods = system.dimension_periods();
  return std::accumulate(periods.begin(), periods.end(), 1ll,
                         [](auto a, auto b) { return std::lcm(a, b); });
}

int main() {
  std::vector<std::string> lines = aoc::strings::getlines();
  std::vector<Moon> moons;
  for (const auto &line : lines) {
    moons.push_back(parse_position(line));
  }
  PlanetarySystem system(std::move(moons));
  std::cout << "Part 1: " << p1(system) << std::endl;
  std::cout << "Part 2: " << p2(system) << std::endl;
}
