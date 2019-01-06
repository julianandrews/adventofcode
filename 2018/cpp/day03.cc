#include <experimental/optional>
#include <string>
#include <unordered_set>
#include <vector>

#include "point.h"
#include "utils.h"

class Claim {
public:
  const long id, x, y, w, h;

  static Claim from_string(const std::string &s) {
    auto words = aoc::utils::split(s, ' ');
    auto coord_strings = aoc::utils::split(words[2], ',');
    auto dimension_strings = aoc::utils::split(words[3], 'x');

    return Claim(std::stol(words[0].substr(1)), std::stol(coord_strings[0]),
                 std::stol(coord_strings[1]), std::stol(dimension_strings[0]),
                 std::stol(dimension_strings[1]));
  }

  std::vector<aoc::point::Point> points() const {
    std::vector<aoc::point::Point> points;
    for (auto i = x; i < x + w; i++) {
      for (auto j = y; j < y + h; j++) {
        points.push_back({i, j});
      }
    }

    return points;
  }

private:
  Claim(long id, long x, long y, long w, long h)
      : id(id), x(x), y(y), w(w), h(h) {}
};

std::unordered_set<aoc::point::Point>
get_overlap(const std::vector<Claim> &claims) {
  std::unordered_set<aoc::point::Point> overlap;
  std::unordered_set<aoc::point::Point> all_points;

  for (auto claim : claims) {
    auto claim_points = claim.points();
    for (const auto &point : claim_points) {
      if (all_points.count(point)) {
        overlap.insert(point);
      }
      all_points.insert(point);
    }
  }

  return overlap;
}

int p1(const std::vector<Claim> &claims) {
  auto overlap = get_overlap(claims);

  return overlap.size();
}

std::experimental::optional<long> p2(const std::vector<Claim> &claims) {
  const auto overlap = get_overlap(claims);
  std::unordered_set<long> good_claims;

  for (const auto &claim : claims) {
    auto claim_points = claim.points();
    if (!std::any_of(claim_points.begin(), claim_points.end(),
                     [&overlap](auto p) { return overlap.count(p); })) {
      good_claims.insert(claim.id);
    }
  }

  return good_claims.size() == 1
             ? std::experimental::optional<long>(*good_claims.begin())
             : std::experimental::nullopt;
}

int main() {
  const std::vector<std::string> lines = aoc::utils::getlines();
  std::vector<Claim> claims;
  std::transform(lines.begin(), lines.end(), std::back_inserter(claims),
                 Claim::from_string);

  std::cout << "Part 1: " << p1(claims) << std::endl;
  std::cout << "Part 2: " << p2(claims).value_or(-1) << std::endl;
}
