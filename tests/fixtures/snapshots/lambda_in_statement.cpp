int main() {
  int x = 0;
  auto f = [&](int v) {
    if (v > 0) {
      return v;
    }
    return -v;
  };
  x = f(3);
  return x;
}
