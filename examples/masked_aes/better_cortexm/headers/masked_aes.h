#include <stdint.h>

void masked_aes_keyschedule128(uint16_t key[8][D + 1], uint16_t masked_bs_rkeys[11][8][D + 1], void (*rng_fill)(char *, int));
void masked_aes_encrypt128(uint16_t state[8][D + 1], uint16_t rkeys[11][8][D + 1], void (*rng_fill)(char *, int));

//
void mask_bitslice_state(uint8_t state[16], uint16_t masked_bs_state[8][D + 1], void (*rng_fill)(char *, int));
void mask(uint16_t v, uint16_t dst[D + 1], void (*rng_fill)(char *, int));
void masked_mixcolumns(uint16_t state[8][D + 1]);
void masked_shiftrows(uint16_t state[8][D + 1]);
void masked_aes_sbox(uint16_t state[8][D + 1], void (*rng_fill)(char *, int));
void masked_and(uint16_t a0[D + 1], uint16_t b0[D + 1], uint16_t a1[D + 1], uint16_t b1[D + 1], uint16_t res0[D + 1], uint16_t res1[D + 1], uint32_t fresh_randoms[R]);
