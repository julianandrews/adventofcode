#include <algorithm>
#include <cstdlib>
#include <numeric>
#include <string>
#include <vector>

#include "utils.h"

class Package {
public:
  Package(int length, int width, int height)
      : length_(length), width_(width), height_(height){};

  static Package make_package(const std::string &s) {
    auto parts = aoc::utils::split(s, 'x');
    std::vector<int> lengths;
    for (const auto &part : parts) {
      lengths.push_back(std::stoi(part));
    }
    if (lengths.size() != 3) {
      throw std::runtime_error("Invalid package: " + s);
    }

    return Package(lengths[0], lengths[1], lengths[2]);
  }

  int wrapping_paper() const {
    auto sides = {length_ * width_, width_ * height_, height_ * length_};
    return 2 * std::accumulate(sides.begin(), sides.end(), 0) +
           *std::min_element(sides.begin(), sides.end());
  }

  int ribbon() const {
    auto perimeters = {2 * (length_ + width_), 2 * (width_ + height_),
                       2 * (height_ + length_)};
    return *std::min_element(perimeters.begin(), perimeters.end()) +
           length_ * width_ * height_;
  }

private:
  int length_;
  int width_;
  int height_;
};

int p1(const std::vector<Package> &packages) {
  int total = 0;
  for (const auto &p : packages) {
    total += p.wrapping_paper();
  }
  return total;
}

int p2(const std::vector<Package> &packages) {
  int total = 0;
  for (const auto &p : packages) {
    total += p.ribbon();
  }
  return total;
}

int main() {
  std::vector<std::string> lines = aoc::utils::get_trimmed_lines();
  std::vector<Package> packages;

  for (const auto &line : lines) {
    try {
      packages.push_back(Package::make_package(line));
    } catch (const std::invalid_argument &exception) {
      std::cerr << "Error parsing '" << line << "': " << exception.what()
                << std::endl;
      return 1;
    }
  }

  try {
    std::cout << "Part 1: " << p1(packages) << std::endl;
    std::cout << "Part 2: " << p2(packages) << std::endl;
  } catch (const std::exception &exception) {
    std::cerr << exception.what() << std::endl;
    return 1;
  }
}
