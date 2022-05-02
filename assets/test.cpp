#include <bits/stdc++.h>
using namespace std;
const int logn = 21;
const int maxn = 2000001;
int f[maxn][logn + 1], Logn[maxn + 1];
inline int read() {  //快读
  char c = getchar();
  int x = 0, f = 1;
  while (c < '0' || c > '9') {
    if (c == '-') f = -1;
    c = getchar();
  }
  while (c >= '0' && c <= '9') {
    x = x * 10 + c - '0';
    c = getchar();
  }
  return x * f;
}
void pre() {  //准备工作，初始化
  Logn[1] = 0;
  Logn[2] = 1;
  for (int i = 3; i < maxn; i++) {
    Logn[i] = Logn[i / 2] + 1;
  }
}

void test0() {
  while (c) {
    d;
    if (e) {
      continue;
    }
  }
}

void test1 {
  while (a) {
    while (b) {
      if (f) {
        break;
      }
    }
  }
}

void test2() {
  if (a) {
    while (b) {
      e;
    }
  } else {
    f;
  }
}

void test3() {
  while (a) {
    while (b) {
      for (c; d; e) {
        f;
        if (g) {
          break;
        }
      }
    }
  }
}
void test4() {
  while (g) {
    if (h) {
      if(i) {
        if (j) {

        } else {
          if (k) {
            while (l) {
              wtf;
            }
          }
        }
      }
    } else {
      n;
    }
  }
  o;
}
void test5() {
  while (p) {
    if (q) {
      r;
    } else {
      s;
    }
  }

}

void test6() {
  if (c) {
  } else 
    for (r; s; t) {
    u;
    if (a) {
      break;
    }
    if (v) {
      continue;
    }
  }
}

void test7() {
  if (a)
    b;
  else if (c)
    d;
  else while (e)
    for (f; g; h)
      do 
        if (i)
          continue;
      while(j);
}

void test8() {
l1 : l2: switch (c) {
    case 0:
    break;
    case 1:
    default:
      d;
  }
}

void test9() {
  switch (c) {
  case 0:
  case 1:
    a + b;
    break;
  case 2:
  case 3:
    c + d;
  case 4:
    switch (f) {
    case 1:
      l;
      break;
    case 7:
      t;
    }
    break;
  default:
    e;
  }
}

void test10() {
  switch (a) {
  case 1:
    switch (b) {
      case 1:
      break;
      default:
        break;
      }
      break;
  default:
    break;
  }
}

void test11() {
  struct grades_list list = {NULL, NULL, 0};
  struct grades *g;
  int is_exit = 0;
  while (is_exit == 0) {
    int choice, num, i, sub_choice;
    char ID[16];
    scanf("%d", &choice);
    switch (choice) {
    case 0:
      clean_info(&list);
      is_exit = 1;
      break;
    case 1: /* 输入 */
      scanf("%d", &num);
      for (i = 0; i < num; ++i) {
        get_grades(&list);
      }
      sort_grades(&list);
      break;
    case 2: /* 输出 */
      for_each(&list, print_basic_info);
      break;
    case 3: /* 修改 */
      scanf("%s%d", ID, &sub_choice);
      g = find_grades(&list, ID);
      if (g == NULL)
        break;
      switch (sub_choice) {
      case 1:
        scanf("%d", &(g->english));
        break;
      case 2:
        scanf("%d", &(g->math));
        break;
      case 3:
        scanf("%d", &(g->physics));
        break;
      case 4:
        scanf("%d", &(g->c_lang));
        break;
      default:
        break;
      }
      count_grades(g);
      break;
    case 4: /* 统计平均 */
      for_each(&list, print_average);
      break;
    case 5: /* 输出总成绩及平均成绩 */
      for_each(&list, print_sum_and_average);
      break;
    default:
      break;
    }
  }
  return 0;
}

void incomplete_switch() {
  // should error
  switch (c) { default }
}

int main() {
  int n = read(), m = read();
  for (int i = 1; i <= n; i++) f[i][0] = read();
  pre();
  for (int j = 1; j <= logn; j++)
    for (int i = 1; i + (1 << j) - 1 <= n; i++)
      f[i][j] = max(f[i][j - 1], f[i + (1 << (j - 1))][j - 1]);  // ST表具体实现
  for (int i = 1; i <= m; i++) {
    int x = read(), y = read();
    int s = Logn[y - x + 1];
    printf("%d\n", max(f[x][s], f[y - (1 << s) + 1][s]));
  }
  return 0;
}
