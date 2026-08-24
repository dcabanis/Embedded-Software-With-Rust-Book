#ifndef CALLBACK_LIB_H
#define CALLBACK_LIB_H

#include <stddef.h>
#include <stdint.h>

/* Pointer + length */
int32_t sum_array(const int32_t *arr, size_t len);

/* NUL-terminated C string. No <string.h> on this
 * target, so the library provides its own strlen-equivalent. */
size_t c_string_len(const char *s);

/* Callback with a context pointer. `register_callback_ctx`
 * just remembers `cb` and `ctx`; `trigger_callback_ctx` is what actually
 * calls back into whichever function was last registered, handing back
 * the exact same `ctx` pointer unchanged. */
typedef void (*callback_ctx_t)(int32_t value, void *ctx);

void register_callback_ctx(callback_ctx_t cb, void *ctx);
void trigger_callback_ctx(int32_t value);

#endif /* CALLBACK_LIB_H */
