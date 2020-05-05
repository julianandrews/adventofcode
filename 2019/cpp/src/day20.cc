#include <cassert>
#include <cctype>
#include <iostream>
#include <optional>
#include <stdexcept>
#include <string>
#include <unordered_map>
#include <vector>

#include "graphs.h"
#include "point.h"
#include "strings.h"

typedef ::aoc::point::Point<int, 2> Coords;
typedef ::aoc::point::Point<int, 3> RecursiveCoords;

enum class PortalType { UP, DOWN };

class DonutMaze;

class SimpleMazeGraph : public aoc::graphs::Graph<Coords, std::vector<Coords>> {
  const DonutMaze *maze_;

public:
  explicit SimpleMazeGraph(const DonutMaze *maze) : maze_(maze) {}

  std::vector<Coords> neighbors(const Coords &point) const override;
};

class RecursiveMazeGraph
    : public aoc::graphs::Graph<RecursiveCoords, std::vector<RecursiveCoords>> {
  const DonutMaze *maze_;

public:
  explicit RecursiveMazeGraph(const DonutMaze *maze) : maze_(maze) {}

  std::vector<RecursiveCoords>
  neighbors(const RecursiveCoords &point) const override;
};

class DonutMaze {
  std::vector<std::vector<char>> map_;
  std::unordered_map<std::string, std::array<Coords, 2>> labels_;
  std::unordered_map<Coords, Coords> down_portals_;
  std::unordered_map<Coords, Coords> up_portals_;
  Coords start_;
  Coords end_;

public:
  DonutMaze(const std::vector<std::string> &lines) {
    int height = static_cast<int>(lines.size() - 4);
    int width = height ? static_cast<int>(lines.at(0).size() - 4) : 0;

    for (std::size_t i = 2; i < lines.size() - 2; ++i) {
      map_.emplace_back();
      const auto &line = lines.at(i);
      if (static_cast<int>(line.size() - 4) != width) {
        throw std::invalid_argument("Non-rectangular map");
      }
      for (std::size_t j = 2; j < line.size() - 2; ++j) {
        char c = line.at(j);
        map_.back().push_back(std::isupper(c) ? ' ' : c);
      }
    }

    Coords hole_start = Coords({0, 0});
    Coords hole_end = Coords({0, 0});
    for (int y = 0; y < height; ++y) {
      for (int x = 0; x < width; ++x) {
        char c = lines.at(y + 2).at(x + 2);
        if (c != '#' && c != '.') {
          if (hole_start == Coords({0, 0})) {
            hole_start = {x, y};
          } else {
            hole_end = {x, y};
          }
        }
      }
    }

    for (int y = 0; y < height; ++y) {
      int line_width = lines.at(y + 2).size();
      if (std::isupper(lines.at(y + 2).at(1))) {
        std::string label =
            std::string(1, lines.at(y + 2).at(0)) + lines.at(y + 2).at(1);
        labels_[label][0] = {0, y};
      }
      if (std::isupper(lines.at(y + 2).at(line_width - 2))) {
        std::string label = std::string(1, lines.at(y + 2).at(line_width - 2)) +
                            lines.at(y + 2).at(line_width - 1);
        labels_[label][0] = {line_width - 5, y};
      }
      if (std::isupper(lines.at(y + 2).at(hole_start[0] + 2))) {
        std::string label =
            std::string(1, lines.at(y + 2).at(hole_start[0] + 2)) +
            lines.at(y + 2).at(hole_start[0] + 3);
        labels_[label][1] = {hole_start[0] - 1, y};
      }
      if (std::isupper(lines.at(y + 2).at(hole_end[0] + 2))) {
        std::string label =
            std::string(1, lines.at(y + 2).at(hole_end[0] + 1)) +
            lines.at(y + 2).at(hole_end[0] + 2);
        labels_[label][1] = {hole_end[0] + 1, y};
      }
    }
    for (int x = 0; x < width; ++x) {
      int line_height = lines.size();
      if (std::isupper(lines.at(1).at(x + 2))) {
        std::string label =
            std::string(1, lines.at(0).at(x + 2)) + lines.at(1).at(x + 2);
        labels_[label][0] = {x, 0};
      }
      if (std::isupper(lines.at(line_height - 2).at(x + 2))) {
        std::string label =
            std::string(1, lines.at(line_height - 2).at(x + 2)) +
            lines.at(line_height - 1).at(x + 2);
        labels_[label][0] = {x, line_height - 5};
      }
      if (std::isupper(lines.at(hole_start[1] + 2).at(x + 2))) {
        std::string label =
            std::string(1, lines.at(hole_start[1] + 2).at(x + 2)) +
            lines.at(hole_start[1] + 3).at(x + 2);
        labels_[label][1] = {x, hole_start[1] - 1};
      }
      if (std::isupper(lines.at(hole_end[1] + 2).at(x + 2))) {
        std::string label =
            std::string(1, lines.at(hole_end[1] + 1).at(x + 2)) +
            lines.at(hole_end[1] + 2).at(x + 2);
        labels_[label][1] = {x, hole_end[1] + 1};
      }
    }

    start_ = labels_.at("AA").at(0);
    end_ = labels_.at("ZZ").at(0);
    labels_.erase("AA");
    labels_.erase("ZZ");

    for (const auto &[label, pair] : labels_) {
      const auto &[up, down] = pair;
      down_portals_[down] = up;
      up_portals_[up] = down;
    }
  }

