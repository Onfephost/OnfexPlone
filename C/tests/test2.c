#include <stdio.h>
#include <string.h>
int plus(int a, int b) {
    return a + b;
}

int minus(int a, int b) {
    return a - b;
}

int mul(int a, int b) {
    return a * b;
}

int div(int a, int b) {
    if (b == 0) {
        printf("Error: Division by zero\n");
        return 0; // Return 0 or handle the error as needed
    }
    return a / b;
}
void spell(char *text) {
    for (int i = 0; i < strlen(text); i++) {
        printf(" | %c", text[i]);
    }
}
struct AstNode
{
    int val;
};
