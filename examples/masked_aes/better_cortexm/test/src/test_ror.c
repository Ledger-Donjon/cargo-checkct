#include <stdint.h>

#include "masked_ror.h"

#define ROTR16(x,shift) ((uint16_t) ((x) >> (shift)) | (uint16_t) ((x) << (16 - (shift))))

int test_ror(void (*rng_fill)(char *, int))
{
    uint16_t masked_v[8];
    uint16_t   ror4_v[8];
    uint16_t   ror8_v[8];
    int nb_err = 0;

    rng_fill((char *)masked_v, 2*8);

    masked_ror4_8(masked_v, ror4_v);
    masked_ror8_8(masked_v, ror8_v);

    // Verif
    for (int i = 0; i < 8; i++) {
        if (ror4_v[i] != ROTR16(masked_v[i], 4)) nb_err++;
        if (ror8_v[i] != ROTR16(masked_v[i], 8)) nb_err++;
    }

    return nb_err;
}
