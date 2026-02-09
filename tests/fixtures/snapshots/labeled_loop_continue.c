int main() {
  int i = 0;
loop_start:
  i++;
  while (i < 3) {
    i++;
    continue;
  }
  if (i < 5) {
    goto loop_start;
  }
  return i;
}
