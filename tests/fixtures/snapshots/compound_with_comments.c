int main() {
  int x = 0;
  {
    // comments should be ignored by parser paths
    x = x + 1;
  }
  return x;
}
