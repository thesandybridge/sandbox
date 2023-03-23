#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <getopt.h>

int calc(int n1, char* op, int n2) {
    int result;
    if (strcmp(op, "+") == 0) {
        result = n1 + n2;
    } else if (strcmp(op, "-") == 0) {
        result = n1 - n2;
    } else if (strcmp(op, "*") == 0) {
        result = n1 * n2;
    } else if (strcmp(op, "/") == 0) {
        result = n1 / n2;
    } else {
        printf("Invalid operator: %s\n", op);
        return 1;  // error code
    }
    return result;
}

int main(int argc, char *argv[]) {
    int opt;
    char *operator;

    while ((opt = getopt(argc, argv, "o:")) != -1) {
        switch (opt) {
            case 'o':
                operator = optarg;
                break;
            default:
                fprintf(stderr, "Usage: %s -o operator num1 num2\n", argv[0]);
                exit(EXIT_FAILURE);
        }
    }

    if (optind + 2 > argc) {
        fprintf(stderr, "Usage: %s -o operator num1 num2\n", argv[0]);
        exit(EXIT_FAILURE);
    }

    long op1 = atoi(argv[optind]);
    long op2 = atoi(argv[optind + 1]);

    printf("%d\n", calc(op1, operator, op2));

    return 0;
}
