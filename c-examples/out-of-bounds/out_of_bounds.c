#include <stdio.h>
#include <string.h>

int main(void) {
  char *devops = "devops class is so dope";
  printf("Some character inside the devops character pointer: %c\n", devops[19]);
  printf("Some piece of memory outside of the devops character pointer: %c\n", devops[24]);
};
