#include <stdint.h>

#include "masked_sbox_lin.h"
#include "masked_xor.h"

void ref_l_int0(uint16_t state[7][D+1], uint16_t all_tmp[2][D+1])
{
    masked_xor(state[0], state[3], state[3]); // t16 = t15 ^ t12
    masked_xor(state[6], state[3], state[6]); // t18 = t6 ^ t16
    masked_xor(state[4], state[3], state[4]); // t20 = t11 ^ t16
    masked_xor(state[6], all_tmp[1], state[6]); // t22 = t18 ^ y19
    masked_xor(state[4], all_tmp[0], state[4]); // t24 = t20 ^ y18
    masked_xor(state[5], state[6], state[5]); // t25 = t21 ^ t22
    masked_xor(state[4], state[1], state[3]); // t27 = t24 ^ t26
    masked_xor(state[2], state[4], state[0]); // t30 = t23 ^ t24
    masked_xor(state[6], state[1], state[1]); // t31 = t22 ^ t26
}


void ref_l1(uint16_t all_tmp[21][D+1], const uint16_t state[8][D+1])
{
    uint16_t sup_tmp0[D + 1];
    uint16_t sup_tmp1[D + 1];

    masked_xor(state[3],       state[5],       all_tmp[13]); // y14 = U3 + U5
    masked_xor(state[0],       state[6],       all_tmp[12]); // y13 = U0 + U6
    masked_xor(state[0],       state[3],       all_tmp[8]); // y9 = U0 + U3
    masked_xor(state[0],       state[5],       all_tmp[7]); // y8 = U0 + U5
    masked_xor(state[1],       state[2],   sup_tmp0); // t0 = U1 + U2
    masked_xor(sup_tmp0,       state[7],       all_tmp[0]); // y1 = t0 + U7
    masked_xor(all_tmp[0],     state[3],       all_tmp[3]); // y4 = y1 + U3
    masked_xor(all_tmp[12], all_tmp[13],       all_tmp[11]); // y12 = y13 + y14
    masked_xor(all_tmp[0],     state[0],       all_tmp[1]); // y2 = y1 + U0
    masked_xor(all_tmp[0],     state[6],       all_tmp[4]); // y5 = y1 + U6
    masked_xor(all_tmp[4],   all_tmp[7],       all_tmp[2]); // y3 = y5 + y8
    masked_xor(state[4],    all_tmp[11],   sup_tmp1); // t1 = U4 + y12
    masked_xor(sup_tmp1,       state[5],       all_tmp[14]); // y15 = t1 + U5
    masked_xor(sup_tmp1,       state[1],       all_tmp[19]); // y20 = t1 + U1
    masked_xor(all_tmp[14],    state[7],       all_tmp[5]); // y6 = y15 + U7
    masked_xor(all_tmp[14],    sup_tmp0,       all_tmp[9]); // y10 = y15 + t0
    masked_xor(all_tmp[19],  all_tmp[8],       all_tmp[10]); // y11 = y20 + y9
    masked_xor(state[7],    all_tmp[10],       all_tmp[6]); // y7 = U7 + y11
    masked_xor(all_tmp[9],  all_tmp[10],       all_tmp[16]); // y17 = y10 + y11
    masked_xor(all_tmp[9],   all_tmp[7],       all_tmp[18]); // y19 = y10 + y8
    masked_xor(sup_tmp0,    all_tmp[10],       all_tmp[15]); // y16 = t0 + y11
    masked_xor(all_tmp[12], all_tmp[15],       all_tmp[20]); // y21 = y13 + y16
    masked_xor(state[0],    all_tmp[15],       all_tmp[17]); // y18 = U0 + y16
}

