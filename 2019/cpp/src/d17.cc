#include <iostream>
#include <optional>
#include <stdexcept>
#include <string_view>
#include <unordered_set>
#include <vector>

#include <direction.h>
#include <intcode.h>
#include <point.h>
#include <strings.h>

using ::aoc::direction::Direction;

typedef ::aoc::point::Point<long long, 2> Coords;

class Scaffold {
  std::vector<std::vector<char>> map_;

  char at(int x, int y) { return map_.at(height() - y - 1).at(x); }

  bool on_scaffold(int x, int y) {
    return 0 <= x && x < width() && 0 <= y && y < height() && at(x, y) != '.';
  }

  std::vector<Coords> neighbors(int x, int y) {
    std::vector<Coords> neighbors;
    for (int i = 0; i < 4; ++i) {
      Coords offset =
          aoc::direction::offset<long long>(static_cast<Direction>(i));
      if (on_scaffold(x + offset[0], y + offset[1])) {
        neighbors.push_back({x + offset[0], y + offset[1]});
      }
    }
    return neighbors;
  }

  std::optional<Coords> vacuum_location() {
    for (int x = 0; x < width(); ++x) {
      for (int y = 0; y < height(); ++y) {
        if (is_vacuum(x, y)) {
          return Coords({x, y});
        }
      }
    }
    return std::nullopt;
  }

  Direction robot_direction(char c) {
    switch (c) {
    case '^':
      return Direction::NORTH;
    case '>':
      return Direction::EAST;
    case 'v':
      return Direction::SOUTH;
    case '<':
      return Direction::WEST;
    default:
      throw std::invalid_argument("Non-robot tile");
    }
  }

  bool is_vacuum(int x, int y) {
    char c = at(x, y);
    return c == '^' || c == '>' || c == 'v' || c == '<' || c == 'X';
  }

public:
  Scaffold(std::vector<std::vector<char>> map) : map_(map) {}

  std::vector<Coords> intersections() {
    std::vector<Coords> points;
    for (int y = 1; y < height() - 1; ++y) {
      for (int x = 1; x < width() - 1; ++x) {
        if (on_scaffold(x, y) && neighbors(x, y).size() == 4) {
          points.push_back({x, y});
        }
      }
    }
    return points;
  }

  std::vector<std::string> full_directions() {
    Coords position = vacuum_location().value();
    std::unordered_set<Coords> visited;
    visited.insert(position);

    std::vector<std::string> directions;
    Direction current_direction = robot_direction(at(position[0], position[1]));
    while (true) {
      auto offset = aoc::direction::offset<long long>(current_direction);
      int distance = 0;
      while (on_scaffold(position[0] + offset[0], position[1] + offset[1])) {
        ++distance;
        position[0] += offset[0];
        position[1] += offset[1];
        visited.insert(position);
      }
      if (distance > 0) {
        directions.push_back(std::to_string(distance));
      } else {
        std::vector<Coords> candidates = neighbors(position[0], position[1]);
        auto candidates_end = std::remove_if(
            candidates.begin(), candidates.end(), [&visited](const auto &p) {
              return visited.find(p) != visited.end();
            });
        if (candidates_end == candidates.begin()) {
          break;
        }
        auto new_position = candidates.at(0);
        auto new_direction = aoc::direction::from_offset(Coords(
            {new_position[0] - position[0], new_position[1] - position[1]}));
        int turn_count = 0;
        while (current_direction != new_direction) {
          current_direction = aoc::direction::right_turn(current_direction);
          ++turn_count;
        }
        if (turn_count == 3) {
          directions.push_back("L");
        } else {
          for (int i = 0; i < turn_count; ++i) {
            directions.push_back("R");
          }
        }
      }
    }

    return directions;
  }

  int height() { return map_.size(); }

  int width() { return map_.size() > 0 ? map_.at(0).size() : 0; }
};

