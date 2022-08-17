#include <stdlib.h>

void g(void)
{
   malloc(4000);
}

void f(void)
{
   malloc(2000);
   g();
}

int main(void)
{
   int i;
   int* a[10];

   // Alloc 10.000 B.
   for (i = 0; i < 10; i++) {
      a[i] = malloc(1000);
   }
   // Total: 10.000 B.

   // Alloc 6.000 B.
   f();
   // Total: 16.000 B.

   // Alloc 4.000 B.
   g();
   // Total: 20.000 B.

   // Free 10.000 B.
   for (i = 0; i < 10; i++) {
      free(a[i]);
   }
   // Total: 10.000 B.

   return 0;
}