  char at(int x, int y) const {
    if (y < 0 || y >= static_cast<int>(map_.size()) || x < 0 ||
        x >= static_cast<int>(map_.at(0).size())) {
      return '#';
    } else {
      return map_.at(y).at(x);
    }
  }

  std::optional<std::pair<Coords, PortalType>> portal_at(Coords point) const {
    auto search = down_portals_.find(point);
    if (search != down_portals_.end()) {
      return std::make_pair(search->second, PortalType::DOWN);
    }
    search = up_portals_.find(point);
    if (search != up_portals_.end()) {
      return std::make_pair(search->second, PortalType::UP);
    }
    return std::nullopt;
  }

  int solution_length() const {
    auto graph = SimpleMazeGraph(this);
    auto bfs = aoc::graphs::BFS<Coords, std::vector<Coords>>(graph, start_);
    for (const auto &node : bfs) {
      if (node.value == end_) {
        return node.depth;
      }
    }
    throw std::runtime_error("End of maze not found");
  }

  int recursive_solution_length() const {
    auto start = RecursiveCoords({start_[0], start_[1], 0});
    auto destination = RecursiveCoords({end_[0], end_[1], 0});
    auto graph = RecursiveMazeGraph(this);
    auto bfs = aoc::graphs::BFS<RecursiveCoords, std::vector<RecursiveCoords>>(
        graph, start);
    for (const auto &node : bfs) {
      if (node.value == destination) {
        return node.depth;
      }
    }

    throw std::runtime_error("End of maze not found");
  }
};

std::vector<Coords> SimpleMazeGraph::neighbors(const Coords &point) const {
  std::vector<Coords> results;

  std::vector<Coords> offsets = {{1, 0}, {-1, 0}, {0, 1}, {0, -1}};
  for (const auto &offset : offsets) {
    Coords new_point = {point[0] + offset[0], point[1] + offset[1]};
    if (maze_->at(new_point[0], new_point[1]) == '.') {
      results.push_back(new_point);
    }
  }
  auto maybe_portal = maze_->portal_at(point);
  if (maybe_portal.has_value()) {
    results.push_back(maybe_portal.value().first);
  }

  return results;
}

std::vector<RecursiveCoords>
RecursiveMazeGraph::neighbors(const RecursiveCoords &point) const {
  std::vector<RecursiveCoords> results;
  int x = point[0];
  int y = point[1];
  int depth = point[2];
  std::vector<Coords> offsets = {{1, 0}, {-1, 0}, {0, 1}, {0, -1}};
  for (const auto &offset : offsets) {
    RecursiveCoords new_point = {x + offset[0], y + offset[1], depth};
    if (maze_->at(x, y) == '.') {
      results.push_back(new_point);
    }
  }
  auto maybe_portal = maze_->portal_at({x, y});
  if (maybe_portal.has_value()) {
    const auto [destination, type] = maybe_portal.value();
    int new_x = destination[0];
    int new_y = destination[1];
    switch (type) {
    case PortalType::DOWN:
      results.push_back(RecursiveCoords({new_x, new_y, depth + 1}));
      break;
    case PortalType::UP:
      if (depth > 0) {
        results.push_back(RecursiveCoords({new_x, new_y, depth - 1}));
      }
      break;
    default:
      throw std::runtime_error("Unexpected PortalType.");
    }
  }

  return results;
}

