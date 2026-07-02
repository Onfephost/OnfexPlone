#include <stdio.h>
#include <string.h>

int main() {
    int nots[4];
    for (int i = 0; i < 4; i++) {
        printf("Enter %d. note: ", i + 1);
        scanf("%d", &nots[i]);
    }
    int sum = 0;
    for (int i = 0; i < 4; i++) {
        sum += nots[i];
    }
    printf("Average of notes: %d\n", sum/4);
    return 0;
}
