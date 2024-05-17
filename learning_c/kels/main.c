#include <stdio.h>
#include <time.h>

#define ITERATIONS 100000000

int main() {
    int n = 123456789;

    // Measure time for bitwise AND
    clock_t start_and = clock();
    for (int i = 0; i < ITERATIONS; i++) {
        int result = n & 1;
    }
    clock_t end_and = clock();
    double time_and = (double)(end_and - start_and) / CLOCKS_PER_SEC;

    // Measure time for modulo
    clock_t start_mod = clock();
    for (int i = 0; i < ITERATIONS; i++) {
        int result = n % 2;
    }
    clock_t end_mod = clock();
    double time_mod = (double)(end_mod - start_mod) / CLOCKS_PER_SEC;

    printf("Time for bitwise AND: %f seconds\n", time_and);
    printf("Time for modulo: %f seconds\n", time_mod);

    return 0;
}

