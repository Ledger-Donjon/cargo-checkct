#include "masked_aes_sbox.h"
#include "masked_and.h"
#include "masked_xor.h"
#include "masked_sbox_lin.h"
#include <stdint.h>

void masked_aes_sbox(uint16_t state[8][D + 1], void (*rng_fill)(char *, int))
{
    uint16_t all_tmp[35][D + 1];
    uint32_t fresh_randoms[R];

    masked_sbox_l1(all_tmp, state);

    rng_fill((char *)fresh_randoms, R*4);
    masked_and(all_tmp[11], all_tmp[14], all_tmp[2], all_tmp[5], state[6], state[5], fresh_randoms); // t2 = y12 & y15 ; t3 = y3 & y6
    masked_xor(state[5], state[6], state[5]); // t4 = t3 ^ t2
    rng_fill((char *)fresh_randoms, R*4);
    masked_and(all_tmp[12], all_tmp[15], all_tmp[3], state[7], state[4], state[3], fresh_randoms); // t7 = y13 & y16 ; t5 = y4 & U7
    masked_xor(state[3], state[6], state[6]); // t6 = t5 ^ t2
    rng_fill((char *)fresh_randoms, R*4);
    masked_and(all_tmp[1], all_tmp[6], all_tmp[4], all_tmp[0], state[3], state[2], fresh_randoms); // t10 = y2 & y7 ; t8 = y5 & y1
    masked_xor(state[2], state[4], state[2]); // t9 = t8 ^ t7
    masked_xor(state[3], state[4], state[4]); // t11 = t10 ^ t7
    rng_fill((char *)fresh_randoms, R*4);
    masked_and(all_tmp[8], all_tmp[10], all_tmp[13], all_tmp[16], state[3], state[1], fresh_randoms); // t12 = y9 & y11 ; t13 = y14 & y17
    masked_xor(state[1], state[3], state[1]); // t14 = t13 ^ t12
    masked_xor(state[5], all_tmp[19], state[5]); // t17 = t4 ^ y20
    masked_xor(state[2], state[1], state[2]); // t19 = t9 ^ t14
    masked_xor(state[5], state[1], state[5]); // t21 = t17 ^ t14
    masked_xor(state[2], all_tmp[20], state[2]); // t23 = t19 ^ y21
    rng_fill((char *)fresh_randoms, R*4);
    masked_and(state[5], state[2], all_tmp[7], all_tmp[9], state[1], state[0], fresh_randoms); // t26 = t21 & t23 ; t15 = y8 & y10

    masked_sbox_l_int0(state, all_tmp + 17);
    //masked_xor(state[0], state[3], state[3]); // t16 = t15 ^ t12
    //masked_xor(state[6], state[3], state[6]); // t18 = t6 ^ t16
    //masked_xor(state[4], state[3], state[4]); // t20 = t11 ^ t16
    //masked_xor(state[6], all_tmp[18], state[6]); // t22 = t18 ^ y19
    //masked_xor(state[4], all_tmp[17], state[4]); // t24 = t20 ^ y18
    //masked_xor(state[5], state[6], state[5]); // t25 = t21 ^ t22
    //masked_xor(state[4], state[1], state[3]); // t27 = t24 ^ t26
    //masked_xor(state[2], state[4], state[0]); // t30 = t23 ^ t24
    //masked_xor(state[6], state[1], state[1]); // t31 = t22 ^ t26

    rng_fill((char *)fresh_randoms, R*4);
    masked_and(state[1], state[0], state[5], state[3], state[1], state[0], fresh_randoms); // t32 = t31 & t30 ; t28 = t25 & t27
    masked_xor(state[0], state[6], state[6]); // t29 = t28 ^ t22
    masked_xor(state[1], state[4], state[1]); // t33 = t32 ^ t24
    masked_xor(state[2], state[1], state[2]); // t34 = t23 ^ t33
    masked_xor(state[3], state[1], state[0]); // t35 = t27 ^ t33
    rng_fill((char *)fresh_randoms, R*4);
    masked_and(state[1], state[7], state[4], state[0], all_tmp[19], state[7], fresh_randoms); // z2 = t33 & U7 ; t36 = t24 & t35
    masked_xor(state[7], state[2], state[4]); // t37 = t36 ^ t34
    masked_xor(state[3], state[7], state[7]); // t38 = t27 ^ t36
    masked_xor(state[1], state[4], state[3]); // t44 = t33 ^ t37
    rng_fill((char *)fresh_randoms, R*4);
    masked_and(state[3], all_tmp[14], state[6], state[7], all_tmp[17], state[7], fresh_randoms); // z0 = t44 & y15 ; t39 = t29 & t38
    masked_xor(state[5], state[7], state[7]); // t40 = t25 ^ t39
    masked_xor(state[7], state[4], state[5]); // t41 = t40 ^ t37
    masked_xor(state[6], state[1], state[2]); // t42 = t29 ^ t33
    masked_xor(state[6], state[7], state[0]); // t43 = t29 ^ t40
    masked_xor(state[2], state[5], all_tmp[34]); // t45 = t42 ^ t41
    rng_fill((char *)fresh_randoms, R*4);
    masked_and(state[4], all_tmp[5], state[0], all_tmp[15], all_tmp[18], all_tmp[20], fresh_randoms); // z1 = t37 & y6 ; z3 = t43 & y16
    rng_fill((char *)fresh_randoms, R*4);
    masked_and(state[7], all_tmp[0], state[6], all_tmp[6], all_tmp[21], all_tmp[22], fresh_randoms); // z4 = t40 & y1 ; z5 = t29 & y7
    rng_fill((char *)fresh_randoms, R*4);
    masked_and(state[2], all_tmp[10], all_tmp[34], all_tmp[16], all_tmp[23], all_tmp[24], fresh_randoms); // z6 = t42 & y11 ; z7 = t45 & y17
    rng_fill((char *)fresh_randoms, R*4);
    masked_and(state[5], all_tmp[9], state[3], all_tmp[11], all_tmp[25], all_tmp[26], fresh_randoms); // z8 = t41 & y10 ; z9 = t44 & y12
    rng_fill((char *)fresh_randoms, R*4);
    masked_and(state[4], all_tmp[2], state[1], all_tmp[3], all_tmp[27], all_tmp[28], fresh_randoms); // z10 = t37 & y3 ; z11 = t33 & y4
    rng_fill((char *)fresh_randoms, R*4);
    masked_and(state[0], all_tmp[12], state[7], all_tmp[4], all_tmp[29], all_tmp[30], fresh_randoms); // z12 = t43 & y13 ; z13 = t40 & y5
    rng_fill((char *)fresh_randoms, R*4);
    masked_and(state[6], all_tmp[1], state[2], all_tmp[8], all_tmp[31], all_tmp[32], fresh_randoms); // z14 = t29 & y2 ; z15 = t42 & y9
    rng_fill((char *)fresh_randoms, R*4);
    masked_and(all_tmp[34], all_tmp[13], state[5], all_tmp[7], all_tmp[33], all_tmp[34], fresh_randoms); // z16 = t45 & y14 ; z17 = t41 & y8


    masked_sbox_l2(state, all_tmp + 17);


    state[7][0] = ~state[7][0];
    state[6][0] = ~state[6][0];
    state[1][0] = ~state[1][0];
    state[2][0] = ~state[2][0];
}

