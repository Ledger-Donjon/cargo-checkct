#include <stdint.h>

#include "masked_aes_sbox.h"
#include "masked_shiftrows.h"
#include "masked_mixcolumns.h"
#include "masked_xor.h"

void static inline add_round_key(uint16_t state[8][D + 1], uint16_t rkey[8][D + 1])
{
    masked_xor(state[0], rkey[0], state[0]);
    masked_xor(state[1], rkey[1], state[1]);
    masked_xor(state[2], rkey[2], state[2]);
    masked_xor(state[3], rkey[3], state[3]);
    masked_xor(state[4], rkey[4], state[4]);
    masked_xor(state[5], rkey[5], state[5]);
    masked_xor(state[6], rkey[6], state[6]);
    masked_xor(state[7], rkey[7], state[7]);
}

void masked_aes_encrypt128(uint16_t state[8][D + 1], uint16_t rkeys[11][8][D + 1], void (*rng_fill)(char *, int))
{
    /* Round 1 */
    add_round_key(state, rkeys[0]);
    masked_aes_sbox(state, rng_fill);
    masked_shiftrows(state);
    masked_mixcolumns(state);

    /* Round 2 */
    add_round_key(state, rkeys[1]);
    masked_aes_sbox(state, rng_fill);
    masked_shiftrows(state);
    masked_mixcolumns(state);

    /* Round 3 */
    add_round_key(state, rkeys[2]);
    masked_aes_sbox(state, rng_fill);
    masked_shiftrows(state);
    masked_mixcolumns(state);

    /* Round 4 */
    add_round_key(state, rkeys[3]);
    masked_aes_sbox(state, rng_fill);
    masked_shiftrows(state);
    masked_mixcolumns(state);

    /* Round 5 */
    add_round_key(state, rkeys[4]);
    masked_aes_sbox(state, rng_fill);
    masked_shiftrows(state);
    masked_mixcolumns(state);

    /* Round 6 */
    add_round_key(state, rkeys[5]);
    masked_aes_sbox(state, rng_fill);
    masked_shiftrows(state);
    masked_mixcolumns(state);

    /* Round 7 */
    add_round_key(state, rkeys[6]);
    masked_aes_sbox(state, rng_fill);
    masked_shiftrows(state);
    masked_mixcolumns(state);

    /* Round 8 */
    add_round_key(state, rkeys[7]);
    masked_aes_sbox(state, rng_fill);
    masked_shiftrows(state);
    masked_mixcolumns(state);

    /* Round 9 */
    add_round_key(state, rkeys[8]);
    masked_aes_sbox(state, rng_fill);
    masked_shiftrows(state);
    masked_mixcolumns(state);

    /* Last round */
    add_round_key(state, rkeys[9]);
    masked_aes_sbox(state, rng_fill);
    masked_shiftrows(state);
    add_round_key(state, rkeys[10]);
}
