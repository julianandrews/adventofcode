#include <iostream>
#include <stdexcept>
#include <vector>

#include "strings.h"

class Image {
  const int width_;
  const int height_;
  const std::vector<int> pixels_;

public:
  Image(std::vector<int> pixels, int width, int height)
      : width_(width), height_(height), pixels_(pixels) {}

  int layer_size() const { return width_ * height_; }

  int num_layers() const { return pixels_.size() / layer_size(); }

  int count_digit(int layer, int digit) const {
    int count = 0;
    for (int i = layer * layer_size(); i < (layer + 1) * layer_size(); ++i) {
      if (pixels_.at(i) == digit) {
        ++count;
      }
    }

    return count;
  }

  int get_pixel(int x, int y) const {
    int pixel;
    for (int layer = 0; layer < num_layers(); ++layer) {
      pixel = pixels_.at(layer * layer_size() + y * width_ + x);
      if (pixel != 2) {
        break;
      }
    }

    return pixel;
  }

  std::string output() const {
    std::stringstream ss;
    for (int y = 0; y < height_; ++y) {
      for (int x = 0; x < width_; ++x) {
        switch (get_pixel(x, y)) {
        case 0:
          ss << " ";
          break;
        case 1:
          ss << "█";
          break;
        case 2:
          ss << "░";
          break;
        default:
          throw std::runtime_error("Unexpected pixel value");
        }
      }
      ss << std::endl;
    }

    return ss.str();
  }
};

int p1(const Image &image) {
  int best_count = image.count_digit(0, 0);
  int best_layer = 0;
  for (int layer = 1; layer < image.num_layers(); ++layer) {
    int count = image.count_digit(layer, 0);
    if (count < best_count) {
      best_count = count;
      best_layer = layer;
    }
  }

  return image.count_digit(best_layer, 1) * image.count_digit(best_layer, 2);
}

std::string p2(const Image &image) { return image.output(); }

int main() {
  try {
    std::string line;
    std::getline(std::cin, line);

    std::vector<int> pixels;
    for (auto c : aoc::strings::trim(line)) {
      pixels.push_back(c - '0');
    }
    Image image = Image(pixels, 25, 6);

    std::cout << "Part 1: " << p1(image) << std::endl;
    std::cout << "Part 2: \n" << p2(image) << std::endl;
  } catch (const std::exception &e) {
    std::cerr << e.what() << std::endl;
    return 1;
  }
}
