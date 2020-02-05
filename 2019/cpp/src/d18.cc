#include <direction.h>
#include <graphs.h>
#include <point.h>
#include <strings.h>

#include <cctype>
#include <iostream>
#include <queue>
#include <set>
#include <stdexcept>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <utility>
#include <vector>

typedef char Tile;
typedef aoc::point::Point<int, 2> Coords;
typedef const std::vector<Coords> Neighbors;

struct MazeState {
  std::set<Tile> robot_locations;
  std::set<Tile> collected_keys;

  MazeState(std::set<Tile> robot_locations, std::set<Tile> collected_keys)
      : robot_locations(std::move(robot_locations)),
        collected_keys(std::move(collected_keys)) {}

  MazeState() {}
};

bool operator==(const MazeState &lhs, const MazeState &rhs) {
  return lhs.robot_locations == rhs.robot_locations &&
         lhs.collected_keys == rhs.collected_keys;
}

namespace std {

template <>
struct hash<MazeState> {
  std::size_t operator()(const MazeState &state) const {
    std::size_t value = 0;
    for (Tile tile : state.collected_keys) {
      value = value * 101 + tile;
    }
    for (Tile tile : state.robot_locations) {
      value = value * 101 + tile;
    }
    return value;
  }
};

}  // namespace std

class KeyMaze : public aoc::graphs::Graph<Coords, Neighbors> {
 public:
  KeyMaze(const std::vector<std::string> &lines)
      : map_(lines.size(), std::vector<Tile>(lines.at(0).size())) {
    for (int y = 0; y < lines.size(); ++y) {
      const auto &line = lines.at(y);
      for (int x = 0; x < line.size(); ++x) {
        Tile tile = line.at(x);
        if (tile == '@') {
          tile = entry_point_count_ + '0';
          ++entry_point_count_;
        }
        map_[y][x] = tile;
        if (std::islower(tile) || std::isdigit(tile)) {
          waypoints_[tile] = {x, y};
        }
      }
    }
  }

  int steps() {
    if (!initialized_) {
      throw std::runtime_error("Must initialize first");
    }
    std::set<Tile> entry_points;
    for (int i = 0; i < entry_point_count_; ++i) {
      entry_points.insert(i + '0');
    }
    MazeState starting_state = MazeState(std::move(entry_points), {});
    auto comp = [](const auto &lhs, const auto &rhs) {
      return lhs.first < rhs.first;
    };
    std::priority_queue<std::pair<int, MazeState>,
                        std::vector<std::pair<int, MazeState>>, decltype(comp)>
        to_process(comp);
    to_process.push(std::make_pair(0, starting_state));
    std::unordered_map<MazeState, int> distances;
    distances[starting_state] = 0;

    while (!to_process.empty()) {
      MazeState maze_state = to_process.top().second;
      to_process.pop();

      for (Tile from_tile : maze_state.robot_locations) {
        for (const auto [to_tile, distance] : waypoint_distances_[from_tile]) {
          bool already_visited = maze_state.collected_keys.find(to_tile) !=
                                 maze_state.collected_keys.end();
          bool has_blocking_doors =
              std::any_of(waypoint_doors_[from_tile][to_tile].begin(),
                          waypoint_doors_[from_tile][to_tile].end(),
                          [&maze_state](Tile door) {
                            return maze_state.collected_keys.find(door) ==
                                   maze_state.collected_keys.end();
                          });
          if (!already_visited && !has_blocking_doors) {
            std::set<Tile> new_locations = maze_state.robot_locations;
            new_locations.erase(from_tile);
            new_locations.insert(to_tile);
            std::set<Tile> new_keys = maze_state.collected_keys;
            new_keys.insert(to_tile);
            MazeState new_state =
                MazeState(std::move(new_locations), std::move(new_keys));
            int extra_distance = waypoint_distances_.at(from_tile).at(to_tile);
            int new_distance = distances.at(maze_state) + extra_distance;
            if (distances.find(new_state) == distances.end() ||
                new_distance < distances.at(new_state)) {
              to_process.push(std::make_pair(new_distance, new_state));
              distances[new_state] = new_distance;
            }
          }
        }
      }
    }

    int min_distance = std::numeric_limits<int>::max();
    for (const auto &[state, distance] : distances) {
      if (state.collected_keys.size() == key_count()) {
        min_distance = std::min(min_distance, distance);
      }
    }
    return min_distance;
  }

  int height() const { return map_.size(); }

  int width() const { return map_.size() > 0 ? map_.at(0).size() : 0; }

  int key_count() const {
    return waypoint_distances_.size() - entry_point_count_;
  }

  Tile at(const Coords &position) const {
    int x = position[0];
    int y = position[1];
    if (0 <= x && x < width() && 0 <= y && y < height()) {
      return map_.at(y).at(x);
    }

    return '#';
  }

  Neighbors neighbors(const Coords &position) const override {
    std::vector<Coords> neighbors;
    for (int i = 0; i < 4; ++i) {
      auto direction = aoc::direction::Direction(i);
      auto neighbor = aoc::direction::step(position, direction);
      if (at(neighbor) != '#') {
        neighbors.push_back(std::move(neighbor));
      }
    }
    return neighbors;
  }

  void initialize() {
    for (const auto &[from_tile, position] : waypoints_) {
      auto bfs = aoc::graphs::BFS<Coords, Neighbors>(*this, position);
      for (const auto &node : bfs) {
        Tile to_tile = at(node.value);
        if (std::islower(to_tile)) {
          waypoint_distances_[from_tile][to_tile] = node.depth;
          if (node.parent) {
            for (auto n = node.parent; n->parent; n = n->parent) {
              Tile t = at(n->value);
              if (std::isupper(t)) {
                waypoint_doors_[from_tile][to_tile].insert(std::tolower(t));
              }
            }
          }
        }
      }
    }
    initialized_ = true;
  }

 private:
  std::vector<std::vector<Tile>> map_;
  std::unordered_map<Tile, Coords> waypoints_;
  std::unordered_map<Tile, std::unordered_map<Tile, int>> waypoint_distances_;
  std::unordered_map<Tile, std::unordered_map<Tile, std::unordered_set<Tile>>>
      waypoint_doors_;
  int entry_point_count_ = 0;
  bool initialized_ = false;
};

int p1(const std::vector<std::string> &lines) {
  KeyMaze maze = KeyMaze(lines);
  maze.initialize();
  return maze.steps();
}

int p2(const std::vector<std::string> &lines) {
  std::vector<std::string> modified_lines = lines;
  for (int y = 0; y < modified_lines.size(); ++y) {
    for (int x = 0; x < modified_lines.at(y).size(); ++x) {
      if (modified_lines.at(y).at(x) == '@') {
        modified_lines[y - 1][x - 1] = '@';
        modified_lines[y - 1][x] = '#';
        modified_lines[y - 1][x + 1] = '@';
        modified_lines[y][x - 1] = '#';
        modified_lines[y][x] = '#';
        modified_lines[y][x + 1] = '#';
        modified_lines[y + 1][x - 1] = '@';
        modified_lines[y + 1][x] = '#';
        modified_lines[y + 1][x + 1] = '@';
      }
    }
  }
  KeyMaze maze = KeyMaze(modified_lines);
  maze.initialize();
  return maze.steps();
}

int main() {
  try {
    std::vector<std::string> lines = aoc::strings::getlines();

    std::cout << "Part 1: " << p1(lines) << std::endl;
    std::cout << "Part 2: " << p2(lines) << std::endl;
  } catch (const std::exception &e) {
    std::cerr << e.what() << std::endl;
    return 1;
  }
}
