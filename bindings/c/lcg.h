#ifndef LCG_H
#define LCG_H
#include <stdint.h>

struct LGC {
    uint64_t state;
};

struct LCG lcg_init(uint64_t seed);
uint32_t lcg_rand(struct LCG *rng);

#endif /* LCG_H */
