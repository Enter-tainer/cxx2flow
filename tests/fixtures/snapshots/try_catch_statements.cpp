int may_fail(int x) {
  if (x < 0) {
    return -1;
  }
  return x;
}

int main() {
  int v = 0;
  try {
    v = may_fail(1);
  } catch (...) {
    v = 42;
  }
  return v;
}
