int main() {
  int t;
  double ans, x;
  printf("Please input your salary:");
  scanf("%lf", &x);
  printf("Please choose a method\n1:with if; 2:with swtch:");
  scanf("%d", &t);
  if (t == 1)
    ans = useif(x);
  else if (t == 2)
    ans = usesw(x);
  else {
    printf("Invalid input!\n");
    return 0;
  };
  if (t == 1)
    printf("calculate by if, the tax is:%.2lf\n", ans);
  else
    printf("calculate by switch, the tax is:%.2lf\n", ans);
  return 0;
}
