#include "callback_lib.h"

int32_t sum_array(const int32_t *arr, size_t len) {
    int32_t total = 0;
    for (size_t i = 0; i < len; i++) {
        total += arr[i];
    }
    return total;
}

size_t c_string_len(const char *s) {
    size_t len = 0;
    while (s[len] != '\0') {
        len++;
    }
    return len;
}

/* Where the registered function pointer and its context are remembered
 * between calls. Both start at 0 ("nothing registered yet"). */
static callback_ctx_t registered_callback = 0;
static void *registered_ctx = 0;

void register_callback_ctx(callback_ctx_t cb, void *ctx) {
    registered_callback = cb;
    registered_ctx = ctx;
}

void trigger_callback_ctx(int32_t value) {
    if (registered_callback != 0) {
        registered_callback(value, registered_ctx);
    }
}
