/* Instantiates jsmn's implementation exactly once. JSMN_STATIC is not
 * defined, so jsmn.h's JSMN_API expands to `extern`, giving jsmn_init()
 * and jsmn_parse() external linkage that Rust can bind against. */
#include "jsmn/jsmn.h"
