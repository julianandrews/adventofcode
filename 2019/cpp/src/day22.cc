#include <cassert>
#include <cstdlib>
#include <iostream>
#include <iterator>
#include <stdexcept>
#include <string>
#include <vector>

#include "modint.h"
#include "strings.h"

using ::aoc::modint::ModInt;

template <unsigned long size> class Deck {
public:
  void cut(unsigned long n) { offset_ += stride_ * n; }

  void increment(unsigned long n) {
    stride_ *= ModInt<size>(n).inverse().value();
  }

  void deal_new_stack() {
    increment(size - 1);
    cut(1);
  }

  void shuffle(const std::vector<std::string> &moves) {
    for (const auto &move : moves) {
      if (move.find("deal with increment") == 0) {
        unsigned long n = std::stoll(move.substr(move.rfind(" ")));
        increment(n);
      } else if (move.find("deal into new stack") == 0) {
        deal_new_stack();
      } else if (move.find("cut") == 0) {
        long n = std::stoll(move.substr(move.rfind(" ")));
        // It's easier if we never have to deal with negative values
        while (n < 0) {
          n += size;
        }
        cut(n);
      }
    }
  }

  bool operator==(const Deck<size> &rhs) {
    return stride_ == rhs.stride_ && offset_ == rhs.offset_;
  }

  Deck &operator*=(unsigned long n) {
    Deck<size> one_shuffle = *this;
    stride_ = 1;
    offset_ = 0;

    while (n) {
      unsigned long count = 1;
      Deck<size> new_deck = one_shuffle;
      while (count <= n / 2) {
        new_deck += new_deck;
        count *= 2;
      }
      operator+=(new_deck);
      n -= count;
    }
    return *this;
  }

  friend Deck<size> operator*(Deck<size> lhs, unsigned long rhs) {
    lhs *= rhs;
    return lhs;
  }

  Deck<size> &operator+=(const Deck<size> &rhs) {
    cut((unsigned long)rhs.offset_);
    stride_ *= rhs.stride_;
    return *this;
  }

  friend Deck<size> operator+(Deck<size> lhs, const Deck<size> &rhs) {
    lhs += rhs;
    return lhs;
  }

  friend std::ostream &operator<<(std::ostream &os, const Deck<size> &deck) {
    os << "Deck<" << size << ">(" << deck.offset_ << ", " << deck.stride_
       << ")";

    return os;
  }

  class iterator {
  public:
    using iterator_category = std::input_iterator_tag;
    using value_type = ModInt<size>;
    using reference = ModInt<size>;
    using diference_type = void;
    using pointer = void;

    iterator(unsigned long i, ModInt<size> stride, ModInt<size> offset)
        : i_(i), stride_(stride), offset_(offset) {}

    iterator &operator++() {
      ++i_;
      return *this;
    }

    bool operator==(const iterator &other) const {
      return i_ == other.i_ && offset_ == other.offset_ &&
             stride_ == other.stride_;
    }

    bool operator!=(const iterator &other) const { return !operator==(other); }

    reference operator*() { return offset_ + stride_ * ModInt<size>(i_); }

  private:
    unsigned long i_;
    ModInt<size> stride_;
    ModInt<size> offset_;
  };

  iterator begin() const { return iterator(0, stride_, offset_); }

  iterator end() const { return iterator(size, stride_, offset_); }

private:
  ModInt<size> stride_ = 1;
  ModInt<size> offset_ = 0;
};

void run_tests() {
  {
    Deck<10> deck;
    deck.shuffle({"deal with increment 9"});
    std::vector<ModInt<10>> result;
    for (auto x : deck) {
      result.push_back(x);
    }
    assert(result == std::vector<ModInt<10>>({0, 9, 8, 7, 6, 5, 4, 3, 2, 1}));
  }

  {
    Deck<10> deck;
    deck.shuffle({
        "deal with increment 7",
        "deal into new stack",
        "deal into new stack",
    });
    std::vector<ModInt<10>> result;
    for (auto x : deck) {
      result.push_back(x);
    }
    assert(result == std::vector<ModInt<10>>({0, 3, 6, 9, 2, 5, 8, 1, 4, 7}));
  }

  {
    Deck<10> deck;
    deck.shuffle({
        "cut 6",
        "deal with increment 7",
        "deal into new stack",
    });
    std::vector<ModInt<10>> result;
    for (auto x : deck) {
      result.push_back(x);
    }
    assert(result == std::vector<ModInt<10>>({3, 0, 7, 4, 1, 8, 5, 2, 9, 6}));
  }

  {
    Deck<10> deck;
    deck.shuffle({
        "deal with increment 7",
        "deal with increment 9",
        "cut -2",
    });
    std::vector<ModInt<10>> result;
    for (auto x : deck) {
      result.push_back(x);
    }
    assert(result == std::vector<ModInt<10>>({6, 3, 0, 7, 4, 1, 8, 5, 2, 9}));
  }

  {
    Deck<10> deck;
    deck.shuffle({
        "deal into new stack",
        "cut -2",
        "deal with increment 7",
        "cut 8",
        "cut -4",
        "deal with increment 7",
        "cut 3",
        "deal with increment 9",
        "deal with increment 3",
        "cut -1",
    });
    std::vector<ModInt<10>> result;
    for (auto x : deck) {
      result.push_back(x);
    }
    assert(result == std::vector<ModInt<10>>({9, 2, 5, 8, 1, 4, 7, 0, 3, 6}));
  }

  {
    std::vector<std::string> moves1 = {
        "deal with increment 7",
        "deal into new stack",
        "cut -2",
    };
    std::vector<std::string> moves2 = {
        "cut 8",
        "cut 3",
        "deal with increment 9",
    };
    Deck<10> deck1;
    deck1.shuffle(moves1);
    Deck<10> deck2;
    deck2.shuffle(moves2);
    Deck<10> expected;
    expected.shuffle(moves1);
    expected.shuffle(moves2);
    assert(deck1 + deck2 == expected);
  }
  {
    std::vector<std::string> moves = {
        "deal into new stack",
        "cut -2",
        "deal with increment 7",
        "cut 8",
        "cut -4",
        "deal with increment 7",
        "cut 3",
        "deal with increment 9",
        "deal with increment 3",
        "cut -1",
    };
    Deck<10> deck;
    deck.shuffle(moves);
    Deck<10> expected;
    for (int i = 0; i < 20; ++i) {
      assert(deck * i == expected);
      expected.shuffle(moves);
    }
  }
}

unsigned long p1(const std::vector<std::string> &moves) {
  Deck<10007> deck;
  deck.shuffle(moves);
  unsigned long i = 0;
  for (auto card : deck) {
    if (card == 2019) {
      return i;
    }
    ++i;
  }
  throw std::runtime_error("Deck value not found");
}

unsigned long p2(const std::vector<std::string> &moves) {
  Deck<119315717514047> deck;
  deck.shuffle(moves);
  deck *= 101741582076661;
  unsigned long i = 0;
  for (auto card : deck) {
    if (i == 2020) {
      return (unsigned long)card;
    }
    ++i;
  }
  throw std::runtime_error("Deck value not found");
}

int main() {
  try {
    run_tests();
    std::cout << "All tests passed!" << std::endl;

    std::vector<std::string> lines = aoc::strings::getlines();

    std::cout << "Part 1: " << p1(lines) << std::endl;
    std::cout << "Part 2: " << p2(lines) << std::endl;
  } catch (const std::exception &e) {
    std::cerr << e.what() << std::endl;
    return 1;
  }
}
