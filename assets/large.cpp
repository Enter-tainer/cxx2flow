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