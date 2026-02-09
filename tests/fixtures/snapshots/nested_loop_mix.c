int main() {
  int i = 0;
  while (i < 3) {
    int j = 0;
    do {
      if (j == 1) {
        j++;
        continue;
      }
      if (i == 2 && j == 2) {
        break;
      }
      j++;
    } while (j < 4);
    i++;
  }
  return i;
}
