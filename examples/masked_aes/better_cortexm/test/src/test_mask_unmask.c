#include <stdint.h>

#include "masking.h"
#include "bitslicing.h"
#include "masked_utils.h"

int test_mask_unmask(void (*rng_fill)(char *, int))
{
    uint16_t masked[D + 1];
    uint32_t masked32[D + 1];
    uint16_t v;
    uint32_t v32;
    rng_fill((char *)(&v), 2);
    rng_fill((char *)(&v32), 4);
    mask(v, masked, rng_fill);
    mask32(v32, masked32, rng_fill);

    if (unmask(masked) != v) return 1;
    if (unmask32(masked32) != v32) return 1;

    return 0;
}

int test_mask_bitslice(void (*rng_fill)(char *, int))
{
    uint8_t state[16];
    uint8_t tobe_masked[16];
    uint8_t result[16];
	uint16_t masked_bs_state[8][D + 1];
    int nb_err = 0;

    rng_fill((char *)state, 16);
    for (int i = 0; i < 16; i++) {
        state[i] = tobe_masked[i];
    }

    mask_bitslice_state(tobe_masked, masked_bs_state, rng_fill);
    unbitslice_unmask_state(masked_bs_state, result);

    for (int i = 0; i < 16; i++) {
        if (state[i] != result[i]) return 1;
    }
    
    return 0;
}
