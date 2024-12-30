#include "stdio.h"
#include "stdlib.h"
#include "test1.h"

void greet() {
    printf("Hello, World!\n");
}

void* get_n_mem(int n) {
    return malloc(n);
}