void ref_l2(uint16_t  state[8][D+1], uint16_t all_tmp[18][D + 1])
{
    masked_xor(all_tmp[15], all_tmp[16], all_tmp[16]); // tc1 = z15 + z16
    masked_xor(all_tmp[10], all_tmp[16], all_tmp[10]); // tc2 = z10 + tc1
    masked_xor(all_tmp[9],  all_tmp[10], all_tmp[9]); // tc3 = z9 + tc2
    masked_xor(all_tmp[0],   all_tmp[2], all_tmp[2]); // tc4 = z0 + z2
    masked_xor(all_tmp[1],   all_tmp[0], all_tmp[1]); // tc5 = z1 + z0
    masked_xor(all_tmp[3],   all_tmp[4], all_tmp[4]); // tc6 = z3 + z4
    masked_xor(all_tmp[12],  all_tmp[2], all_tmp[0]); // tc7 = z12 + tc4
    masked_xor(all_tmp[7],   all_tmp[4], all_tmp[7]); // tc8 = z7 + tc6
    masked_xor(all_tmp[8],   all_tmp[0], all_tmp[8]); // tc9 = z8 + tc7
    masked_xor(all_tmp[7],   all_tmp[8], all_tmp[8]); // tc10 = tc8 + tc9
    masked_xor(all_tmp[4],   all_tmp[1], all_tmp[1]); // tc11 = tc6 + tc5
    masked_xor(all_tmp[3],   all_tmp[5], all_tmp[3]); // tc12 = z3 + z5
    masked_xor(all_tmp[13], all_tmp[16], all_tmp[13]); // tc13 = z13 + tc1
    masked_xor(all_tmp[2],   all_tmp[3], all_tmp[2]); // tc14 = tc4 + tc12

    masked_xor(all_tmp[9],   all_tmp[1],    state[3]); // S3 = tc3 + tc11
    masked_xor(all_tmp[6],   all_tmp[7], all_tmp[6]); // tc16 = z6 + tc8
    masked_xor(all_tmp[14],  all_tmp[8], all_tmp[14]); // tc17 = z14 + tc10
    masked_xor(all_tmp[13],  all_tmp[2], all_tmp[13]); // tc18 = tc13 + tc14
    masked_xor(all_tmp[12], all_tmp[13],    state[7]); // S7 = z12 # tc18
    masked_xor(all_tmp[15],  all_tmp[6], all_tmp[15]); // tc20 = z15 + tc16
    masked_xor(all_tmp[10], all_tmp[11], all_tmp[11]); // tc21 = tc2 + z11
    masked_xor(all_tmp[9],   all_tmp[6],    state[0]); // S0 = tc3 + tc16
    masked_xor(all_tmp[8],  all_tmp[13],    state[6]); // S6 = tc10 # tc18
    masked_xor(all_tmp[2],     state[3],    state[4]); // S4 = tc14 + S3
    masked_xor(state[3],     all_tmp[6],    state[1]); // S1 = S3 # tc16
    masked_xor(all_tmp[14], all_tmp[15], all_tmp[15]); // tc26 = tc17 + tc20
    masked_xor(all_tmp[15], all_tmp[17],    state[2]); // S2 = tc26 # z17
    masked_xor(all_tmp[11], all_tmp[14],    state[5]); // S5 = tc21 + tc17
}

int test_sbox_lin1(void (*rng_fill)(char *, int))
{
    int nb_err = 0;
    uint16_t all_tmp_ref[21][D+1];
    uint16_t all_tmp[21][D+1];
    uint16_t state[8][D+1];
    rng_fill((char *)state, (D+1)*8*2);
    masked_sbox_l1(all_tmp, state);
    ref_l1(all_tmp_ref, state);

    for (int i = 0; i < 21; i++) {
        for (int j = 0; j < D+1; j++) {
            if (all_tmp[i][j] != all_tmp_ref[i][j]) nb_err++;
        }
    }
    return nb_err;
}

int test_sbox_lin2(void (*rng_fill)(char *, int))
{
    int nb_err = 0;
    uint16_t all_tmp[18][D+1];
    uint16_t state[8][D+1];
    uint16_t state_ref[8][D+1];
    rng_fill((char *)all_tmp, (D+1)*18*2);
    masked_sbox_l2(state, all_tmp);
    ref_l2(state_ref, all_tmp);

    for (int i = 0; i < 8; i++) {
        for (int j = 0; j < D+1; j++) {
            if (state[i][j] != state_ref[i][j]) nb_err++;
        }
    }
    return nb_err;
}

int test_sbox_lin_int0(void (*rng_fill)(char *, int))
{
    int nb_err = 0;
    uint16_t all_tmp[2][D+1];
    uint16_t state[7][D+1];
    uint16_t state_ref[7][D+1];

    rng_fill((char *)state, (D+1)*7*2);
    rng_fill((char *)all_tmp, (D+1)*2*2);

    for (int i = 0; i < 7; i++) {
        for (int j = 0; j < D + 1; j++) {
            state_ref[i][j] = state[i][j];
        }
    }

    //    printf("\n");
    //    printf("\n");
    //    printf("\n");
    //    printf("\n");
    //    printf("\n");

    masked_sbox_l_int0(state, all_tmp);
    ref_l_int0(state_ref, all_tmp);


    for (int i = 0; i < 7; i++) {
        for (int j = 0; j < D+1; j++) {
            if (state[i][j] != state_ref[i][j]) nb_err++;
        }
    }
    return nb_err;
}
