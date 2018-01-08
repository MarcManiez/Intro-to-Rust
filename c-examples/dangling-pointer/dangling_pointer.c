#include <stdio.h>
#include <string.h>

char *dangle(void) {
  char buffer[10]; // Fixed size => goes on the satck
  char *pointer = buffer; // declares a pointer to the `buffer`.
  printf("The pointer we declared: %p\n", pointer); // Let's print the pointer to get a good look at it.
  strcpy(pointer, "world!"); // Fills the buffer with a word
  printf("Inside dangle: %s\n", pointer);
  return pointer; // returns pointer
} // dangle stack frame is dropped, bye bye `buffer`!

int main(void) {
  char *pointer = dangle(); // Retrieves pointer from dangle
  printf("The pointer we declared: %p\n", pointer); // It's the same pointer, phew!
  printf("Inside main: Hello, %s\n", pointer); // But what does it point to?
}

// Some alternate program could reuse the area of memory formerly used by buffer, and access it...
