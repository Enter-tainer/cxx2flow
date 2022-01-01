#include <stdio.h>
#include <stdlib.h>
typedef struct node node;
typedef struct single_list single_list;

struct node {
  void* data;
  node* next;
};

struct single_list {
  node* head;
  int size;
};

single_list init_list() {
  single_list x;
  x.size       = 0;
  x.head       = malloc(sizeof(node));
  x.head->next = x.head->data = NULL;
  return x;
}

single_list push_back_list(single_list x, void* data) {
  node* t = x.head;
  while (t->next)
    t = t->next;
  t->next       = malloc(sizeof(node));
  t->next->next = NULL;
  t->next->data = data;
  x.size++;
  return x;
}

single_list push_front_list(single_list x, void* data) {
  node* t = x.head;
  node* n = malloc(sizeof(node));
  n->data = data;
  n->next = t->next;
  t->next = n;
  x.size++;
  return x;
}

single_list insert_list(single_list x, void* data, int pos) {
  // after insert, data stays in the pos postion
  node* t = x.head;
  for (int i = 0; i < pos && t->next; ++i, t = t->next)
    ;
  node* n = malloc(sizeof(node));
  n->data = data;
  n->next = t->next;
  t->next = n;
  x.size++;
  return x;
}

void* in_access_list(single_list x, int pos) {
  node* t = x.head;
  for (int i = 0; i <= pos && t->next; ++i, t = t->next)
    ;
  return t->data;
}

#define access_list(type) (type*)in_access_list

int search_list(single_list x, void* data,
                int (*eq)(const void* a, const void* b)) {
  node* t = x.head->next;
  for (int i = 0; t->next; ++i, t = t->next)
    if (eq(t->data, data))
      return i;
  return -1;
}

#define from_heap(type)                                                        \
  type* from_heap_##type(type x) {                                             \
    type* t = malloc(sizeof(type));                                            \
    *t      = x;                                                               \
    return t;                                                                  \
  }

single_list delete_node(single_list x, int pos) {
  node* t = x.head;
  for (int i = 0; i < pos && t->next; ++i, t = t->next)
    ;
  node* d = t->next;
  t->next = d->next;
  free(d->data);
  d->next = d->data = NULL;
  free(d);
  x.size--;
  return x;
}

single_list pop_front_list(single_list x) { return delete_node(x, 0); }

single_list pop_back_list(single_list x) { return delete_node(x, x.size - 1); }

void drop_nodes(node* x) {
  if (x->next) {
    drop_nodes(x->next);
  }
  free(x->data);
  x->data = x->next = NULL;
  free(x);
}

void drop_list(single_list x) { drop_nodes(x.head); }

int iseq(const int* a, const int* b) { return *a == *b; }

from_heap(int);

typedef struct student {
  int id;
  char name[16];
  int eng;
  int mat;
  int phy;
  int cpl;
} student;

int get_choice() {
  printf("\t\t1. Input data\n");
  printf("\t\t2. Ouput data\n");
  printf("\t\t3. Modify data of someone\n");
  printf("\t\t4. Ouput avg mark\n");
  printf("\t\t5. Output data & avg mark & sum\n");
  printf("\t\t6. Sort\n");
  printf("\t\t7. Exit\n");
  int c;
  scanf("%d", &c);
  return c;
}

int isleq(const student* a, const student* b) {
  int sum1, sum2;
  sum1 = a->cpl + a->eng + a->phy + a->mat;
  sum2 = b->cpl + b->eng + b->phy + b->mat;
  return sum1 <= sum2;
}

node *sort_node(node *s) {
  if (s == NULL || s->next == NULL)
    return s;
  node* head, *leq = NULL, *g = NULL;
  node* p = s->next;
  head    = s;
  s->next = NULL;
  for (; p; ) {
    node* t      = p->next;
    student* stu = p->data, *piv = head->data;
    if (isleq(stu, piv)) {
      p->next = leq;
      leq     = p;
    } else {
      p->next = g;
      g       = p;
    }
    p = t;
  }
  leq = sort_node(leq);
  g   = sort_node(g);
  node* res = head;
  if (leq) {
    res = leq;
    p = leq;
    while (p->next)
      p = p->next;
    p->next = head;
  }
  head->next = g;
  return res;
}

single_list sort_list(single_list x) {
  x.head->next = sort_node(x.head->next);
  return x;
}

