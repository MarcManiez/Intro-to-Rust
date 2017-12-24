#include <stdlib.h>

void leak() {
  char *character;
  character = (char *)malloc(10); // malloc tells gives us 10 bytes on the heap.
} // oops, we forgot to use free

void no_leak() {
  char *character;
  character = (char *)malloc(10);
  free(character); // we've learned from our mistake, 5 lines ago.
}

int main(void) {
  leak();
  no_leak();
}