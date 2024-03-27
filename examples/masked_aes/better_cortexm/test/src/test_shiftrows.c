#include <stdint.h>

#include "masked_shiftrows.h"
#include "masking.h"

void swapmove(uint16_t *r, uint16_t m, int n)
{
    uint16_t t = (*r ^ (*r << n)) & m;
    *r = *r ^ t;
    *r = *r ^ (t >> n);
}

void shiftrows(uint16_t state[8])
{
    for (int i = 0; i < 8; i++) {
        swapmove(state + i, 0x4c80, 2);
        swapmove(state + i, 0xa0a0, 1);
    }
}

int test_shiftrows(void (*rng_fill)(char *, int))
{
    uint16_t bs_state[8];
    uint16_t bs_masked_state[8][D + 1];
    int nb_err = 0;

    rng_fill((char *) bs_state, 8*2);

    for (int i = 0; i < 8; i++) {
        mask(bs_state[i], bs_masked_state[i], rng_fill);
    }

    masked_shiftrows(bs_masked_state);
    shiftrows(bs_state);

    for (int i = 0; i < 8; i++) {
        if (unmask(bs_masked_state[i]) != bs_state[i]) nb_err++;
    }

    return nb_err;
}
