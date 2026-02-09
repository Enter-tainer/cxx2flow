int main() {
  int x = 0;
  goto done;
  x = 99;
done:
  x++;
  return x;
}
