#ifndef BITSLICING_H
#define BITSLICING_H
#include <stdint.h>

void bitslice(uint32_t state0, uint32_t state1, uint32_t state2, uint32_t state3, uint16_t bs_state_out[8]);
void unbitslice(uint16_t bs_state[8], uint32_t state[4]);

#endif /*  BITSLICING_H */
