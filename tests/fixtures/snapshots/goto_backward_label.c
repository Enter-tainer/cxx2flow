int main() {
  int x = 0;
start:
  x++;
  if (x < 3) {
    goto start;
  }
  return x;
}
