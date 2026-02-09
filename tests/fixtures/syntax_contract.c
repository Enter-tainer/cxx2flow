int helper(int v) {
  if (v > 0) {
    return v;
  }
  return -v;
}

int main() {
  int x = 1;
  if (x == 0) {
    x = 10;
  } else if (x == 1) {
    x = 20;
  } else {
    x = 30;
  }

  while (x < 40) {
    x++;
    if (x % 3 == 0) {
      continue;
    }
    if (x > 35) {
      break;
    }
  }

  do {
    x++;
  } while (x < 42);

  for (int i = 0; i < 3; i++) {
    x += i;
  }

  switch (x) {
    case 1:
      x += 1;
      break;
    case 2:
      x += 2;
      break;
    default:
      x += 3;
  }

label_a:
  x += helper(1);
  if (x < 100) {
    goto label_a;
  }

  return x;
}
