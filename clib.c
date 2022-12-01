#include <stdlib.h>
#include <stdint.h>

#define N 4

struct mat {
    uint32_t data[N][N];
};

void __attribute__((noinline)) mult(const struct mat *a, const struct mat *b, struct mat *c) {
    for (size_t i = 0; i < N; ++i) {
        for (size_t j = 0; j < N; ++j) {
            for (size_t k = 0; k < N; ++k) {
                c->data[i][j] += a->data[i][k] * b->data[k][j];
            }
        }
    }
}
