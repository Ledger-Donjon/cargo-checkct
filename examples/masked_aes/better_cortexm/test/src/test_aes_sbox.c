#include <stdint.h>

#include "bitslicing.h"
#include "masking.h"
#include "masked_aes_sbox.h"

#define ROTL8(x,shift) ((uint8_t) ((x) << (shift)) | ((x) >> (8 - (shift))))

void initialize_aes_sbox(uint8_t sbox[256])
{
	uint8_t p = 1, q = 1;

	/* loop invariant: p * q == 1 in the Galois field */
	do {
		/* multiply p by 3 */
		p = p ^ (p << 1) ^ (p & 0x80 ? 0x11B : 0);

		/* divide q by 3 (equals multiplication by 0xf6) */
		q ^= q << 1;
		q ^= q << 2;
		q ^= q << 4;
		q ^= q & 0x80 ? 0x09 : 0;

		/* compute the affine transformation */
		uint8_t xformed = q ^ ROTL8(q, 1) ^ ROTL8(q, 2) ^ ROTL8(q, 3) ^ ROTL8(q, 4);

		sbox[p] = xformed ^ 0x63;
	} while (p != 1);

	/* 0 is a special case since it has no inverse */
	sbox[0] = 0x63;
}

int test_aes_sbox(void (*rng_fill)(char *, int))
{
    uint8_t array_sbox[256];
    int nb_err = 0;
    initialize_aes_sbox(array_sbox);

    uint16_t bs_sbox[8];
    uint16_t bs_masked_sbox[8][D + 1];
    uint32_t v[4];

    rng_fill((char *)v, 16);
    bitslice(v[0], v[1], v[2], v[3], bs_sbox);


    for (int i = 0; i < 8; i++) {
        mask(bs_sbox[i], bs_masked_sbox[i], rng_fill);
    }

    masked_aes_sbox(bs_masked_sbox, rng_fill);

    uint32_t final_v[4];
    uint16_t final_bs[8];
    uint16_t tmp;
    for (int i = 0; i < 8; i++) {
        final_bs[i] = unmask(bs_masked_sbox[i]);
    }
    unbitslice(final_bs, final_v);

    for (int i = 0; i < 16; i++) {
        if (((uint8_t *)final_v)[i] != array_sbox[((uint8_t *)v)[i]]) nb_err++;
    }
    return nb_err;
}
