#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <time.h>

// Function to calculate the fast inverse square root
float rsqrt(float number) {
    long i;
    float x2, y;
    const float threehalfs = 1.5F;

    x2 = number * 0.5F;
    y = number;

    i = * (long *) &y;
    i = 0x5f3759df - (i >> 1);
    y = * (float *) &i;

    // Perform two iterations of Newton's method for improved accuracy
    y = y * (threehalfs - (x2 * y * y));
    y = y * (threehalfs - (x2 * y * y));

    return y;
}

// Function to generate an array of random floats
void generateRandomFloats(float *array, int n) {
    for (int i = 0; i < n; i++) {
        array[i] = (float)rand() / RAND_MAX * 100.0f; // Random floats between 0 and 100
    }
}

// Function to calculate the average of an array of floats
float calculateAverage(float *array, int n) {
    float sum = 0.0f;
    for (int i = 0; i < n; i++) {
        sum += array[i];
    }
    return sum / n;
}

// Function to measure time taken by a function
double measureTime(void (*func)(float *, float *, int), float *inputArray, float *outputArray, int n) {
    clock_t start, end;
    start = clock();
    func(inputArray, outputArray, n);
    end = clock();
    return ((double) (end - start)) / CLOCKS_PER_SEC;
}

// Wrapper for rsqrt calculations
void rsqrtWrapper(float *inputArray, float *outputArray, int n) {
    for (int i = 0; i < n; i++) {
        outputArray[i] = rsqrt(inputArray[i]);
    }
}

// Wrapper for built-in sqrt calculations with division
void sqrtDivWrapper(float *inputArray, float *outputArray, int n) {
    for (int i = 0; i < n; i++) {
        outputArray[i] = 1.0f / sqrt(inputArray[i]);
    }
}

// Wrapper for built-in sqrt calculations alone
void sqrtWrapper(float *inputArray, float *outputArray, int n) {
    for (int i = 0; i < n; i++) {
        outputArray[i] = sqrt(inputArray[i]);
    }
}

int main() {
    int n = 10000000; // Length of the array
    int runs = 1; // Number of runs to average
    float *inputArray = (float *)malloc(n * sizeof(float));
    float *outputArray = (float *)malloc(n * sizeof(float));
    generateRandomFloats(inputArray, n);

    double totalTimeBitHack = 0.0;
    double totalTimeSqrtDiv = 0.0;
    double totalTimeSqrt = 0.0;

    for (int i = 0; i < runs; i++) {
        totalTimeBitHack += measureTime(rsqrtWrapper, inputArray, outputArray, n);
        float averageBitHack = calculateAverage(outputArray, n);
        printf("Average of the reverse square roots (bit hack) in run %d: %f\n", i+1, averageBitHack);

        totalTimeSqrtDiv += measureTime(sqrtDivWrapper, inputArray, outputArray, n);
        float averageSqrtDiv = calculateAverage(outputArray, n);
        printf("Average of the reverse square roots (sqrt division) in run %d: %f\n", i+1, averageSqrtDiv);

        totalTimeSqrt += measureTime(sqrtWrapper, inputArray, outputArray, n);
        float averageSqrt = calculateAverage(outputArray, n);
        printf("Average of the square roots (sqrt alone) in run %d: %f\n", i+1, averageSqrt);
    }

    printf("Average time taken by bit hack method: %f seconds\n", totalTimeBitHack / runs);
    printf("Average time taken by built-in sqrt function with division: %f seconds\n", totalTimeSqrtDiv / runs);
    printf("Average time taken by built-in sqrt function alone: %f seconds\n", totalTimeSqrt / runs);

    free(inputArray);
    free(outputArray);
    return 0;
}

