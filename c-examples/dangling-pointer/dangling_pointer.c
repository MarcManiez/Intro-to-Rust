#include <stdio.h>
#include <string.h>

int main(void) {
  char *pointer = func(); // Retrieves pointer from func
  printf("The pointer we declared: %p\n", pointer); // It's the same pointer, phew!
  printf("Inside main: Hello, %s\n", pointer); // But what does it point to?
}

char *func() {
  char buffer[10]; // Fixed size => goes on the satck
  char *pointer = buffer; // declares a pointer to the buffer
  printf("The pointer we declared: %p\n", pointer);
  strcpy(pointer, "world!"); // Fills the buffer with a word
  printf("Inside func: %s\n", pointer);
  return (pointer); // returns pointer
} // func stack frame is dropped, bye bye buffer!

// Some alternate program could reuse the area of memory formerly used by buffer, and our program could access it...