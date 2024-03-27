#ifndef MASKED_SBOX_LIN_H
#define MASKED_SBOX_LIN_H
#include <stdint.h>

void masked_sbox_l1(uint16_t all_tmp[21][D + 1],  const uint16_t state[8][D+1]);
void masked_sbox_l2(uint16_t  state[8][D+1], uint16_t all_tmp[18][D + 1]);
void masked_sbox_l_int0(uint16_t  state[7][D+1], const uint16_t all_tmp[2][D + 1]);

#endif /* MASKED_SBOX_LIN_H */
