#include <stdio.h>
#include <stdlib.h>

#if 1
int main() {
  int *r;
  {
    int x = 5;
    r = &x;
  }
  printf("r: %d\n", *r); // Undefined behaviour
  return 0;
}

#else

int main() {
  int *r;
  {
    int *x = malloc(sizeof(int));
    *x = 5;
    r = x;
    free(x);
  }

  printf("r: %d\n", *r); // Undefined behaviour
  return 0;
}

#endif
