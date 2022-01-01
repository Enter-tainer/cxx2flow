#include <stdio.h>
int char_count, word_count, line_count = 1;
char ch;
int state; // the global state of the finite state machine
enum { kchn, keng, kendl, kspc }; // four states

char fill_ch() {
  // read a char from stdin
  ++char_count;
  ch = getchar();
  return ch;
}

int is_chn(char s) { return s & 0x80; } // return true if s is a chinese char

int is_spc(char s) {
  return s == ' ' || s == '\n' || s == '\t';
} // return true if s is space

int is_newline(char s) { return s == '\n'; } // return true if s is a newline

int is_eng(char s) {
  return ('0' <= s && s <= '9' || 'a' <= s && s <= 'z' ||
          'A' <= s && s <= 'Z'); // return true if s is number or alpha
}

int is_eof(char s) { return s == EOF; } // return true if s is EOF

char peek_next() {
  // first get a char from stdin then put it back
  // so as to peek the next char in stdin
  ch = getchar();
  ungetc(ch, stdin);
  return ch;
}

void set_state() {
  // set global state after peeking the next char
  char cc = peek_next();
  if (is_eng(cc))
    state = keng;
  else if (is_newline(cc))
    state = kendl;
  else if (is_spc(cc))
    state = kspc;
  else if (is_chn(cc))
    state = kchn;
}

int main() {
  set_state();
  while (ch != EOF) {
    // if ch is not EOF, keep running the FSM
    switch (state) {
    case kchn: {
      fill_ch();
      fill_ch();
      // in this case, I assume that Chinese char is encoded with GB2312.
      // if it is encoded with UTF-8, you need to add an additional fill_ch()
      ++word_count;
      --char_count;
      // two char can represent a chinese char, if it is UTF-8, it should be
      // char_cout -= 2
      set_state();
      break;
    }
    case keng: {
      while (is_eng(ch)) {
        fill_ch();
      }
      ungetc(ch, stdin);
      --char_count;
      ++word_count;
      set_state();
      break;
    }
    case kendl: {
      ++line_count;
      --char_count;
      fill_ch();
      set_state();
      break;
    }
    case kspc: {
      fill_ch();
      --char_count;
      set_state();
      break;
    }
    }
  }
  printf("%d lines, %d chars, %d words", line_count, char_count, word_count);
}
