#ifndef MY_RUST_LIB_H
#define MY_RUST_LIB_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

int32_t square(int32_t value);
int32_t divide(int32_t a, int32_t b, int32_t *result);

#ifdef __cplusplus
}
#endif

#endif /* MY_RUST_LIB_H */
