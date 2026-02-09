int main() {
  int x = -1;
  switch (x) {
    case -1:
      x = 10;
      break;
    case 0:
      x = 20;
      break;
    default:
      x = 30;
  }
  return x;
}
