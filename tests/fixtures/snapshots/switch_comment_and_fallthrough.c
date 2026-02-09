int main() {
  int x = 1;
  switch (x) {
    case 0:
      x = x + 10;
      break;
    case 1:
      // fallthrough path
    case 2:
      x = x + 20;
      break;
    default:
      x = x + 30;
  }
  return x;
}
