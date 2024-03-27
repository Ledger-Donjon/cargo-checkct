
#include <stdint.h>

void masked_aes_sbox(uint16_t state[8][D + 1], void (*rng_fill)(char *, int));
