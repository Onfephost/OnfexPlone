#include <stdio.h>
#include <string.h>
#include "test2.c"

int main() {
    char text[] = "Hello, World!";
    spell(text);
    printf("Division of 10 by 2: %d\n", div(10, 2));
    return 0;
}