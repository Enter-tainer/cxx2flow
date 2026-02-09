int main() {
  int x = 1;
L1:
L2:
  x++;
  if (x < 4) {
    goto L2;
  }
  return x;
}
