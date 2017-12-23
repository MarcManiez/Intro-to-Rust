#include <stdlib.h>

void leak() {
  char *ch;
  ch = (char *)malloc(10);
}

int main(void) {
  leak();
}