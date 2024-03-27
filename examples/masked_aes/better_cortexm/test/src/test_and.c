#include <stdint.h>
#include "masked_and.h"

int test_and(void (*rng_fill)(char *, int))
{
    uint16_t a0[D + 1];
    uint16_t b0[D + 1];
    uint16_t a1[D + 1];
    uint16_t b1[D + 1];
    uint16_t res0[D + 1];
    uint16_t res1[D + 1];
    uint32_t fresh_randoms[20];

    rng_fill((char *)a0, 2*D + 1);
    rng_fill((char *)b0, 2*D + 1);
    rng_fill((char *)a1, 2*D + 1);
    rng_fill((char *)b1, 2*D + 1);
    rng_fill((char *)fresh_randoms, 4*R);

    masked_and(a0, b0, a1, b1, res0, res1, fresh_randoms);

    // Verif
    uint16_t a0_unmasked  = 0;
    uint16_t b0_unmasked  = 0;
    uint16_t a1_unmasked  = 0;
    uint16_t b1_unmasked  = 0;
    uint16_t res0_unmasked = 0;
    uint16_t res1_unmasked = 0;
    for (int i = 0; i < D + 1; i++) {
        a0_unmasked  ^= a0[i];
        b0_unmasked  ^= b0[i];
        a1_unmasked  ^= a1[i];
        b1_unmasked  ^= b1[i];
        res0_unmasked ^= res0[i];
        res1_unmasked ^= res1[i];
    }

    uint16_t res0_true = (a0_unmasked & b0_unmasked); 
    uint16_t res1_true = (a1_unmasked & b1_unmasked);
    if (res0_true != res0_unmasked || res1_true != res1_unmasked) {
        return 1;
    }
    return 0;

}
