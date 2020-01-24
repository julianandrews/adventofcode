#include <cmath>
#include <optional>
#include <sstream>
#include <stdexcept>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <vector>

#include "graphs.h"
#include "strings.h"

typedef const ::std::unordered_set<std::string> &Neighbors;

struct Material {
  std::string kind;
  long long quantity;
};

struct Reaction {
  Material output;
  std::vector<Material> inputs;
};

class ReactionGraph : public aoc::graphs::Graph<std::string, Neighbors> {
  std::unordered_map<std::string, std::unordered_set<std::string>> inputs_;
  std::unordered_map<std::string, Reaction> reactions_;
  std::unordered_set<std::string> EMPTY_;

public:
  ReactionGraph(std::vector<Reaction> reactions) {
    for (const auto &reaction : reactions) {
      std::unordered_set<std::string> input_kinds;
      for (const auto &material : reaction.inputs) {
        input_kinds.insert(material.kind);
      }
      inputs_[reaction.output.kind] = input_kinds;
      reactions_.emplace(reaction.output.kind, reaction);
    }
  }

  const std::optional<Reaction> reaction_for_kind(const std::string &kind) {
    auto it = reactions_.find(kind);
    return it == reactions_.end() ? std::nullopt
                                  : std::optional<Reaction>(it->second);
  }

  Neighbors neighbors(const std::string &kind) const override {
    return inputs_.find(kind) != inputs_.end() ? inputs_.at(kind)
                                               : ReactionGraph::EMPTY_;
  }
};

std::vector<Reaction> parse_reactions(std::vector<std::string> lines) {
  auto parse_material = [](std::string s) {
    auto i = s.find(" ");
    return Material(
        {s.substr(i + 1, s.length() - i - 1), std::stoi(s.substr(0, i))});
  };

  std::vector<Reaction> reactions;
  for (const auto &line : lines) {
    auto i = line.find(" => ");
    std::string input_part = line.substr(0, i);
    std::vector<Material> inputs;
    int start = 0;
    while (start < input_part.length()) {
      auto end = std::min(input_part.find(", ", start), input_part.length());
      inputs.push_back(parse_material(input_part.substr(start, end - start)));
      start = end + 2;
    }

    reactions.push_back(
        {parse_material(line.substr(i + 4, line.length() - i - 4)),
         std::move(inputs)});
  }

  return reactions;
}

std::vector<Material> raw_inputs(const std::vector<Reaction> &reactions,
                                 const Material &goal) {
  std::unordered_map<std::string, long long> needed_materials;
  needed_materials[goal.kind] = goal.quantity;
  ReactionGraph graph = ReactionGraph(reactions);
  std::unordered_set<std::string> all_kinds;
  for (const auto &reaction : reactions) {
    all_kinds.insert(reaction.output.kind);
    for (const auto &material : reaction.inputs) {
      all_kinds.insert(material.kind);
    }
  }

  std::vector<Material> inputs;
  auto toposort =
      aoc::graphs::Toposort<std::string, Neighbors>(graph, all_kinds);
  for (const auto &kind : toposort) {
    if (needed_materials.find(kind) != needed_materials.end()) {
      const long long needed_quantity = needed_materials[kind];
      const auto reaction = graph.reaction_for_kind(kind);
      if (reaction.has_value()) {
        long long multiple =
            ceil((double)needed_quantity / reaction->output.quantity);
        for (const auto &material : reaction->inputs) {
          needed_materials[material.kind] += material.quantity * multiple;
        }
      } else {
        inputs.push_back({kind, needed_quantity});
      }
    }
  }

  return inputs;
}

long long required_ore(const std::vector<Reaction> &reactions,
                       Material material) {
  auto materials = raw_inputs(reactions, material);
  for (const auto &[kind, quantity] : materials) {
    if (kind == "ORE") {
      return quantity;
    }
  }
  return 0;
}

long long ore_fuel_yield(const std::vector<Reaction> &reactions,
                         long long available_ore) {
  long long min_fuel = 0;

  long long ceiling = -1;
  while (ceiling == -1 or ceiling - min_fuel > 1) {
    long long fuel =
        ceiling == -1 ? std::max(2 * min_fuel, 1ll) : (min_fuel + ceiling) / 2;
    long long ore = required_ore(reactions, {"FUEL", fuel});
    if (ore > available_ore) {
      ceiling = fuel;
    } else {
      min_fuel = fuel;
    }
  }

  return min_fuel;
}

int p1(const std::vector<Reaction> &reactions) {
  return required_ore(reactions, {"FUEL", 1});
}

int p2(const std::vector<Reaction> &reactions) {
  return ore_fuel_yield(reactions, 1000000000000);
}

int main() {
  try {
    std::vector<Reaction> reactions = parse_reactions(aoc::strings::getlines());

    std::cout << "Part 1: " << p1(reactions) << std::endl;
    std::cout << "Part 2: " << p2(reactions) << std::endl;
  } catch (const std::exception &e) {
    std::cerr << e.what() << std::endl;
    return 1;
  }
}
