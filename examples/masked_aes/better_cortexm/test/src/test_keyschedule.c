#include <stdint.h>

#include "masking.h"
#include "masked_aes_keyschedule.h"
#include "bitslicing.h"
#include "masked_utils.h"


static uint32_t true_rkeys[11][4] = {
    { 0x16157e2b, 0xa6d2ae28, 0x8815f7ab, 0x3c4fcf09 },
    { 0x17fefaa0, 0xb12c5488, 0x3939a323, 0x05766c2a },
    { 0xf295c2f2, 0x43b9967a, 0x7a803559, 0x7ff65973 },
    { 0x7d47803d, 0x3efe1647, 0x447e231e, 0x3b887a6d },
    { 0x41a544ef, 0x7f5b52a8, 0x3b2571b6, 0x00ad0bdb },
    { 0xf8c6d1d4, 0x879d837c, 0xbcb8f2ca, 0xbc15f911 },
    { 0x7aa3886d, 0xfd3e0b11, 0x4186f9db, 0xfd9300ca },
    { 0x0ef7544e, 0xf3c95f5f, 0xb24fa684, 0x4fdca64e },
    { 0x2173d2ea, 0xd2ba8db5, 0x60f52b31, 0x2f298d7f },
    { 0xf36677ac, 0x21dcfa19, 0x4129d128, 0x6e005c57 },
    { 0xa8f914d0, 0x8925eec9, 0xc80c3fe1, 0xa60c63b6 }
};

int test_vectors_keyschedule(void (*rng_fill)(char *, int))
{
    int nb_err = 0;

    //uint8_t key[16] = {0x16, 0x15, 0x7e, 0x2b, 0xa6, 0xd2, 0xae, 0x28, 0x88, 0x15, 0xf7, 0xab, 0x3c, 0x4f, 0xcf, 0x09};
    uint8_t key[16] = {0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c};
    uint16_t masked_bs_rkeys[11][8][D + 1];
    uint16_t masked_bs_key[8][D + 1];

    mask_bitslice_state(key, masked_bs_key, rng_fill);
    masked_aes_keyschedule128(masked_bs_key, masked_bs_rkeys, rng_fill);



    uint32_t rkeys[11][4];
    for (int i = 0; i < 11; i++) {
        unbitslice_unmask_state(masked_bs_rkeys[i], (uint8_t *)(rkeys[i]));
        for (int j = 0; j < 4; j++) {
            if (true_rkeys[i][j] != rkeys[i][j]) nb_err++;
        }
    }
    return nb_err;

}
