enum class State { Init = 0, Work = 1, Done = 2 };

int main() {
  State state = State::Work;
  int v = 0;
  switch (state) {
    case State::Init:
      v = 1;
      break;
    case State::Work:
      v = 2;
      break;
    default:
      v = 3;
  }
  return v;
}
