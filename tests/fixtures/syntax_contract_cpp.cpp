namespace ns {
template <typename T>
T pick(T a, T b) {
  if (a > b) {
    return a;
  }
  return b;
}

class C {
 public:
  int run() {
    int x = 0;
    for (int i = 0; i < 3; i++) {
      x += i;
    }
    return x;
  }
};
}  // namespace ns

enum class State { Init, Work, Done };

struct Vec {
  int* begin();
  int* end();
};

int main() {
  int x = 0;
  auto f = [&](int v) {
    if (v > 0) {
      return v;
    }
    return -v;
  };
  x = f(1);

  for (auto n : Vec{}) {
    x += n;
  }

  State state = State::Work;
  switch (state) {
    case State::Init:
      x += 1;
      break;
    case State::Work:
      x += 2;
      break;
    default:
      x += 3;
  }

  try {
    x += ns::pick(1, 2);
  } catch (...) {
    x = 0;
  }

  return x;
}
