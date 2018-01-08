#include <stdio.h>
#include <string.h>

int main(void) {
  char destination[100] = "Yacine is back!"; // declare a buffer of 100 bytes
  char *source = " Praise be! Praise be.\n"; // declare a charater pointer (string)
  char (*pointer_to_pointer)[100] = &destination; // Create a pointer to a mutable string (pointers are mutable by default in C)
  strcat(destination, source); // concatenate strings
  printf("Dereferencing pointer to string: %s\n", *pointer_to_pointer);
}

// This behavior is prohibited in Rust. Pointers can be mutable, but if they are there may be only one pointer to the mutable object.
// This is to prevent data races, which result from the fact that if a pointer is written to and read from at the same moment, there is no easy way to reconcile those behaviors in a predictable way.
