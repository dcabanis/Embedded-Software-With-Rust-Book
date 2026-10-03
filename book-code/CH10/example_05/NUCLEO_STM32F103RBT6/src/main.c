#include <stdint.h>
#include <zephyr/kernel.h>
#include <zephyr/drivers/gpio.h>
#include "led_logic.h"

/* Obtain the board's LED pin and its active-level setting. */
static const struct gpio_dt_spec led =
    GPIO_DT_SPEC_GET(DT_ALIAS(led0), gpios);

int main(void)
{
    if (!gpio_is_ready_dt(&led)) {
        return 0;
    }

    int err = gpio_pin_configure_dt(&led, GPIO_OUTPUT_INACTIVE);
    if (err < 0) {
        return 0;
    }

    uint32_t step = 0;

    for (;;) {
        /* This call executes Rust within the current Zephyr thread. */
        uint32_t on = led_pattern(step);

        err = gpio_pin_set_dt(&led, on != 0);
        if (err < 0) {
            return 0;
        }

        step = (step + 1U) % 10U;

        /* Suspend this thread before requesting the next pattern step. */
        k_msleep(200);
    }
}
