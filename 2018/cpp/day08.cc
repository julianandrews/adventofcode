#include <algorithm>
#include <memory>
#include <numeric>
#include <string>
#include <vector>

#include "utils.h"

namespace {
using std::string;
using std::vector;
}  // namespace

class TreeNode {
 private:
  const vector<TreeNode> children;
  const vector<int> metadata;

  TreeNode(const vector<TreeNode> children, const vector<int> metadata)
      : children(children), metadata(metadata){};

  static TreeNode from_iterator_pointer(vector<int>::const_iterator *it) {
    vector<TreeNode> children;
    vector<int> metadata;
    int num_children = *(*it)++;
    int num_metadata = *(*it)++;
    children.reserve(num_children);
    metadata.reserve(num_metadata);
    for (int i = 0; i < num_children; ++i) {
      children.push_back(TreeNode::from_iterator_pointer(it));
    }
    for (int i = 0; i < num_metadata; ++i) {
      metadata.push_back(*(*it)++);
    }

    return TreeNode(children, metadata);
  }

 public:
  static TreeNode from_numbers(const vector<int> &numbers) {
    auto it = numbers.begin();
    return TreeNode::from_iterator_pointer(&it);
  }

  int metadata_sum() const {
    int total = std::accumulate(metadata.begin(), metadata.end(), 0);
    for (const auto &child : children) {
      total += child.metadata_sum();
    }

    return total;
  }

  int value() const {
    if (children.size()) {
      int total = 0;
      for (int i : metadata) {
        if (0 < i && i <= children.size()) {
          total += children[i - 1].value();
        }
      }
      return total;
    } else {
      return metadata_sum();
    }
  }
};

int p1(vector<int> numbers) {
  return TreeNode::from_numbers(numbers).metadata_sum();
}

int p2(vector<int> numbers) { return TreeNode::from_numbers(numbers).value(); }

int main() {
  vector<string> lines = aoc::utils::getlines();
  vector<string> strings = aoc::utils::split(lines[0], ' ');

  vector<int> numbers;
  std::transform(strings.begin(), strings.end(), std::back_inserter(numbers),
                 [](string &s) { return std::stoi(s); });

  std::cout << "Part 1: " << p1(numbers) << std::endl;
  std::cout << "Part 2: " << p2(numbers) << std::endl;
}
