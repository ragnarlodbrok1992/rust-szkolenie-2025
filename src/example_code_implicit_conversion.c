#include <stdio.h>

// Code to show some implicit conversion
// Stuff we try to avoid in Rust
// TODO: Create a code to show implicit conversion
int main() {
  printf("Implicit conversion example for Rust course.\n");

  int a = 10;
  float b = 3.1415f;
  double c;

  float result1 = a + b;
  printf("int + float = %f", result1);

  c = b;
  printf("float to double: %lf\n", c);

  double result2 = a * b;
  printf("int * float = %lf\n", result2);

  char d = 'A';
  int e = d + 2;
  printf("char to int: %d\n", e);
  printf("char to int: %c\n", e);

  return 0;
}
