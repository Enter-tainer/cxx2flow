int main() {
  DQ::init();
  scanf("%d%d%d", &I, &m, &p);
  for (int i = 1; i <= m; i++) {
    char op[5];
    int x, y;
    scanf("%s%d%d", op, &x, &y);
    if (op[0] == 'I' && op[1] == 'F')
      DQ::push_front(make_pair(_(x), y));
    else if (op[0] == 'I' && op[1] == 'G')
      DQ::push_back(make_pair(_(x), y));
    else if (op[0] == 'D' && op[1] == 'F')
      DQ::pop_front();
    else if (op[0] == 'D' && op[1] == 'G')
      DQ::pop_back();
    else
      printf("%d\n", DQ::query(x, y));
  }
  return 0;
}