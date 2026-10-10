/* src/led_pattern.c */
#include "led_logic.h"

uint32_t led_pattern(uint32_t step)
{
    switch (step % 10U) {
    case 0:
    case 2:
        return 1U;
    default:
        return 0U;
    }
}