std::vector<std::vector<char>> get_map(aoc::intcode::VM *vm) {
  std::vector<std::vector<char>> map;
  map.emplace_back();
  for (auto output = vm->get_next_output(); output.has_value();
       output = vm->get_next_output()) {
    char c = (char)output.value();
    if (c == '\n') {
      map.emplace_back();
    } else {
      map.back().push_back(c);
    }
  }
  while (map.back().empty()) {
    map.pop_back();
  }
  int width = map.at(0).size();
  for (const auto &row : map) {
    if (static_cast<int>(row.size()) != width) {
      throw std::runtime_error("Invalid map!");
    }
  }

  return map;
}

bool is_valid_routine(const std::string_view routine) {
  bool expecting_comma = false;
  for (char c : routine) {
    if (expecting_comma) {
      if (c != ',') {
        return false;
      }
    } else {
      if (c != 'A' && c != 'B' && c != 'C') {
        return false;
      }
    }
    expecting_comma = !expecting_comma;
  }
  return true;
}

std::optional<std::vector<std::string>>
get_routines(const std::string &full_routine) {
  std::size_t max_function_length = 20;
  for (std::size_t a_len = max_function_length; a_len > 0; --a_len) {
    if (full_routine[a_len] != ',') {
      continue;
    }
    std::string_view function_a = full_routine;
    function_a = function_a.substr(0, a_len);
    std::string routine_a =
        aoc::strings::replace_substring(full_routine, function_a, "A");

    std::size_t b_start = routine_a.find_first_not_of("A,");
    if (b_start == routine_a.npos) {
      return std::vector<std::string>(
          {routine_a, std::string(function_a), "", ""});
    }
    for (int b_len = max_function_length; b_len > 0; --b_len) {
      if (b_start + b_len >= routine_a.npos ||
          routine_a[b_start + b_len] != ',') {
        continue;
      }
      std::string_view function_b = routine_a;
      function_b = function_b.substr(b_start, b_len);
      if (function_b.find('A') != function_b.npos) {
        continue;
      }
      std::string routine_b =
          aoc::strings::replace_substring(routine_a, function_b, "B");

      std::size_t c_start = routine_b.find_first_not_of("AB,");
      if (c_start == routine_b.npos) {
        return std::vector<std::string>(
            {routine_b, std::string(function_a), std::string(function_b), ""});
      }
      std::size_t c_end = routine_b.find_first_of("AB", c_start);
      std::size_t c_len;
      if (c_end == routine_b.npos) {
        c_len = c_end - c_start;
      } else {
        if (routine_b[c_end - 1] != ',') {
          continue;
        }
        c_len = c_end - c_start - 1;
      }

      std::string_view function_c = routine_b;
      function_c = function_c.substr(c_start, c_len);
      std::string main_routine =
          aoc::strings::replace_substring(routine_b, function_c, "C");
      if (is_valid_routine(main_routine)) {
        return std::vector<std::string>({main_routine, std::string(function_a),
                                         std::string(function_b),
                                         std::string(function_c)});
      }
    }
  }
  return std::nullopt;
}

int p1(const std::vector<long long> &program) {
  aoc::intcode::VM vm(program, []() { return 0; });
  auto map = get_map(&vm);
  Scaffold scaffold(std::move(map));
  int total = 0;
  for (const auto &point : scaffold.intersections()) {
    total += point[0] * (scaffold.height() - point[1] - 1);
  }
  return total;
}

int p2(const std::vector<long long> &program) {
  // Build the inputs we'll use to run the robot.
  aoc::intcode::VM vm(program, []() { return 0; });
  Scaffold scaffold(get_map(&vm));
  std::string full_routine =
      aoc::strings::join(scaffold.full_directions(), ",");
  auto routines = get_routines(full_routine);

  std::string input_string;
  for (const auto &s : routines.value()) {
    input_string += s;
    input_string += '\n';
  }
  input_string += "n\n";
  int i = 0;
  auto inputs = [&input_string, &i]() { return input_string[i++]; };

  // Run the robot using the inputs generated above.
  auto mod_program = program;
  mod_program[0] = 2;
  vm = aoc::intcode::VM(mod_program, inputs);
  long long value = 0;
  for (auto output = vm.get_next_output(); output.has_value();
       output = vm.get_next_output()) {
    value = output.value();
  }
  return value;
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
