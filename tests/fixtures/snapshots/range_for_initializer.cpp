struct Vec {
  int* begin();
  int* end();
};

int main() {
  int sum = 0;
  for (auto n : Vec{}) {
    sum += n;
  }
  return sum;
}
