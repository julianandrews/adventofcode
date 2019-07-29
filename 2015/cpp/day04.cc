#include <openssl/md5.h>
#include <sstream>

#include "utils.h"

int first_with_leading_zeros(const std::string &secret,
                             const int leading_zeros) {
  if (leading_zeros < 0 ||
      (std::size_t)leading_zeros > sizeof(unsigned long) * 2) {
    throw std::invalid_argument("Too many leading zeros");
  }
  unsigned char digest[MD5_DIGEST_LENGTH];

  for (int i = 0;; ++i) {
    std::string input = secret + std::to_string(i);
    MD5((unsigned char *)input.c_str(), input.size(), (unsigned char *)&digest);

    bool success = true;
    for (int j = 0; j < leading_zeros / 2; ++j) {
      if (digest[j] != 0) {
        success = false;
        break;
      }
    }
    if (leading_zeros % 2 != 0 && (digest[leading_zeros / 2] & 240) != 0) {
      success = false;
    }

    if (success) {
      return i;
    }
  }
}

int p1(const std::string &secret) {
  return first_with_leading_zeros(secret, 5);
}

int p2(const std::string &secret) {
  return first_with_leading_zeros(secret, 6);
}

int main() {
  std::string secret = aoc::utils::get_trimmed_line();

  try {
    std::cout << "Part 1: " << p1(secret) << std::endl;
    std::cout << "Part 2: " << p2(secret) << std::endl;
  } catch (const std::exception &exception) {
    std::cerr << exception.what() << std::endl;
    return 1;
  }
}
