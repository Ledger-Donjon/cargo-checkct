#include <stdint.h>

#include "masked_utils.h"

int test_utils(void (*rng_fill)(char *, int))
{
    uint8_t state[16];
    uint8_t state1[16];
    int nb_err = 0;
    rng_fill((char *)state, 16);

    uint16_t masked_bs_state[8][D + 1];
    mask_bitslice_state(state, masked_bs_state, rng_fill);
    unbitslice_unmask_state(masked_bs_state, state1);

    for (int i = 0; i < 16; i++) {
        if (state[i] != state1[i]) nb_err++;
    }

    return nb_err;
}
