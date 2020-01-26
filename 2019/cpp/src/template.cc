#include <iostream>
#include <stdexcept>

#include "strings.h"

int p1() { return 0; }
int p2() { return 0; }

int main() {
  try {
    std::cout << "Part 1: " << p1() << std::endl;
    std::cout << "Part 2: " << p2() << std::endl;
  } catch (const std::exception &e) {
    std::cerr << e.what() << std::endl;
    return 1;
  }
}
