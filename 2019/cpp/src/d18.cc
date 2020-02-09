#include <direction.h>
#include <graphs.h>
#include <point.h>
#include <strings.h>

#include <algorithm>
#include <bitset>
#include <cassert>
#include <cctype>
#include <iostream>
#include <optional>
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

class MazeState {
  const long long collected_keys_ = 0;
  std::set<Tile> robot_locations_;

  MazeState(std::set<Tile> robot_locations, long long collected_keys)
      : robot_locations_(std::move(robot_locations)),
        collected_keys_(collected_keys) {}

public:
  MazeState next_state(Tile from_tile, Tile to_tile) const {
    auto new_locations = robot_locations_;
    new_locations.erase(from_tile);
    new_locations.insert(to_tile);
    auto new_keys = collected_keys_ | (1 << (to_tile - 'a'));
    return MazeState(std::move(new_locations), new_keys);
  }

  bool has_key(Tile key) const { return collected_keys_ & (1 << (key - 'a')); }

  int key_count() const {
    return std::bitset<std::numeric_limits<long long>::digits>(collected_keys_)
        .count();
  }

  std::string get_robot_tiles() const {
    std::string output;
    for (Tile tile : robot_locations_) {
      output.push_back(tile);
    }

    return output;
  }

  explicit MazeState(int num_entry_points) {
    for (int i = 0; i < num_entry_points; ++i) {
      robot_locations_.insert(i + '0');
    }
  }

  friend std::ostream &operator<<(std::ostream &os, const MazeState &state);
  friend bool operator==(const MazeState &lhs, const MazeState &rhs);
  friend struct std::hash<MazeState>;
};

std::ostream &operator<<(std::ostream &os, const MazeState &state) {
  os << "MazeState(";
  for (Tile tile : state.robot_locations_) {
    os << tile;
  }
  os << ", ";
  for (int i = 0; i < 64; ++i) {
    if (state.collected_keys_ & (1 << i)) {
      os << (char)(i + 'a');
    }
  }
  os << ")";
  return os;
}

bool operator==(const MazeState &lhs, const MazeState &rhs) {
  return lhs.robot_locations_ == rhs.robot_locations_ &&
         lhs.collected_keys_ == rhs.collected_keys_;
}

namespace std {

template <> struct hash<MazeState> {
  std::size_t operator()(const MazeState &state) const {
    std::size_t value = state.collected_keys_;
    for (Tile tile : state.robot_locations_) {
      value = value * 101 + tile;
    }
    return value;
  }
};

} // namespace std

class KeyMaze : public aoc::graphs::Graph<Coords, Neighbors> {
  bool should_visit(const MazeState &maze_state, Tile from_tile, Tile to_tile) {
    if (maze_state.has_key(to_tile)) {
      return false;
    }
    for (Tile door : waypoint_doors_[from_tile][to_tile]) {
      if (!maze_state.has_key(door)) {
        return false;
      }
    }
    return true;
  }

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

  std::optional<int> steps() {
    if (!initialized_) {
      throw std::runtime_error("Must initialize first");
    }

    typedef std::unordered_map<MazeState, int> DistanceMap;
    DistanceMap distances;

    const MazeState starting_state = MazeState(entry_point_count_);
    std::vector<std::pair<int, DistanceMap::iterator>> to_process;
    auto [it, _] =
        distances.insert(std::make_pair(std::move(starting_state), 0));
    to_process.push_back(std::make_pair(0, it));
    auto comapare_first = [](const auto &lhs, const auto &rhs) {
      return lhs.first > rhs.first;
    };

    int key_count = waypoint_distances_.size() - entry_point_count_;
    while (!to_process.empty()) {
      const auto &maze_state = to_process.front().second->first;
      int current_distance = to_process.front().first;
      std::pop_heap(to_process.begin(), to_process.end(), comapare_first);
      to_process.pop_back();
      if (distances.at(maze_state) != current_distance) {
        continue;
      }
      if (maze_state.key_count() == key_count) {
        return current_distance;
      }

      for (Tile from_tile : maze_state.get_robot_tiles()) {
        for (const auto [to_tile, distance] :
             waypoint_distances_.at(from_tile)) {
          if (should_visit(maze_state, from_tile, to_tile)) {
            int new_distance = distances.at(maze_state) +
                               waypoint_distances_.at(from_tile).at(to_tile);

            MazeState new_state = maze_state.next_state(from_tile, to_tile);
            auto search = distances.find(new_state);
            if (search == distances.end()) {
              auto [it, _] = distances.insert(
                  std::make_pair(std::move(new_state), new_distance));
              to_process.push_back(std::make_pair(new_distance, it));
              std::push_heap(to_process.begin(), to_process.end(),
                             comapare_first);
            } else if (new_distance < search->second) {
              search->second = new_distance;
              to_process.push_back(std::make_pair(new_distance, search));
              std::push_heap(to_process.begin(), to_process.end(),
                             comapare_first);
            }
          }
        }
      }
    }
    return std::nullopt;
  }

