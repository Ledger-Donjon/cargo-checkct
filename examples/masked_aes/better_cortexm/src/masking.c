#include <stdint.h>
#include "masking.h"

void mask(uint16_t v, uint16_t dst[D + 1], void (*rng_fill)(char *, int))
{
    rng_fill((char *)dst, D*2);

    uint16_t tmp = 0;
    for (int i = 0; i < D; i++) {
        tmp ^= dst[i];
    }
    dst[D] = tmp ^ v;
}

void mask32(uint32_t v, uint32_t dst[D + 1], void (*rng_fill)(char *, int))
{
    rng_fill((char *)dst, D*4);

    uint32_t tmp = 0;
    for (int i = 0; i < D; i++) {
        tmp ^= dst[i];
    }
    dst[D] = tmp ^ v;
}
            
uint16_t unmask(uint16_t v[D + 1])
{
    uint16_t res = 0;
    for (int i = 0; i < D + 1; i++) {
        res ^= v[i];
    }
    return res;
}

uint32_t unmask32(uint32_t v[D + 1])
{
    uint32_t res = 0;
    for (int i = 0; i < D + 1; i++) {
        res ^= v[i];
    }
    return res;
}
