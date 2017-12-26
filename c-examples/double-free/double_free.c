#include <stdlib.h>

void double_free() {
  char *character;
  character = (char *)malloc(10);
  free(character);
  free(character); // This is a bad idea, but gcc lets us do it.
}

int main(void) {
  double_free(); // surfaces an error, but only at runtime
}