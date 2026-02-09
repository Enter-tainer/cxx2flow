struct Runner {
  int run() {
    int x = 0;
    for (int i = 0; i < 3; i++) {
      x += i;
    }
    return x;
  }
};

int main() {
  Runner r;
  int y = r.run();
  if (y > 2) {
    y++;
  }
  return y;
}
