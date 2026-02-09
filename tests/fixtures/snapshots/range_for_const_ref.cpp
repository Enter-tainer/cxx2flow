struct Vec {
  int* begin();
  int* end();
};

int main() {
  Vec nums;
  int sum = 0;
  for (const auto& n : nums) {
    sum += n;
  }
  return sum;
}
