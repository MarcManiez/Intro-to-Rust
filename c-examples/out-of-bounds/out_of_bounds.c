#include <stdio.h>
#include <string.h>

int main(void) {
  char *devops = "devops class is so dope"; // declare a character pointer
  printf("Some character inside the devops character pointer: %c\n", devops[22]); // last character "\n" nonwithstanding
  printf("Some piece of memory outside of the devops character pointer: %c\n", devops[24]); // accesses an area of memory that is not part of the string we initialized! (in fact, it's the first character of the following line's string...)
};
