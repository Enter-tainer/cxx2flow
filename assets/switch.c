// decide if a number is a phone number

#include <stdio.h>
#include <string.h>

int main() {
  char str[100];
  scanf("%s", str);
  int len = strlen(str);
  if (len == 11 && str[0] == '1') {
    switch (str[1]) {
    case '3':
    case '4':
    case '5':
    case '7':
    case '8':
      printf("is a phone number");
      break;

    default:
      printf("not a phone number");
      break;
    }
  } else {
    printf("not a phone number");
  }
}
