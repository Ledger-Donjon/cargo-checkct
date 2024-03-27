#include <stdint.h>

#include "masked_mixcolumns.h"
#include "masking.h"
#include "bitslicing.h"

static inline uint16_t ror4(uint16_t n)
{
    return (n >> 4 | ((n << 12) & 0xF000));
}

static inline uint16_t ror8(uint16_t n)
{
    return (n >> 8 | ((n << 8) & 0xFF00));
}

void mixcolumns(uint16_t state[8])
{
    uint16_t tmp0;
    uint16_t tmp1;
    uint16_t tmp2;

    tmp0 = ror4(state[0]);
    // R0 + (R0 >>> 4) 
    tmp0 = state[0] ^ tmp0;

    // R'0
    tmp1 = ror4(state[1]);
    tmp1 = tmp1 ^ state[1];      // R1 + (R1 >>> 4)

    state[0] = ror4(state[0]);
    state[0] = state[0] ^ tmp1;  // (R0 >>> 4) + (R1 + (R1 >>> 4))
    tmp2 = ror8(tmp0);                      // (R0 + (R0 >>> 4)) >>> 16
    state[0] = state[0] ^ tmp2;
    
    // R'1
    tmp2 = ror4(state[2]);
    tmp2 = tmp2 ^ state[2];      // R2 + (R2 >>> 4)

    state[1] = ror4(state[1]);
    state[1] = state[1] ^ tmp2;
    tmp1 = ror8(tmp1);                      // (R1 + (R1 >>> 4)) >>> 16
    state[1] = state[1] ^ tmp1;

    // R'2
    tmp1 = ror4(state[3]);
    tmp1 = tmp1 ^ state[3];      // R3 + (R3 >>> 4)

    tmp2 = ror8(tmp2);                      // (R2 + (R2 >>> 4)) >>> 16
    state[2] = ror4(state[2]);
    state[2] = state[2] ^ tmp1;
    state[2] = state[2] ^ tmp2;

    // R'3
    tmp2 = ror4(state[4]);
    tmp2 = tmp2 ^ state[4];      // R4 + (R4 >>> 4)

    tmp1 = ror8(tmp1);                      // (R3 + (R3 >>> 4)) >>> 16
    state[3] = ror4(state[3]);
    state[3] = state[3] ^ tmp2;
    state[3] = state[3] ^ tmp1;
    state[3] = state[3] ^ tmp0;  // Adding R0

    // R'4
    tmp1 = ror4(state[5]);
    tmp1 = tmp1 ^ state[5];      // R5 + (R5 >>> 4)

    tmp2 = ror8(tmp2);                      // (R4 + (R4 >>> 4)) >>> 16
    state[4] = ror4(state[4]);
    state[4] = state[4] ^ tmp1;
    state[4] = state[4] ^ tmp2;
    state[4] = state[4] ^ tmp0;  // Adding R0

    // R'5
    tmp2 = ror4(state[6]);
    tmp2 = tmp2 ^ state[6];      // R6 + (R6 >>> 4)

    tmp1 = ror8(tmp1);                      // (R5 + (R5 >>> 4)) >>> 16
    state[5] = ror4(state[5]);
    state[5] = state[5] ^ tmp2;
    state[5] = state[5] ^ tmp1;

    // R'6
    tmp1 = ror4(state[7]);
    tmp1 = tmp1 ^ state[7];      // R7 + (R7 >>> 4)

    tmp2 = ror8(tmp2);                      // (R6 + (R6 >>> 4)) >>> 16
    state[6] = ror4(state[6]);
    state[6] = state[6] ^ tmp1;
    state[6] = state[6] ^ tmp2;
    state[6] = state[6] ^ tmp0;  // Adding R0

    // R'7
    state[7] = ror4(state[7]);
    state[7] = state[7] ^ tmp0;
    tmp1 = ror8(tmp1);                      // (R7 + (R7 >>> 4)) >>> 16
    state[7] = state[7] ^ tmp1;
}

int test_mixcolumns(void (*rng_fill)(char *, int))
{
    uint16_t bs_state[8];
    uint32_t state[4];
    uint16_t bs_masked_state[8][D + 1];
    int nb_err = 0;

    rng_fill((char *) state, 8*2);
    bitslice(state[0], state[1], state[2], state[3], bs_state);

    for (int i = 0; i < 8; i++) {
        mask(bs_state[i], bs_masked_state[i], rng_fill);
    }

    masked_mixcolumns(bs_masked_state);
    mixcolumns(bs_state);

    for (int i = 0; i < 8; i++) {
        if (unmask(bs_masked_state[i]) != bs_state[i]) nb_err++;
    }

    unbitslice(bs_state, state);

    return nb_err;
}

int test_vectors_mixcolumns(void (*rng_fill)(char *, int))
{
    uint16_t bs_state[8];
    uint32_t state[4];
    uint16_t bs_masked_state[8][D + 1];
    int nb_err = 0;

    state[0] = 0x455313db;
    state[1] = 0x5c220af2;
    state[2] = 0x01010101;
    state[3] = 0xc6c6c6c6;

    bitslice(state[0], state[1], state[2], state[3], bs_state);

    for (int i = 0; i < 8; i++) {
        mask(bs_state[i], bs_masked_state[i], rng_fill);
    }

    masked_mixcolumns(bs_masked_state);

    for (int i = 0; i < 8; i++) {
        bs_state[i] = unmask(bs_masked_state[i]);
    }

    unbitslice(bs_state, state);

    if (state[0] != 0xbca14d8e) nb_err++;
    if (state[1] != 0x9d58dc9f) nb_err++;
    if (state[2] != 0x01010101) nb_err++;
    if (state[3] != 0xc6c6c6c6) nb_err++;

    return nb_err;
}
