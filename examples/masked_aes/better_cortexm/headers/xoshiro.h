// /!\ Quick, dirty and weak PRNG, not to be used in production like this.

void prng_init(int seed);
void prng_fill(char *dst, int size);
uint64_t next(void);