int main() {
  single_list s = init_list();
  int choice    = 0;
  while ((choice = get_choice()) != 7) {
    if (choice == 1) {
      printf("Input n and <id> <name> <eng> <mat> <phy> <cpl>, one student per "
             "line\n");
      int n;
      scanf("%d", &n);
      while (n--) {
        student* stu = malloc(sizeof(student));
        scanf("%d %s %d %d %d %d", &stu->id, &stu->name, &stu->eng, &stu->mat,
              &stu->phy, &stu->cpl);
        s = push_back_list(s, stu);
      }

    } else if (choice == 2) {
      int idx = 0;
      for (node* i = s.head->next; i; i = i->next, idx++) {
        student* stu = i->data;
        printf("%d: %d %s %d %d %d %d\n", idx, stu->id, stu->name, stu->eng,
               stu->mat, stu->phy, stu->cpl);
      }
    } else if (choice == 3) {
      printf("Input student idx and <id> <name> <eng> <mat> <phy> <cpl>\n");
      student stu, *tbmdf;
      int idx;
      scanf("%d %d %s %d %d %d %d", &idx, &stu.id, &stu.name, &stu.eng,
            &stu.mat, &stu.phy, &stu.cpl);
      tbmdf  = access_list(student)(s, idx);
      *tbmdf = stu;
    } else if (choice == 4) {
      double sum = 0;
      for (node* i = s.head->next; i; i = i->next) {
        student* stu = i->data;
        sum += stu->cpl + stu->mat + stu->eng + stu->phy;
      }
      printf("avg: %.2lf\n", sum / 4.0 / s.size);
    } else if (choice == 5) {
      for (node* i = s.head->next; i; i = i->next) {
        double sum   = 0;
        student* stu = i->data;
        sum          = stu->cpl + stu->mat + stu->eng + stu->phy;
        printf("%d %s sum: %lf avg: %lf\n", stu->id, stu->name, sum, sum / 4);
      }
    } else if (choice == 6) {
      s = sort_list(s);
    }
  }
  drop_list(s);
}

mint mul(const mint* a, const mint* b) {
  const mint* c = a;
  if (a->len > b->len)
    a = b, b = c;
  mint t;
  memset(&t, 0, sizeof(t));
  t.len = 1;
  for (int i = 0; i < a->len; i++) {
    for (int j = 0; j < b->len; ++j) {
      t.d[i + j] += a->d[i] * b->d[j];
    }
    for (int j = 0; j < t.len; ++j) {
      if (t.d[j] >= 10) {
        int c = t.d[j] / 10;
        t.d[j] %= 10;
        t.d[j + 1] += c;
        if (j == t.len - 1)
          ++t.len;
      }
    }
  }
  for (int j = 0; j < t.len; ++j) {
    if (t.d[j] >= 10) {
      int c = t.d[j] / 10;
      t.d[j] %= 10;
      t.d[j + 1] += c;
      if (j == t.len - 1)
        ++t.len;
    }
  }
  for (int i = 499; i >= 1; --i) {
    if (t.d[i]) {
      t.len = 1 + i;
      break;
    }
  }
  return t;
}

token_result_t parse_num(sds source_code, int ptr, int line, int column, int *skips) {
    int v_int = 0;
    double v_double = 0;
    bool is_double = false, is_long_lit = false;
    int len = sdslen(source_code);
    int i = ptr, double_cnt = -1;
    int base = 10;
    if (source_code[i] == '0') {
        if (i + 1 < len) {
            if (tolower(source_code[i + 1]) == 'x') {
                base = 16;
                i += 2;
            } else if (source_code[i + 1] == '.' || is_valid_symbol_to_end_identifier(source_code[i + 1])) {

            } else if (isdigit(source_code[i + 1])) {
                base = 8;
                i++;
            } else {
                sds msg = sdscatprintf(sdsempty() ,"expecting hex or oct lit, but get '%c'", source_code[i + 1]);
                return err_token(msg);
            }
        }
    }

    for (; i < len; ++i) {
        bool valid = false;
        if (base == 10) {
            valid = isdigit(source_code[i]);
        } else if (base == 8) {
            valid = isoctdigit(source_code[i]);
        } else {
            valid = ishexdigit(source_code[i]);
        }
        if (valid) {
            if (is_double) {
                v_double += pow(base, double_cnt--) * (char2int(source_code[i]));
            } else {
                v_int *= base;
                v_int += char2int(source_code[i]);
            }
        } else {
            if (source_code[i] == '.') {
                if (base == 8 || base == 16) {
                    sds msg = sdscatprintf(sdsempty(), "float in oct and hex is not supported");
                    return err_token(msg);
                }
                is_double = true;
                v_double = v_int;
            } else if (isblank(source_code[i]) || source_code[i] == 0 || is_valid_symbol_to_end_identifier(source_code[i])) {
                break;
            } else if (source_code[i] == 'L') {
                if (is_double) {
                    sds msg = sdscatprintf(sdsempty(), "unexpected 'L'");
                } else {
                    is_long_lit = true;
                }
            } else {
                sds err_msg = sdscatprintf(sdsempty(), "unexpected '%c', at line %d, column %d", source_code[i], line,
                                           column);
                *skips = i - ptr;
                return err_token(err_msg);
            }
        }
    }
    *skips = i - ptr;
    if (is_double) {
        struct Token res = {
                .column_num = column,
                .line_num = line,
                .type = FloatLit,
                .v.f = v_double,
        };
        return ok_token(res);
    } else {
        struct Token res = {
                .column_num = column,
                .line_num = line,
                .type = is_long_lit ? LongLit: IntLit,
                .v.v = v_int,
        };
        return ok_token(res);
    }
}
