#ifndef TEST_MASK_UNMASK_H
#define TEST_MASK_UNMASK_H

int test_mask_unmask(void (*rng_fill)(char *, int));
int test_mask_bitslice(void (*rng_fill)(char *, int));
int test_bitslice(void (*rng_fill)(char *, int));

#endif /* TEST_MASK_UNMASK_H */
