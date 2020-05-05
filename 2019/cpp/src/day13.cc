#include <iostream>
#include <unordered_map>
#include <vector>

#include "intcode.h"
#include "point.h"
#include "strings.h"

typedef ::aoc::point::Point<long long, 2> Coords;

enum class TileType { EMPTY = 0, WALL = 1, BLOCK = 2, PADDLE = 3, BALL = 4 };

struct Tile {
  Coords location;
  TileType type;
};

class ArcadeMachine {
  aoc::intcode::VM vm_;
  std::unordered_map<Coords, TileType> map;
  long long ball_location_;
  long long paddle_location_;
  long long score_ = 0;

  std::optional<Tile> next_tile() {
    auto x = vm_.get_next_output();
    auto y = vm_.get_next_output();
    auto t = vm_.get_next_output();
    if (!x.has_value() || !y.has_value() || !t.has_value()) {
      return std::nullopt;
    }
    return Tile{{x.value(), y.value()}, static_cast<TileType>(t.value())};
  }

  long long ai() {
    long long offset = ball_location_ - paddle_location_;
    return offset < 0 ? -1 : (offset > 0 ? 1 : 0);
  }

public:
  ArcadeMachine(std::vector<long long> program)
      : vm_(program, [this] { return this->ai(); }) {}

  void run() {
    for (auto tile = next_tile(); tile.has_value(); tile = next_tile()) {
      auto &[location, tile_type] = tile.value();
      auto [x, y] = location.values;
      if (x == -1 && y == 0) {
        score_ = static_cast<int>(tile_type);
        continue;
      }
      if (tile_type == TileType::BALL) {
        ball_location_ = x;
      } else if (tile_type == TileType::PADDLE) {
        paddle_location_ = x;
      }
      map[Coords({x, y})] = tile_type;
    }
  }

  int tile_count(TileType tile_type) {
    int count = 0;
    for (const auto &[_, t] : map) {
      if (t == tile_type) {
        ++count;
      }
    }
    return count;
  }

  int get_score() { return score_; }
};

int p1(std::vector<long long> program) {
  ArcadeMachine machine = ArcadeMachine(program);
  machine.run();
  return machine.tile_count(TileType::BLOCK);
}

int p2(std::vector<long long> program) {
  program[0] = 2;
  ArcadeMachine machine = ArcadeMachine(program);
  machine.run();
  return machine.get_score();
}

int main() {
  try {
    std::string line;
    getline(std::cin, line);

    std::vector<long long> program;
    for (std::string s : aoc::strings::split(line, ',')) {
      program.push_back(std::stoll(s));
    }

    std::cout << "Part 1: " << p1(program) << std::endl;
    std::cout << "Part 2: " << p2(program) << std::endl;
  } catch (const std::exception &e) {
    std::cerr << e.what() << std::endl;
    return 1;
  }
}
