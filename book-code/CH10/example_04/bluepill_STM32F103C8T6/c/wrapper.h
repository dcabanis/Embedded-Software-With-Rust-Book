/* The single entry point bindgen parses. JSMN_HEADER
 * suppresses jsmn.h's static helper functions, leaving only the public
 * declarations jsmn_init(), jsmn_parse(), and the jsmntok_t /
 * jsmntype_t / jsmn_parser types for bindgen to see. */
#define JSMN_HEADER
#include "jsmn/jsmn.h"
