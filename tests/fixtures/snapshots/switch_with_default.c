int main() {
  int v = 2;
  switch (v) {
    case 1:
      v += 10;
      break;
    case 2:
    case 3:
      v += 20;
      break;
    default:
      v += 30;
  }
  return v;
}
