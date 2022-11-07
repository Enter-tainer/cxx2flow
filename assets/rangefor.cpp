void basic() {
    for (auto i : vec) {
        i = i + 1;
    }
}

void auto_ref() {
    for (auto & i : map) {
        a;
    }
}

void const_auto_rvref() {
    for (const auto &&i : vec) {
        i - 1;
    }
}

void auto_destruct() {
    for (auto [x, y] : vec) {
        x + y;
    }
}

void int_init() {
    for (int i : vec) {
        i;
    }
}