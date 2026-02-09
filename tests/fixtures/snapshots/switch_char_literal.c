int main() {
  int code = 'b';
  switch (code) {
    case 'a':
      code = 1;
      break;
    case 'b':
      code = 2;
      break;
    default:
      code = 3;
  }
  return code;
}