  int height() const { return map_.size(); }

  int width() const { return map_.size() > 0 ? map_.at(0).size() : 0; }

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

std::vector<std::string> tweak_lines(const std::vector<std::string> &lines) {
  std::vector<std::string> mod_lines = lines;
  for (int y = 0; y < lines.size(); ++y) {
    for (int x = 0; x < lines.at(y).size(); ++x) {
      if (lines.at(y).at(x) == '@') {
        mod_lines[y - 1][x - 1] = '@';
        mod_lines[y - 1][x] = '#';
        mod_lines[y - 1][x + 1] = '@';
        mod_lines[y][x - 1] = '#';
        mod_lines[y][x] = '#';
        mod_lines[y][x + 1] = '#';
        mod_lines[y + 1][x - 1] = '@';
        mod_lines[y + 1][x] = '#';
        mod_lines[y + 1][x + 1] = '@';
      }
    }
  }

  return mod_lines;
}

int maze_steps(const std::vector<std::string> &lines) {
  KeyMaze maze = KeyMaze(lines);
  maze.initialize();
  auto steps = maze.steps();
  if (!steps.has_value()) {
    throw std::runtime_error("No path found through maze");
  }
  return steps.value();
}

void run_tests() {
  assert(maze_steps({
             "#########",
             "#b.A.@.a#",
             "#########",
         }) == 8);

  assert(maze_steps({
             "########################",
             "#f.D.E.e.C.b.A.@.a.B.c.#",
             "######################.#",
             "#d.....................#",
             "########################",
         }) == 86);

  assert(maze_steps({
             "########################",
             "#...............b.C.D.f#",
             "#.######################",
             "#.....@.a.B.c.d.A.e.F.g#",
             "########################",
         }) == 132);

  assert(maze_steps({
             "#################",
             "#i.G..c...e..H.p#",
             "########.########",
             "#j.A..b...f..D.o#",
             "########@########",
             "#k.E..a...g..B.n#",
             "########.########",
             "#l.F..d...h..C.m#",
             "#################",
         }) == 136);

  assert(maze_steps({
             "########################",
             "#@..............ac.GI.b#",
             "###d#e#f################",
             "###A#B#C################",
             "###g#h#i################",
             "########################",
         }) == 81);

  assert(maze_steps({
             "#######",
             "#a.#Cd#",
             "##@#@##",
             "#######",
             "##@#@##",
             "#cB#Ab#",
             "#######",
         }) == 8);

  assert(maze_steps({
             "###############",
             "#d.ABC.#.....a#",
             "######@#@######",
             "###############",
             "######@#@######",
             "#b.....#.....c#",
             "###############",
         }) == 24);

  assert(maze_steps({
             "#############",
             "#DcBa.#.GhKl#",
             "#.###@#@#I###",
             "#e#d#####j#k#",
             "###C#@#@###J#",
             "#fEbA.#.FgHi#",
             "#############",
         }) == 32);
}

int p1(const std::vector<std::string> &lines) { return maze_steps(lines); }

int p2(const std::vector<std::string> &lines) {
  return maze_steps(tweak_lines(lines));
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