void run_tests() {
  DonutMaze maze_1({
      "         A           ", "         A           ", "  #######.#########  ",
      "  #######.........#  ", "  #######.#######.#  ", "  #######.#######.#  ",
      "  #######.#######.#  ", "  #####  B    ###.#  ", "BC...##  C    ###.#  ",
      "  ##.##       ###.#  ", "  ##...DE  F  ###.#  ", "  #####    G  ###.#  ",
      "  #########.#####.#  ", "DE..#######...###.#  ", "  #.#########.###.#  ",
      "FG..#########.....#  ", "  ###########.#####  ", "             Z       ",
      "             Z       ",
  });
  assert(maze_1.solution_length() == 23);

  DonutMaze maze_2({
      "                   A               ",
      "                   A               ",
      "  #################.#############  ",
      "  #.#...#...................#.#.#  ",
      "  #.#.#.###.###.###.#########.#.#  ",
      "  #.#.#.......#...#.....#.#.#...#  ",
      "  #.#########.###.#####.#.#.###.#  ",
      "  #.............#.#.....#.......#  ",
      "  ###.###########.###.#####.#.#.#  ",
      "  #.....#        A   C    #.#.#.#  ",
      "  #######        S   P    #####.#  ",
      "  #.#...#                 #......VT",
      "  #.#.#.#                 #.#####  ",
      "  #...#.#               YN....#.#  ",
      "  #.###.#                 #####.#  ",
      "DI....#.#                 #.....#  ",
      "  #####.#                 #.###.#  ",
      "ZZ......#               QG....#..AS",
      "  ###.###                 #######  ",
      "JO..#.#.#                 #.....#  ",
      "  #.#.#.#                 ###.#.#  ",
      "  #...#..DI             BU....#..LF",
      "  #####.#                 #.#####  ",
      "YN......#               VT..#....QG",
      "  #.###.#                 #.###.#  ",
      "  #.#...#                 #.....#  ",
      "  ###.###    J L     J    #.#.###  ",
      "  #.....#    O F     P    #.#...#  ",
      "  #.###.#####.#.#####.#####.###.#  ",
      "  #...#.#.#...#.....#.....#.#...#  ",
      "  #.#####.###.###.#.#.#########.#  ",
      "  #...#.#.....#...#.#.#.#.....#.#  ",
      "  #.###.#####.###.###.#.#.#######  ",
      "  #.#.........#...#.............#  ",
      "  #########.###.###.#############  ",
      "           B   J   C               ",
      "           U   P   P               ",
  });
  assert(maze_2.solution_length() == 58);

  DonutMaze maze_3({
      "             Z L X W       C                 ",
      "             Z P Q B       K                 ",
      "  ###########.#.#.#.#######.###############  ",
      "  #...#.......#.#.......#.#.......#.#.#...#  ",
      "  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  ",
      "  #.#...#.#.#...#.#.#...#...#...#.#.......#  ",
      "  #.###.#######.###.###.#.###.###.#.#######  ",
      "  #...#.......#.#...#...#.............#...#  ",
      "  #.#########.#######.#.#######.#######.###  ",
      "  #...#.#    F       R I       Z    #.#.#.#  ",
      "  #.###.#    D       E C       H    #.#.#.#  ",
      "  #.#...#                           #...#.#  ",
      "  #.###.#                           #.###.#  ",
      "  #.#....OA                       WB..#.#..ZH",
      "  #.###.#                           #.#.#.#  ",
      "CJ......#                           #.....#  ",
      "  #######                           #######  ",
      "  #.#....CK                         #......IC",
      "  #.###.#                           #.###.#  ",
      "  #.....#                           #...#.#  ",
      "  ###.###                           #.#.#.#  ",
      "XF....#.#                         RF..#.#.#  ",
      "  #####.#                           #######  ",
      "  #......CJ                       NM..#...#  ",
      "  ###.#.#                           #.###.#  ",
      "RE....#.#                           #......RF",
      "  ###.###        X   X       L      #.#.#.#  ",
      "  #.....#        F   Q       P      #.#.#.#  ",
      "  ###.###########.###.#######.#########.###  ",
      "  #.....#...#.....#.......#...#.....#.#...#  ",
      "  #####.#.###.#######.#######.###.###.#.#.#  ",
      "  #.......#.......#.#.#.#.#...#...#...#.#.#  ",
      "  #####.###.#####.#.#.#.#.###.###.#.###.###  ",
      "  #.......#.....#.#...#...............#...#  ",
      "  #############.#.#.###.###################  ",
      "               A O F   N                     ",
      "               A A D   M                     ",
  });
  assert(maze_3.recursive_solution_length() == 396);
}

int p1(const std::vector<std::string> &lines) {
  DonutMaze maze(lines);
  return maze.solution_length();
}

int p2(const std::vector<std::string> &lines) {
  DonutMaze maze(lines);
  return maze.recursive_solution_length();
}

int main() {
  try {
    run_tests();
    std::cout << "All tests passed" << std::endl;

    std::vector<std::string> lines = aoc::strings::getlines();

    std::cout << "Part 1: " << p1(lines) << std::endl;
    std::cout << "Part 2: " << p2(lines) << std::endl;
  } catch (const std::exception &e) {
    std::cerr << e.what() << std::endl;
    return 1;
  }
}
