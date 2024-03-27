#ifndef MASKED_AND_H
#define MASKED_AND_H
#include <stdint.h>


void masked_and(uint16_t a0[D + 1], uint16_t b0[D + 1], uint16_t a1[D + 1], uint16_t b1[D + 1], uint16_t res0[D + 1], uint16_t res1[D + 1], uint32_t fresh_randoms[R]);

#endif /* MASKED_AND_H */
