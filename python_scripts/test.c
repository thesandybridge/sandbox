#include <stdio.h>

void modifyValue(int *valuePtr) {
    *valuePtr += 1;
}

int main() {
    int value = 1;
    modifyValue(&value);
    printf("%d\n", value);
    return 0;
}

