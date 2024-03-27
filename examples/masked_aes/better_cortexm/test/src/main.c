#include <stdint.h>
#include <stdio.h>

#include "xoshiro.h"
#include "test_and.h"
#include "test_aes_sbox.h"
#include "test_mask_unmask.h"
#include "test_ror.h"
#include "test_shiftrows.h"
#include "test_bitslicing.h"
#include "test_mixcolumns.h"
#include "test_keyschedule.h"
#include "test_utils.h"
#include "test_masked_aes.h"

// REV and REV16 instruction
void my_exit(int status, int id)
{
    register int r0 __asm__("r0") = status;
    register int r1 __asm__("r1") = id;
    __asm__("swi 1"); // Makes qemu crash and print register values
}

void launch_test(int (*test_fnc)(void (*)(char *, int)), int *seed, int id)
{
    int nb_err = 0;

    for (int i = 0; i < 1000; i++,(*seed)++) {
        prng_init(*seed);
        nb_err += test_fnc(prng_fill);
    }

    if (nb_err != 0) my_exit(nb_err, id);
}

void main() {
    int seed = 511553;

    launch_test(&test_and,                  &seed, 0);
    launch_test(&test_mask_unmask,          &seed, 1);
    launch_test(&test_aes_sbox,             &seed, 2);
    launch_test(&test_ror,                  &seed, 3);
    launch_test(&test_shiftrows,            &seed, 4);
    launch_test(&test_bitslicing,           &seed, 5);
    launch_test(&test_mixcolumns,           &seed, 6);
    launch_test(&test_vectors_mixcolumns,   &seed, 7);
    launch_test(&test_utils,                &seed, 8);
    launch_test(&test_vectors_keyschedule,  &seed, 9);
    launch_test(&test_vectors_aes,          &seed, 10);
    my_exit(0, -1);

}
