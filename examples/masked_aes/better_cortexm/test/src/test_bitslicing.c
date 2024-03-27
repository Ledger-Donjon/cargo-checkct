#include <stdint.h>

#include "bitslicing.h"

int test_bitslicing(void (*rng_fill)(char *, int))
{
    uint16_t bs_state[8];
    uint8_t state[16];
    uint32_t *state32 = (uint32_t *)state;
    uint8_t result[16];
    uint32_t *result32 = (uint32_t *)result;

    rng_fill((char *)state, 16);

    bitslice(state32[0], state32[1], state32[2], state32[3], bs_state);
    unbitslice(bs_state, result32);

    for (int i = 0; i < 16; i++) {
        if (result[i] != state[i]) return 1;
    }
    return 0;
}
