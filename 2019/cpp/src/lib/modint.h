#ifndef AOC_MODINT_H
#define AOC_MODINT_H

#include <cstdint>
#include <iostream>
#include <optional>
#include <type_traits>

namespace aoc {
namespace modint {

template <unsigned long m> class ModInt {
public:
  ModInt(unsigned long value) : value_(value % m) {}

  explicit operator unsigned long() const { return value_; }

  friend bool operator==(const ModInt<m> &lhs, const ModInt<m> &rhs) {
    return lhs.value_ == rhs.value_;
  }

  friend bool operator<(const ModInt<m> &lhs, const ModInt<m> &rhs) {
    return lhs.value_ < rhs.value_;
  }

  friend bool operator!=(const ModInt<m> &lhs, const ModInt<m> &rhs) {
    return !operator==(lhs, rhs);
  }

  friend bool operator>(const ModInt<m> &lhs, const ModInt<m> &rhs) {
    return operator<(rhs, lhs);
  }

  friend bool operator<=(const ModInt<m> &lhs, const ModInt<m> &rhs) {
    return !operator>(lhs, rhs);
  }

  friend bool operator>=(const ModInt<m> &lhs, const ModInt<m> &rhs) {
    return !operator<(lhs, rhs);
  }

  ModInt<m> &operator++() {
    if (value_ >= m - 1) {
      value_ -= m;
    }
    ++value_;
    return *this;
  }

  ModInt<m> &operator--() {
    if (value_ == 0) {
      value_ = m;
    }
    --value_;
    return *this;
  }

  ModInt<m> operator++(int) {
    ModInt<m> temp(value_);
    operator++();
    return temp;
  }

  ModInt<m> operator--(int) {
    ModInt<m> temp(value_);
    operator--();
    return temp;
  }

  ModInt<m> &operator+=(const ModInt<m> &rhs) {
    if (value_ >= m - rhs.value_) {
      value_ -= m;
    }
    value_ += rhs.value_;
    return *this;
  };

  friend ModInt<m> operator+(ModInt<m> lhs, const ModInt<m> &rhs) {
    lhs += rhs;
    return lhs;
  }

  ModInt<m> &operator*=(const ModInt<m> &rhs) {
    unsigned long a = value_;
    unsigned long b = rhs.value_;
    value_ = 0;

    while (a != 0) {
      if (a & 1) {
        if (b >= m - value_) {
          value_ -= m;
        }
        value_ += b;
      }
      a >>= 1;

      unsigned long temp_b = b;
      if (b >= m - b) {
        temp_b -= m;
      }
      b += temp_b;
    }

    return *this;
  }

  friend ModInt<m> operator*(ModInt<m> lhs, const ModInt<m> &rhs) {
    lhs *= rhs;
    return lhs;
  }

  friend std::ostream &operator<<(std::ostream &os, const ModInt<m> &value) {
    os << value.value_;
    return os;
  }

  std::optional<ModInt<m>> inverse() {
    struct unsignedLongWithSign {
      unsigned long value;
      bool isNegative;
    };

    unsigned long a = value_;
    unsigned long b = m;
    unsignedLongWithSign x0 = {0, false};
    unsignedLongWithSign x1 = {1, false};

    while (a > 1) {
      if (b == 0) {
        return std::nullopt;
      }
      unsigned long q = a / b;

      // (b, a) := (a % b, b)
      unsigned long t = b;
      b = a % b;
      a = t;

      // (x0, x1) := (x1 - q * x0, x0)
      unsignedLongWithSign t2 = x0;
      unsigned long qx0 = q * x0.value;
      if (x0.isNegative != x1.isNegative) {
        x0.value = x1.value + qx0;
        x0.isNegative = x1.isNegative;
      } else {
        x0.value = (x1.value > qx0) ? x1.value - qx0 : qx0 - x1.value;
        x0.isNegative = (x1.value > qx0) ? x1.isNegative : !x0.isNegative;
      }
      x1 = t2;
    }

    return ModInt<m>(x1.isNegative ? (m - x1.value) : x1.value);
  }

private:
  unsigned long value_ = 0;
};

} // namespace modint
} // namespace aoc

#endif
