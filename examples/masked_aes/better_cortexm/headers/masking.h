#ifndef MASKING_H
#define MASKING_H


void mask(uint16_t v, uint16_t dst[D + 1], void (*rng_fill)(char *, int));
void mask32(uint32_t v, uint32_t dst[D + 1], void (*rng_fill)(char *, int));
uint16_t unmask(uint16_t v[D + 1]);
uint32_t unmask32(uint32_t v[D + 1]);

#endif /* MASKING_H */
